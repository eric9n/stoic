use crate::menu;
use tauri::{Manager, Runtime, WindowBuilder};
use tauri_plugin_store::StoreBuilder;

pub const OPENAI_URL: &str = "https://chat.openai.com";

pub fn init_window<R: Runtime>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle();
    init_app(app_handle)?;
    Ok(())
}

pub fn init_app<R: Runtime>(
    app_handle: &tauri::AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new(
        app_handle,
        "Stoic".to_string(),
        tauri::WindowUrl::External(OPENAI_URL.parse().unwrap()),
    );

    builder = builder.inner_size(800.0, 600.0);

    let mut store = StoreBuilder::new("config").build(app_handle.clone());
    let _ = store.load();
    if let Some(proxy) = store.get("proxy") {
        let proxy_args = format!("--proxy={}", proxy.as_str().unwrap());
        builder = builder.additional_browser_args(&proxy_args);
    }

    #[cfg(target_os = "macos")]
    {
        builder = builder.tabbing_identifier("Stoic");
    }
    let window = builder.title("Stoic").build()?;

    window.on_menu_event(menu::handle_menu_event);
    Ok(())
}
