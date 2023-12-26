// use crate::setup;
use serde_json::json;
use tauri::Runtime;
use tauri_plugin_store::StoreBuilder;

#[tauri::command]
pub async fn set_proxy<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    proxy: &str,
) -> Result<(), String> {
    let proxy_args = format!("--proxy={}", proxy);
    println!("proxy_args {:?}", proxy_args);
    let mut store = StoreBuilder::new("config").build(app.clone());
    let _ = store.insert("proxy".into(), json!(proxy));
    let _ = store.save();

    let _ = window.close();
    app.restart();

    Ok(())
}
