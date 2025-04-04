// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fov_camera::models::camera::Camera;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = CameraCalculatorUI::new()?;
    let mut camera: Camera = Camera::new();
    camera.set_width(1920);

    ui.on_calculate({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_focal_length(ui.get_fov() + 1_f32);
        }
    });

    ui.run()?;

    Ok(())
}
