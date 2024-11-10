#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, TitleBarStyle, WebviewUrl, WebviewWindowBuilder, Runtime, WebviewWindow};

mod commands;
mod server;
mod system_tray;
mod utils;

use commands::{start_server, stop_server};
use server::AuthServer;

use std::sync::Mutex;
use tauri_plugin_autostart::MacosLauncher;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton, NSWindowStyleMask, NSWindowTitleVisibility};

#[cfg(target_os = "macos")]
use objc::runtime::YES;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
}

impl<R: Runtime> WindowExt for WebviewWindow<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                title_transparent,
            );

            id.setStyleMask_(style_mask);

            if remove_tool_bar {
                let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
                let _: () = msg_send![close_button, setHidden: YES];
                let min_button =
                    id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
                let _: () = msg_send![min_button, setHidden: YES];
                let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
                let _: () = msg_send![zoom_button, setHidden: YES];
            }

            id.setTitleVisibility_(if title_transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });

            id.setTitlebarAppearsTransparent_(if title_transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

pub fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .manage(Mutex::new(AuthServer::new()))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let app_handle = app.app_handle().clone();
            let win_builder = WebviewWindowBuilder::new(&app_handle, "main", WebviewUrl::default())
                .inner_size(500.0, 400.0)
                .skip_taskbar(true)
                .fullscreen(false)
                .resizable(false)
                .title("Gitbar")
                .title_bar_style(TitleBarStyle::Overlay)
                .decorations(true)
                .visible(false);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window: tauri::WebviewWindow = win_builder.build().unwrap();

            #[cfg(target_os = "macos")]
            window.set_transparent_titlebar(true, true);
            window.set_always_on_top(true).unwrap();

            window.open_devtools();

            system_tray::setup_system_tray(app);

            // let autostart_manager = app.autolaunch();
            // let enabled = autostart_manager.is_enabled().unwrap();

            // tray.on_tray_icon_event(|tray, event| match event {
            //     TrayIconEvent::Click {
            //         button: MouseButton::Left,
            //         button_state: MouseButtonState::Up,
            //         ..
            //     } => {
            //         println!("left click pressed and released");
            //         // in this example, let's show and focus the main window when the tray is clicked
            //         let app = tray.app_handle();
            //         if let Some(w) = app.get_webview_window("main") {
            //             let visible = w.is_visible().unwrap();
            //             if visible {
            //                 w.hide().unwrap();
            //             } else {
            //                 let _ = w.as_ref().window().move_window(Position::TrayCenter);
            //                 w.show().unwrap();
            //                 w.set_focus().unwrap();
            //             }
            //         }
            //     }
            //     _ => {
            //         println!("unhandled event {event:?}");
            //     }
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // set_review_count,
            start_server,
            stop_server
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
