// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::models::camera::Camera;
use std::{cell::RefCell, rc::Rc};
slint::include_modules!();

pub struct App {
    camera: Rc<RefCell<Camera>>,
    ui: CameraCalculatorUI,
}

impl App {
    pub fn new() -> Self {
        let camera: Rc<RefCell<Camera>> = Rc::new(RefCell::new(Camera::default()));
        let ui: CameraCalculatorUI = CameraCalculatorUI::new().unwrap();
        Self { camera, ui }
    }

    pub fn init(&mut self) {
        let camera: Rc<RefCell<Camera>> = self.camera.clone();
        let ui: slint::Weak<CameraCalculatorUI> = self.ui.as_weak();
        ui.upgrade().unwrap().on_calculate({
            println!("on_calculate 1");
            move || {
                println!("on_calculate 2");
                let mut camera: std::cell::RefMut<'_, Camera> = camera.borrow_mut();
                let focal_length: f32 = ui.upgrade().unwrap().get_focal_length() as f32;
                println!("Focal length: {:?}", focal_length);
                let old_fov: f32 = camera.get_focal_length();
                camera.set_focal_length(focal_length + old_fov);
                ui.upgrade().unwrap().set_fov(camera.get_focal_length());
            }
        });
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.ui.run()?;
        Ok(())
    }
} // impl App

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
