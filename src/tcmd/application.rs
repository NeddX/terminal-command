

pub struct Application {
    title: String,
    running: bool,
    window: crate::termgl::Window,
    active_scene: crate::tcmd::Scene2D
}

impl Application {
    pub fn new(title: String, width: u16, height: u16, delay: u32) -> Self {
        return Application {
            title: title,
            running: true,
            window: crate::termgl::Window::new(width, height, delay),
            active_scene: crate::tcmd::Scene2D::new(Vec::new())
        };
    }

    pub fn run(&mut self) {
        let mut delta: f32 = -1.0;
        while self.running {

        }
    }
}
