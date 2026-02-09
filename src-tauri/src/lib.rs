#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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
