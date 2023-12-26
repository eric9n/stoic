use tauri::generate_handler;
// use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_log::Target;
mod menu;
mod service;
mod setup;

pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("stoic".into()),
                })])
                .build(),
        )
        .setup(setup::init_window)
        .menu(menu::init_menu)
        .enable_macos_default_menu(false)
        .invoke_handler(generate_handler![service::set_proxy])
        .run(context)
        .expect("error while running tauri application");
}
