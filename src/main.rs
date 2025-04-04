// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fov_camera::models::camera::Camera;
use std::error::Error;

slint::include_modules!();

struct App {
    camera: Camera,
    ui: CameraCalculatorUI,
}
impl App {
    pub fn new() -> Self {
        App {
            camera: Camera::new(),
            ui: CameraCalculatorUI::new().unwrap(),
        }
    }

    pub fn setup_callbacks(&mut self) {
        self.ui.on_calculate({
            let ui_handle = self.ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                ui.set_focal_length(9999_f32);
            }
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.setup_callbacks();

    app.camera.set_width(1920);

    app.ui.run()?;

    Ok(())
}
