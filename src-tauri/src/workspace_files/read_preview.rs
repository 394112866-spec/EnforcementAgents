//! Read a workspace file as text for the preview modal.
//!
//! Mirrors sidecar `/agent/file` semantics:
//!   * Resolve relative path inside workspace.
//!   * Reject if file doesn't exist.
//!   * Reject non-previewable types (binary / unknown — UI shows the modal
//!     only when content can be displayed as text).
//!   * Cap response at 512KB so a forgotten 50MB JSON doesn't pin the IPC
//!     channel.
//! Returns the same `{ content, name, size }` shape so DirectoryPanel's
//! `FilePreviewModal` consumer doesn't need a parallel branch.

use std::fs;

use serde::Serialize;

use super::path_safety::{resolve_inside_workspace, validate_workspace_root};

const MAX_PREVIEW_BYTES: u64 = 512 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResult {
    pub content: String,
    pub name: String,
    pub size: u64,
}

#[tauri::command]
pub async fn cmd_workspace_read_preview(
    workspace: String,
    path: String,
) -> Result<PreviewResult, String> {
    if path.trim().is_empty() {
        return Err("Missing path".to_string());
    }
    let workspace_root = validate_workspace_root(&workspace)?;
    let resolved = resolve_inside_workspace(&workspace_root, &path)?;
    let metadata = fs::symlink_metadata(&resolved)
        .map_err(|_| "File not found".to_string())?;
    if metadata.is_symlink() {
        // Resolve once for size + previewability.
        let stat = fs::metadata(&resolved).map_err(|_| "Symlink target missing".to_string())?;
        if !stat.is_file() {
            return Err("Not a regular file".to_string());
        }
    } else if !metadata.is_file() {
        return Err("Not a regular file".to_string());
    }

    let name = std::path::Path::new(&path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    if !is_previewable(&name) {
        return Err("File type not supported".to_string());
    }

    let size = fs::metadata(&resolved).map(|m| m.len()).unwrap_or(0);
    if size > MAX_PREVIEW_BYTES {
        return Err("File too large to preview".to_string());
    }

    let content = fs::read_to_string(&resolved)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    Ok(PreviewResult { content, name, size })
}

/// Mirrors `src/shared/fileTypes.ts::isPreviewable`: **binary-blocklist** strategy.
/// Anything NOT a known binary extension is considered previewable; extensionless
/// names (Makefile, LICENSE, .gitignore, Caddyfile, etc.) are always previewable.
///
/// Cross-review caught the original allowlist as a real UX regression — sidecar
/// `isPreviewableText` falls through to this same blocklist via `isPreviewable`,
/// so the renderer's "show preview" gate and the Rust port's "allow read" gate
/// must agree. Set must stay in sync with `src/shared/fileTypes.ts` BINARY_EXTENSIONS.
fn is_previewable(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let ext = match lower.rsplit_once('.') {
        Some((_, e)) if !e.is_empty() => e,
        // No dot at all → extensionless file (Makefile, LICENSE, …) — previewable.
        // A leading-dot dotfile (`.zshrc`, `.gitignore`) gets `_` empty / the rest
        // as ext via rsplit_once, but our heuristic above mirrors fileTypes.ts:93
        // (`if (!ext || ext === filename.toLowerCase()) return true`) — when the
        // extension equals the whole filename (no real ext), treat as previewable.
        _ => return true,
    };
    if ext == lower {
        return true;
    }
    !BINARY_EXTENSIONS.contains(&ext)
}

/// Keep in sync with `src/shared/fileTypes.ts::BINARY_EXTENSIONS`.
const BINARY_EXTENSIONS: &[&str] = &[
    // Images (superset of IMAGE_EXTENSIONS — includes raw/vector formats)
    "png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "ico", "tiff", "tif",
    "psd", "ai", "eps", "raw", "cr2", "nef", "heic", "heif", "avif", "jxl",
    // Video
    "mp4", "avi", "mov", "mkv", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "3gp",
    // Audio
    "mp3", "wav", "aac", "ogg", "flac", "wma", "m4a", "opus", "aiff",
    // Archives / Compressed
    "zip", "tar", "gz", "bz2", "xz", "rar", "7z", "zst", "lz4", "lzma", "cab", "dmg", "iso",
    // Executables / Libraries
    "exe", "dll", "so", "dylib", "bin", "app", "msi", "deb", "rpm", "apk", "ipa",
    // Compiled / Object
    "o", "obj", "class", "pyc", "pyo", "wasm", "elc",
    // Fonts
    "ttf", "otf", "woff", "woff2", "eot",
    // Documents (binary formats)
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf",
    // Databases
    "db", "sqlite", "sqlite3", "mdb",
    // Other binary
    "dat", "ds_store", "swp", "swo",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_files::test_support::make_test_workspace;

    #[tokio::test]
    async fn reads_text_file() {
        let ws = make_test_workspace("preview_text");
        fs::write(ws.join("hello.md"), "hi there").unwrap();
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "hello.md".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(res.content, "hi there");
        assert_eq!(res.name, "hello.md");
        assert_eq!(res.size, 8);
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_missing() {
        let ws = make_test_workspace("preview_missing");
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "nope.md".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_non_previewable() {
        let ws = make_test_workspace("preview_binary");
        fs::write(ws.join("blob.bin"), b"\x00\x01\x02").unwrap();
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "blob.bin".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_oversize() {
        let ws = make_test_workspace("preview_oversize");
        let big = "a".repeat((MAX_PREVIEW_BYTES + 1) as usize);
        fs::write(ws.join("big.md"), &big).unwrap();
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "big.md".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_traversal() {
        let ws = make_test_workspace("preview_traversal");
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "../etc/hosts".to_string(),
        )
        .await;
        assert!(res.is_err());
        let _ = fs::remove_dir_all(&ws);
    }

    // Cross-review regression guards: the renderer side `isPreviewable` is a
    // BINARY blocklist (not text allowlist). The Rust gate must agree, otherwise
    // the user clicks a file the UI says is previewable and Rust returns "not
    // supported".
    #[tokio::test]
    async fn allows_extensionless_files() {
        let ws = make_test_workspace("preview_extless");
        for name in ["Makefile", "LICENSE", "Dockerfile", "Caddyfile"] {
            fs::write(ws.join(name), "content").unwrap();
            let res = cmd_workspace_read_preview(
                ws.to_string_lossy().to_string(),
                name.to_string(),
            )
            .await;
            assert!(res.is_ok(), "{} should be previewable", name);
        }
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn allows_dotfiles() {
        let ws = make_test_workspace("preview_dotfiles");
        for name in [".zshrc", ".gitconfig", ".npmrc", ".tool-versions"] {
            fs::write(ws.join(name), "content").unwrap();
            let res = cmd_workspace_read_preview(
                ws.to_string_lossy().to_string(),
                name.to_string(),
            )
            .await;
            assert!(res.is_ok(), "{} should be previewable", name);
        }
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn allows_unknown_text_extension() {
        let ws = make_test_workspace("preview_unknown_ext");
        fs::write(ws.join("data.weirdext"), "still text").unwrap();
        let res = cmd_workspace_read_preview(
            ws.to_string_lossy().to_string(),
            "data.weirdext".to_string(),
        )
        .await;
        assert!(res.is_ok(), "unknown extension should default to previewable");
        let _ = fs::remove_dir_all(&ws);
    }

    #[tokio::test]
    async fn rejects_known_binary_extensions() {
        let ws = make_test_workspace("preview_binary_ext");
        for name in ["app.exe", "vid.mp4", "music.mp3", "lib.so", "doc.docx"] {
            fs::write(ws.join(name), "fake").unwrap();
            let res = cmd_workspace_read_preview(
                ws.to_string_lossy().to_string(),
                name.to_string(),
            )
            .await;
            assert!(res.is_err(), "{} should be rejected as binary", name);
        }
        let _ = fs::remove_dir_all(&ws);
    }
}
