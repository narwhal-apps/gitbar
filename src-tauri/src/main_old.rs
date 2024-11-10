#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// mod auto_start;
mod commands;
mod server;
mod utils;

use tauri::{Manager, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use crate::menu::{create_system_tray_menu, handle_system_tray_event};

// use commands::{set_review_count, start_server, stop_server};
use commands::{start_server, stop_server};
use server::AuthServer;
use std::sync::Mutex;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton, NSWindowStyleMask, NSWindowTitleVisibility};

#[cfg(target_os = "macos")]
use objc::runtime::YES;

use tauri::{Runtime, Window};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

// mod system_tray;

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
}

// impl<R: Runtime> WindowExt for Window<R> {
//     #[cfg(target_os = "macos")]
//     fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
//         unsafe {
//             let id = self.ns_window().unwrap() as cocoa::base::id;
//             NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
//             let mut style_mask = id.styleMask();
//             style_mask.set(
//                 NSWindowStyleMask::NSFullSizeContentViewWindowMask,
//                 title_transparent,
//             );

//             id.setStyleMask_(style_mask);

//             if remove_tool_bar {
//                 let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
//                 let _: () = msg_send![close_button, setHidden: YES];
//                 let min_button =
//                     id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
//                 let _: () = msg_send![min_button, setHidden: YES];
//                 let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
//                 let _: () = msg_send![zoom_button, setHidden: YES];
//             }

//             id.setTitleVisibility_(if title_transparent {
//                 NSWindowTitleVisibility::NSWindowTitleHidden
//             } else {
//                 NSWindowTitleVisibility::NSWindowTitleVisible
//             });

//             id.setTitlebarAppearsTransparent_(if title_transparent {
//                 cocoa::base::YES
//             } else {
//                 cocoa::base::NO
//             });
//         }
//     }
// }

fn main() {
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let tray_menu = SystemTrayMenu::new().add_item(quit);
    // let system_tray = SystemTray::new().with_menu(tray_menu);
    // let edit_menu = Submenu::new(
    //     "Edit",
    //     Menu::new()
    //         .add_native_item(MenuItem::Undo)
    //         .add_native_item(MenuItem::Redo)
    //         .add_native_item(MenuItem::Separator)
    //         .add_native_item(MenuItem::Cut)
    //         .add_native_item(MenuItem::Copy)
    //         .add_native_item(MenuItem::Paste)
    //         .add_native_item(MenuItem::SelectAll),
    // );

    // let menu = Menu::new().add_submenu(edit_menu);

    let app = tauri::Builder::default()
        // .plugin(tauri_plugin_autostart::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_positioner::init())
        // .on_system_tray_event(|app, event| {
        //     tauri_plugin_positioner::on_tray_event(app, &event);
        // })
        // .menu(menu)
        // .system_tray(system_tray)
        // .on_system_tray_event(move |app, event| match event {
        //     SystemTrayEvent::LeftClick { .. } => {
        //         let w = app.get_window("main").unwrap();
        //         let visible = w.is_visible().unwrap();
        //         if visible {
        //             w.hide().unwrap();
        //         } else {
        //             let _ = w.move_window(Position::TrayCenter);
        //             w.show().unwrap();
        //             w.set_focus().unwrap();
        //         }
        //     }
        //     SystemTrayEvent::RightClick {
        //         position: _,
        //         size: _,
        //         ..
        //     } => {
        //         println!("system tray received a right click");
        //     }
        //     SystemTrayEvent::DoubleClick {
        //         position: _,
        //         size: _,
        //         ..
        //     } => {
        //         println!("system tray received a double click");
        //     }
        //     SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        //         "quit" => {
        //             std::process::exit(0);
        //         }
        //         _ => {}
        //     },
        //     _ => {}
        // })
        // .on_window_event(|event| match event.event() {
        //     tauri::WindowEvent::CloseRequested { api, .. } => {
        //         // don't kill the app when the user clicks close. this is important
        //         event.window().hide().unwrap();
        //         api.prevent_close();
        //     }
        //     tauri::WindowEvent::Focused(false) => {
        //         // hide the window automatically when the user
        //         // clicks out. this is for a matter of taste.
        //         event.window().hide().unwrap();
        //     }
        //     _ => {}
        // })
        .manage(Mutex::new(AuthServer::new()))
        .invoke_handler(tauri::generate_handler![
            // set_review_count,
            start_server,
            stop_server
        ])
        // .plugin(auto_start::init(None))
        .setup(|app| {
            #[cfg(target_os = "macos")]
            // don't show on the taskbar/springboard
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle().clone();
            let win_builder = WebviewWindowBuilder::new(&app_handle, "main", WebviewUrl::default());
            // set transparent title bar only when building for macOS
            // #[cfg(target_os = "macos")]
            // let w = win_builder.title_bar_style(TitleBarStyle::Transparent);

            // let window = win_builder.build().unwrap();

            window.open_devtools();

            // let _ = system_tray::init_system_tray(app);
            let _ = tauri::tray::TrayIconBuilder::new()
                .on_tray_icon_event(|tray_handle, event| {
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
                })
                .build(app)?;
        

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, event| match event {
        tauri::RunEvent::Exit => {
            println!("Exiting")
        }
        _ => {}
    });
}
