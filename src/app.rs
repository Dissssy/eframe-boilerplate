pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {}
}
