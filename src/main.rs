// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cell::RefCell, rc::Rc};
use fov_camera::models::camera::Camera;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: CameraCalculatorUI =  CameraCalculatorUI::new().unwrap();
    let camera = Rc::new(RefCell::new(Camera::default()));

    {
        let app = app.as_weak();
        let camera_clone = camera.clone();
        app.upgrade().unwrap().on_calculate({
            println!("on_calculate 1");
            move || {
                println!("on_calculate 2");
                let mut camera = camera_clone.borrow_mut();
                let focal_length: f32 = app.upgrade().unwrap().get_focal_length() as f32;
                println!("Focal length: {:?}", focal_length);
                camera.set_focal_length(focal_length);
                app.upgrade().unwrap().set_fov(camera.get_focal_length());
            }
        });
    }
    app.run()?;

    Ok(())
}
