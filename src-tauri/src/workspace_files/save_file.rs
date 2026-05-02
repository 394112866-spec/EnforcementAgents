//! Save edited workspace file content (Phase D.5 / E1 migration).
//!
//! Mirrors sidecar `/agent/save-file` semantics:
//! - File MUST already exist (this command does NOT create — that's
//!   `cmd_workspace_new_file`'s job; `FilePreviewModal` only opens
//!   existing files for edit, so this distinction matches the UX path).
//! - 512KB content cap (matches `read_preview` cap so a roundtrip
//!   read → edit → save is symmetric).
//! - Atomic write via tmp + rename in the same parent directory.
//! - Read-side path resolver (`resolve_existing_inside_workspace`) so
//!   a workspace-internal symlink to an outside file (`evil_link →
//!   /etc/passwd`) cannot be overwritten through this endpoint — the
//!   CRIT check from Phase D.5 cross-review applies symmetrically to
//!   write-while-existing.

use std::fs;
use std::io::Write;

use serde::Serialize;

use super::path_safety::{resolve_existing_inside_workspace, validate_workspace_root};

const MAX_CONTENT_BYTES: usize = 512 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveFileResult {
    pub success: bool,
}

#[tauri::command]
pub async fn cmd_workspace_save_file(
    workspace: String,
    path: String,
    content: String,
) -> Result<SaveFileResult, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("path is required".to_string());
    }
    if content.len() > MAX_CONTENT_BYTES {
        return Err("Content too large".to_string());
    }
    let workspace_root = validate_workspace_root(&workspace)?;
    let resolved = resolve_existing_inside_workspace(&workspace_root, trimmed)?;

    // `resolved` is canonicalized — guaranteed to be a real file inside the
    // canonical workspace. Confirm it's a regular file (not a dir we
    // accidentally accepted via canonicalize).
    let metadata = fs::metadata(&resolved).map_err(|_| "File not found".to_string())?;
    if !metadata.is_file() {
        return Err("Not a regular file".to_string());
    }

    // Atomic write: tmp file in the same parent dir, then rename. Same-dir
    // tmp guarantees the rename is on the same filesystem and so atomic.
    // The tmp name embeds pid+nanos so concurrent saves of the same file
    // don't collide on the tmp slot.
    let parent = resolved
        .parent()
        .ok_or_else(|| "Cannot determine parent directory".to_string())?;
    let file_name = resolved
        .file_name()
        .ok_or_else(|| "Cannot determine filename".to_string())?
        .to_string_lossy()
        .to_string();
    let tmp_name = format!(
        ".{}.myagents-save-{}-{}.tmp",
        file_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0),
    );
    let tmp_path = parent.join(&tmp_name);

    {
        let mut tmp_file = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create tmp file: {}", e))?;
        tmp_file
            .write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write tmp file: {}", e))?;
        // Drop the file handle before rename — Windows requires this.
    }

    if let Err(e) = fs::rename(&tmp_path, &resolved) {
        // Best-effort cleanup so a failed rename doesn't leak a tmp file.
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("Failed to commit save: {}", e));
    }

    Ok(SaveFileResult { success: true })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_files::test_support::make_test_workspace;

    #[tokio::test]
    async fn writes_existing_file() {
        let ws = make_test_workspace("save_writes");
        fs::write(ws.join("a.md"), "old").unwrap();
        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "a.md".to_string(),
            "new content".to_string(),
        )
        .await
        .unwrap();
        assert!(res.success);
        assert_eq!(fs::read_to_string(ws.join("a.md")).unwrap(), "new content");
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_missing_file() {
        let ws = make_test_workspace("save_missing");
        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "nope.md".to_string(),
            "x".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_oversize() {
        let ws = make_test_workspace("save_oversize");
        fs::write(ws.join("f.md"), "old").unwrap();
        let big = "a".repeat(MAX_CONTENT_BYTES + 1);
        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "f.md".to_string(),
            big,
        )
        .await;
        assert!(res.is_err());
        // File unchanged.
        assert_eq!(fs::read_to_string(ws.join("f.md")).unwrap(), "old");
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_traversal() {
        let ws = make_test_workspace("save_traversal");
        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "../etc/hosts".to_string(),
            "x".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_directory_target() {
        let ws = make_test_workspace("save_dir");
        fs::create_dir_all(ws.join("sub")).unwrap();
        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "sub".to_string(),
            "x".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    // Cross-review symmetry guard: the Phase D.5 read-side symlink-escape gate
    // covers cmd_workspace_save_file too (we use the same resolver).
    // A workspace-internal symlink to `/etc/...` MUST NOT be writable via save.
    #[cfg(unix)]
    #[tokio::test]
    async fn rejects_symlink_escape() {
        use std::os::unix::fs::symlink;
        let ws = make_test_workspace("save_symlink_escape");
        let outside = std::env::temp_dir().join(format!(
            "save_outside_{}",
            std::process::id()
        ));
        fs::create_dir_all(&outside).unwrap();
        let real = outside.join("real.md");
        fs::write(&real, "OUTSIDE").unwrap();
        symlink(&real, ws.join("evil.md")).unwrap();

        let res = cmd_workspace_save_file(
            ws.to_string_lossy().to_string(),
            "evil.md".to_string(),
            "OVERWRITE".to_string(),
        )
        .await;
        assert!(res.is_err());
        // The outside file is untouched.
        assert_eq!(fs::read_to_string(&real).unwrap(), "OUTSIDE");
        let _ = fs::remove_dir_all(&ws);
        let _ = fs::remove_dir_all(&outside);
    }
}
