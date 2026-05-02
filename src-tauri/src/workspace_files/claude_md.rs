//! Read / write `<workspace>/CLAUDE.md` (Phase D.5 / E1 migration).
//!
//! Mirrors sidecar `/api/claude-md` GET/POST. Used by SystemPromptsPanel in
//! Settings to view / edit a workspace's CLAUDE.md (project-level system
//! prompt addendum). Path is fixed per workspace — `<workspace>/CLAUDE.md` —
//! so there's no path-traversal surface on the input side.
//!
//! - Read returns `{ exists, path, content }` to mirror the sidecar shape;
//!   "not exists" is NOT an error (Settings UI shows an empty editor).
//! - Write creates the file if missing (CLAUDE.md is often "created on
//!   first edit" — no separate "create" affordance in UI).
//! - Atomic write via tmp + rename, same pattern as `save_file.rs`.

use std::fs;
use std::io::Write;

use serde::Serialize;

use super::path_safety::validate_workspace_root;

const CLAUDE_MD_FILENAME: &str = "CLAUDE.md";

/// Hard cap on CLAUDE.md size — sidecar had no explicit cap (`writeFileSync`
/// would just OOM on huge inputs); we add one to match the rest of the
/// workspace_files surface. 1MB is well above any practical CLAUDE.md.
const MAX_CONTENT_BYTES: usize = 1024 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadClaudeMdResult {
    pub success: bool,
    pub exists: bool,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteClaudeMdResult {
    pub success: bool,
    pub path: String,
}

#[tauri::command]
pub async fn cmd_workspace_read_claude_md(
    workspace: String,
) -> Result<ReadClaudeMdResult, String> {
    let workspace_root = validate_workspace_root(&workspace)?;
    let claude_md = workspace_root.join(CLAUDE_MD_FILENAME);
    let path_str = claude_md.to_string_lossy().to_string();

    // Use symlink_metadata first to avoid following a malicious link out of
    // the workspace. CLAUDE.md is fixed-name and at workspace root, but a
    // user could (legitimately) symlink it; if that link goes outside, fail
    // the same way the read-side gate does — `not found` is uniform UX.
    let meta = match fs::symlink_metadata(&claude_md) {
        Ok(m) => m,
        Err(_) => {
            return Ok(ReadClaudeMdResult {
                success: true,
                exists: false,
                path: path_str,
                content: String::new(),
            });
        }
    };
    if meta.is_symlink() {
        // If the symlink target falls outside the workspace, refuse rather
        // than leak content — same posture as `read_preview.rs`.
        let canonical = fs::canonicalize(&claude_md)
            .map_err(|_| "Symlink target unavailable".to_string())?;
        let canonical_root = fs::canonicalize(&workspace_root)
            .map_err(|_| "Workspace root canonicalize failed".to_string())?;
        if !canonical.starts_with(&canonical_root) {
            return Err("CLAUDE.md symlink escapes workspace".to_string());
        }
    }
    if !claude_md.is_file() {
        // Resolved (via symlink follow) to something that isn't a file.
        return Err("CLAUDE.md is not a regular file".to_string());
    }

    let content = fs::read_to_string(&claude_md)
        .map_err(|e| format!("Failed to read CLAUDE.md: {}", e))?;
    Ok(ReadClaudeMdResult {
        success: true,
        exists: true,
        path: path_str,
        content,
    })
}

#[tauri::command]
pub async fn cmd_workspace_write_claude_md(
    workspace: String,
    content: String,
) -> Result<WriteClaudeMdResult, String> {
    if content.len() > MAX_CONTENT_BYTES {
        return Err("Content too large".to_string());
    }
    let workspace_root = validate_workspace_root(&workspace)?;
    let claude_md = workspace_root.join(CLAUDE_MD_FILENAME);

    // If CLAUDE.md exists as a symlink escaping the workspace, refuse to
    // write — symmetric with the read-side check. This blocks an
    // `evil_link → /etc/sudoers` pre-planted in a malicious repo.
    if let Ok(meta) = fs::symlink_metadata(&claude_md) {
        if meta.is_symlink() {
            let canonical = fs::canonicalize(&claude_md)
                .map_err(|_| "Symlink target unavailable".to_string())?;
            let canonical_root = fs::canonicalize(&workspace_root)
                .map_err(|_| "Workspace root canonicalize failed".to_string())?;
            if !canonical.starts_with(&canonical_root) {
                return Err("CLAUDE.md symlink escapes workspace".to_string());
            }
        }
    }

    // Atomic write via tmp + rename. Same pattern as `save_file.rs`.
    let tmp_name = format!(
        ".{}.myagents-claude-md-{}-{}.tmp",
        CLAUDE_MD_FILENAME,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0),
    );
    let tmp_path = workspace_root.join(&tmp_name);
    {
        let mut tmp_file = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create tmp file: {}", e))?;
        tmp_file
            .write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write tmp file: {}", e))?;
    }
    if let Err(e) = fs::rename(&tmp_path, &claude_md) {
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("Failed to commit CLAUDE.md write: {}", e));
    }

    Ok(WriteClaudeMdResult {
        success: true,
        path: claude_md.to_string_lossy().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_files::test_support::make_test_workspace;

    #[tokio::test]
    async fn read_returns_exists_false_when_missing() {
        let ws = make_test_workspace("claude_md_read_missing");
        let res = cmd_workspace_read_claude_md(ws.to_string_lossy().to_string())
            .await
            .unwrap();
        assert!(res.success);
        assert!(!res.exists);
        assert_eq!(res.content, "");
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn read_returns_content() {
        let ws = make_test_workspace("claude_md_read_content");
        fs::write(ws.join("CLAUDE.md"), "# project rules\nbe nice").unwrap();
        let res = cmd_workspace_read_claude_md(ws.to_string_lossy().to_string())
            .await
            .unwrap();
        assert!(res.exists);
        assert!(res.content.contains("be nice"));
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn write_creates_when_missing() {
        let ws = make_test_workspace("claude_md_write_create");
        let res = cmd_workspace_write_claude_md(
            ws.to_string_lossy().to_string(),
            "# new\n".to_string(),
        )
        .await
        .unwrap();
        assert!(res.success);
        assert!(ws.join("CLAUDE.md").is_file());
        assert_eq!(
            fs::read_to_string(ws.join("CLAUDE.md")).unwrap(),
            "# new\n"
        );
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn write_overwrites_existing() {
        let ws = make_test_workspace("claude_md_write_overwrite");
        fs::write(ws.join("CLAUDE.md"), "old").unwrap();
        cmd_workspace_write_claude_md(
            ws.to_string_lossy().to_string(),
            "new".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(fs::read_to_string(ws.join("CLAUDE.md")).unwrap(), "new");
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn write_rejects_oversize() {
        let ws = make_test_workspace("claude_md_write_oversize");
        let big = "a".repeat(MAX_CONTENT_BYTES + 1);
        let res =
            cmd_workspace_write_claude_md(ws.to_string_lossy().to_string(), big).await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    // Symlink-escape parity with read_preview / save_file.
    #[cfg(unix)]
    #[tokio::test]
    async fn rejects_symlink_escape_on_read() {
        use std::os::unix::fs::symlink;
        let ws = make_test_workspace("claude_md_symlink_read");
        let outside = std::env::temp_dir().join(format!(
            "claude_md_outside_{}",
            std::process::id()
        ));
        fs::create_dir_all(&outside).unwrap();
        let secret = outside.join("secret.md");
        fs::write(&secret, "TOP-SECRET").unwrap();
        symlink(&secret, ws.join("CLAUDE.md")).unwrap();

        let res = cmd_workspace_read_claude_md(ws.to_string_lossy().to_string()).await;
        assert!(res.is_err());
        assert!(!format!("{:?}", res).contains("TOP-SECRET"));
        let _ = fs::remove_dir_all(&ws);
        let _ = fs::remove_dir_all(&outside);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn rejects_symlink_escape_on_write() {
        use std::os::unix::fs::symlink;
        let ws = make_test_workspace("claude_md_symlink_write");
        let outside = std::env::temp_dir().join(format!(
            "claude_md_outside_w_{}",
            std::process::id()
        ));
        fs::create_dir_all(&outside).unwrap();
        let secret = outside.join("secret.md");
        fs::write(&secret, "OUTSIDE").unwrap();
        symlink(&secret, ws.join("CLAUDE.md")).unwrap();

        let res = cmd_workspace_write_claude_md(
            ws.to_string_lossy().to_string(),
            "OVERWRITE".to_string(),
        )
        .await;
        assert!(res.is_err());
        // The outside file is untouched.
        assert_eq!(fs::read_to_string(&secret).unwrap(), "OUTSIDE");
        let _ = fs::remove_dir_all(&ws);
        let _ = fs::remove_dir_all(&outside);
    }
}
