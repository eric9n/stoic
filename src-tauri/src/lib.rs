#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::WindowBuilder;

pub fn run() {
    // let builder = tauri::Builder::new();
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            #[allow(unused_mut)]
            let mut builder = WindowBuilder::new(
                app,
                "Stoic".to_string(),
                tauri::WindowUrl::External("https://www.youtube.com".parse().unwrap()),
            );
            builder = builder.additional_browser_args("--proxy=http://127.0.0.1:1027");
            builder = builder.inner_size(800.0, 600.0);

            #[cfg(target_os = "macos")]
            {
                builder = builder.tabbing_identifier("Stoic");
            }
            let _window = builder.title("Stoic").build()?;
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
