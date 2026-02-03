use std::process::Command;
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Copies text to the system clipboard
fn copy_to_clipboard(app: &AppHandle, text: &str) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}

/// Simulates Cmd+V paste keystroke via AppleScript (macOS)
#[cfg(target_os = "macos")]
fn simulate_paste() -> Result<(), String> {
    let output = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to keystroke \"v\" using command down",
        ])
        .output()
        .map_err(|e| format!("Failed to execute AppleScript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AppleScript failed: {}", stderr));
    }

    Ok(())
}

/// Placeholder for Windows implementation
#[cfg(target_os = "windows")]
fn simulate_paste() -> Result<(), String> {
    // TODO: Windows implementation with SendInput API
    // Will need to use windows-rs crate to call SendInput for Ctrl+V
    Err("Auto-paste not yet implemented on Windows".to_string())
}

/// Fallback for other platforms
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn simulate_paste() -> Result<(), String> {
    Err("Auto-paste not supported on this platform".to_string())
}

/// Pastes text into the active application by copying to clipboard and simulating paste keystroke
///
/// # Arguments
/// * `app` - Tauri app handle for clipboard access
/// * `text` - Text to paste into the active application
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(String)` with error description on failure
#[tauri::command]
pub async fn auto_paste(app: AppHandle, text: String) -> Result<(), String> {
    // 1. Copy text to clipboard
    copy_to_clipboard(&app, &text)?;

    // 2. Small delay to ensure clipboard is updated and focus is stable
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // 3. Simulate paste keystroke
    simulate_paste()?;

    Ok(())
}
