use tauri::{
    image::Image,
    menu::MenuBuilder,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let menu = MenuBuilder::new(app).text("quit", "Quit").build()?;

    app.handle().plugin(tauri_plugin_positioner::init());
    let _ = TrayIconBuilder::with_id("gitbar-tray")
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |_, event| match event.id().as_ref() {
            "quit" => {
                std::process::exit(0);
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
            let window = tray.app_handle().get_webview_window("main").unwrap();
            let _ = window.as_ref().window().move_window(Position::TrayCenter);
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    println!("left click pressed and released");

                    let visible = window.is_visible().unwrap();

                    if visible {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                _ => {
                    // println!("unhandled event {event:?}");
                }
            }
        })
        .icon(get_icon())
        .build(app)?;

    Ok(())
}

fn get_icon() -> Image<'static> {
    let theme = match dark_light::detect() {
        dark_light::Mode::Dark => "dark",
        dark_light::Mode::Light => "light",
        dark_light::Mode::Default => "light",
    };

    println!("{:?} theme: ", theme);

    return Image::from_path(format!("icons/tray-{theme}.png")).unwrap();
}
