use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
fn open_external(app: AppHandle, url: String) -> Result<(), String> {
    let url = url.trim();
    if !(url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("mailto:")
        || url.starts_with("tel:"))
    {
        return Err("unsupported url scheme".into());
    }

    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .plugin(tauri_plugin_google_auth::init());

    #[cfg(desktop)]
    let builder = desktop::configure(builder);

    #[cfg(mobile)]
    let builder = mobile::configure(builder);

    builder
        .setup(|app| {
            #[cfg(desktop)]
            desktop::setup(app)?;

            #[cfg(mobile)]
            mobile::setup(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
