use fov_camera::backend::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app: App = App::new();
    app.init();
    app.run()?;

    Ok(())
}
