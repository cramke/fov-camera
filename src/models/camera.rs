pub struct Camera {
    width: u32,
    height: u32,
    width_pixels: u32,
    height_pixels: u32,
    pixel_aspect_ratio: f32,
    format: String,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            width: 0,
            height: 0,
            width_pixels: 0,
            height_pixels: 0,
            pixel_aspect_ratio: 1.0,
            format: String::new(),
        }
    }

    // Setters
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    pub fn set_width_pixels(&mut self, width_pixels: u32) {
        self.width_pixels = width_pixels;
    }
    pub fn set_height_pixels(&mut self, height_pixels: u32) {
        self.height_pixels = height_pixels;
    }
    pub fn set_pixel_aspect_ratio(&mut self, pixel_aspect_ratio: f32) {
        self.pixel_aspect_ratio = pixel_aspect_ratio;
    }
    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }
    // Getters
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn get_width_pixels(&self) -> u32 {
        self.width_pixels
    }
    pub fn get_height_pixels(&self) -> u32 {
        self.height_pixels
    }
    pub fn get_pixel_aspect_ratio(&self) -> f32 {
        self.pixel_aspect_ratio
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
