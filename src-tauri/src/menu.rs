use tauri::menu::MenuEvent;
use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::Manager;
use tauri::{AppHandle, Runtime, Window, WindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn handle_menu_event<R: Runtime>(window: &Window<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "proxy" => {
            let app = window.app_handle();
            if let Some(window) = app.get_window("proxy_window") {
                // 如果窗口已存在，显示并聚焦该窗口
                window.show().unwrap();
                window.set_focus().unwrap();
            } else {
                // 如果窗口不存在，创建一个新的窗口
                let window = WindowBuilder::new(
                    app,
                    "proxy_window", // 使用一个唯一的标识符
                    tauri::WindowUrl::App("index.html".into()),
                )
                .title("Stoic 代理设置")
                .inner_size(600.0, 400.0)
                .build()
                .expect("error while creating a new window");
                window.show().unwrap();
                window.set_focus().unwrap();
                let _ = window.move_window(Position::LeftCenter);
            }
        }
        _ => {}
    }
}

pub fn init_menu<R: Runtime>(handle: &AppHandle<R>) -> Result<Menu<R>, tauri::Error> {
    let proxy_menu_item = MenuItem::with_id(handle, "proxy", "代理设置", true, None);

    let menu = Menu::with_items(
        handle,
        &[&Submenu::with_items(handle, "File", true, &[&proxy_menu_item]).unwrap()],
    );

    menu
}
