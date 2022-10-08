mod app;
use app::Application;

#[cfg(target_os = "windows")]
fn fix_stdout() {
    use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

fn main() {
    #[cfg(target_family = "windows")]
    fix_stdout();
    let icon = image::load_from_memory(include_bytes!("../icon.png"))
        .unwrap()
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "The(R)",
        options,
        Box::new(|_cc| Box::new(Application::new())),
    );
}
