use base64::Engine;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "android")]
mod android;

// ── Download command ──

#[tauri::command]
async fn save_download(
    app: AppHandle,
    filename: String,
    data_base64: String,
) -> Result<String, String> {
    let data = base64::engine::general_purpose::STANDARD
        .decode(&data_base64)
        .map_err(|e| format!("Base64 decode error: {e}"))?;

    // Drop the base64 string immediately to free memory
    drop(data_base64);

    let doc_dir = app.path().document_dir().map_err(|e| e.to_string())?;
    let download_dir = doc_dir.join("Downloads");
    std::fs::create_dir_all(&download_dir)
        .map_err(|e| format!("Cannot create directory: {e}"))?;

    // Handle filename collisions
    let sanitized = filename.replace(['/', '\\', '\0'], "_");
    let target = download_dir.join(&sanitized);
    let final_path = if target.exists() {
        let stem = target
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let ext = target
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let mut n = 1u32;
        loop {
            let candidate = download_dir.join(format!("{stem} ({n}){ext}"));
            if !candidate.exists() {
                break candidate;
            }
            n += 1;
        }
    } else {
        target
    };

    std::fs::write(&final_path, &data).map_err(|e| format!("Cannot write file: {e}"))?;

    Ok(final_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string())
}

// ── Setup ──

pub fn configure(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![crate::open_external, save_download])
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let mut init_script = String::new();
    init_script.push_str(include_str!("../google-auth-init.js"));
    init_script.push('\n');
    init_script.push_str(include_str!("../ios-download-init.js"));
    init_script.push('\n');
    init_script.push_str(include_str!("../external-links.js"));
    init_script.push('\n');
    init_script.push_str(include_str!("../mobile-gestures.js"));

    let _window = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External("https://monochrome.samidy.com".parse().unwrap()),
    )
    .initialization_script(init_script)
    .build()?;

    println!("[DEBUG] mobile webview built");

    #[cfg(target_os = "ios")]
    ios::setup(app)?;

    #[cfg(target_os = "android")]
    android::setup(app)?;

    Ok(())
}
