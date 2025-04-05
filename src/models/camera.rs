
pub struct Camera {
    width_mm: f32,
    height_mm: f32,
    width_pixels: u32,
    height_pixels: u32,
    aspect_ratio: f32,
    focal_length: f32,
    fov: f32,
    working_distance: f32,
    ppm: f32,
    gsd: f32,
    format: String,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            width_mm: 0.0,
            height_mm: 0.0,
            width_pixels: 0,
            height_pixels: 0,
            aspect_ratio: 1.0,
            focal_length: 0.0,
            fov: 0.0,
            working_distance: 0.0,
            ppm: 0.0,
            gsd: 0.0,
            format: String::new(),
        }
    }

    // Setters
    pub fn set_width(&mut self, width: f32) {
        self.width_mm = width;
    }
    pub fn set_height(&mut self, height: f32) {
        self.height_mm = height;
    }
    pub fn set_width_pixels(&mut self, width_pixels: u32) {
        self.width_pixels = width_pixels;
    }
    pub fn set_height_pixels(&mut self, height_pixels: u32) {
        self.height_pixels = height_pixels;
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
    pub fn set_focal_length(&mut self, focal_length: f32) {
        self.focal_length = focal_length;
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
    pub fn set_working_distance(&mut self, working_distance: f32) {
        self.working_distance = working_distance;
    }
    pub fn set_ppm(&mut self, ppm: f32) {
        self.ppm = ppm;
    }
    pub fn set_gsd(&mut self, gsd: f32) {
        self.gsd = gsd;
    }
    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }

    // Getters
    pub fn get_width_mm(&self) -> f32 {
        self.width_mm
    }
    pub fn get_height_mm(&self) -> f32 {
        self.height_mm
    }
    pub fn get_width_pixels(&self) -> u32 {
        self.width_pixels
    }
    pub fn get_height_pixels(&self) -> u32 {
        self.height_pixels
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn get_focal_length(&self) -> f32 {
        self.focal_length
    }
    pub fn get_fov(&self) -> f32 {
        self.fov
    }
    pub fn get_working_distance(&self) -> f32 {
        self.working_distance
    }
    pub fn get_ppm(&self) -> f32 {
        self.ppm
    }
    pub fn get_gsd(&self) -> f32 {
        self.gsd
    }
    pub fn get_format(&self) -> &str {
        &self.format
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}
