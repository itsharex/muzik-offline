use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

pub fn setup_macos(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let win_builder =
        WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .fullscreen(false)
        .resizable(true)
        .title("muzik-offline")
        .inner_size(980, 623)
        .min_inner_size(980, 623)
        .transparent(true)
        .center();

    // set transparent title bar only when building for macOS
    #[cfg(target_os = "macos")]
    let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

    let window = win_builder.build()?;

    // set background color only when building for macOS
    #[cfg(target_os = "macos")]
    {
    use cocoa::appkit::{NSColor, NSWindow};
    use cocoa::base::{id, nil};

    let ns_window = window.ns_window()? as id;
        unsafe {
            let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                nil,
                50.0 / 255.0,
                158.0 / 255.0,
                163.5 / 255.0,
                1.0,
            );
            ns_window.setBackgroundColor_(bg_color);
        }
    }

    Ok(())

}