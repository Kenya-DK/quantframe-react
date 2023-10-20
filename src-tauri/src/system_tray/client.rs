use tauri::{CustomMenuItem, SystemTrayMenu};

pub fn get_tray_menu() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_item(hide);
    tray_menu
}

pub fn get_tray_event(event: String) -> String {
    match event.as_str() {
        "quit" => {
            std::process::exit(0);
        }
        "hide" => {
            print!("hide");
        }
        _ => {
            print!("unknown");
        }
    }
    event
}
