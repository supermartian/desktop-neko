use std::sync::mpsc::Sender;
use tray_icon::{TrayIconBuilder, TrayIcon, menu::{Menu, MenuItem, Submenu, PredefinedMenuItem}};
use tray_icon::menu::MenuEvent;
use tray_icon::TrayIconEvent;
use crate::app::AppMessage;
use std::collections::HashMap;

pub fn setup_tray(tx: Sender<AppMessage>, package_names: &[String]) -> TrayIcon {
    let tray_menu = Menu::new();
    let spawn_menu = Submenu::new("Spawn Pet", true);
    
    let mut spawn_id_map = HashMap::new();
    
    for (idx, name) in package_names.iter().enumerate() {
        let spawn_item = MenuItem::new(name, true, None);
        let _ = spawn_menu.append(&spawn_item);
        spawn_id_map.insert(spawn_item.id().clone(), idx);
    }
    
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append(&spawn_menu);
    let _ = tray_menu.append(&PredefinedMenuItem::separator());
    let _ = tray_menu.append(&quit_item);

    let quit_id = quit_item.id().clone();

    std::thread::spawn(move || {
        let menu_channel = tray_icon::menu::MenuEvent::receiver();
        
        loop {
            if let Ok(event) = menu_channel.recv() {
                if let Some(&pkg_idx) = spawn_id_map.get(&event.id) {
                    let _ = tx.send(AppMessage::SpawnPet(pkg_idx));
                } else if event.id == quit_id {
                    let _ = tx.send(AppMessage::Quit);
                }
            }
        }
    });

    // Create a 16x16 solid red icon so it's easily visible in the tray
    let mut rgba = Vec::new();
    for _ in 0..(16*16) {
        rgba.extend_from_slice(&[255, 0, 0, 255]);
    }
    let icon = tray_icon::Icon::from_rgba(rgba, 16, 16).unwrap();

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Desktop Neko")
        .with_icon(icon)
        .build()
        .unwrap()
}
