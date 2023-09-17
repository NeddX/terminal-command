use super::*;

pub struct Application {
    title: String,
    running: bool,
    window: termgl::Window,
    active_scene: Scene2D,
    tp1: Instant,
    tp2: Instant
}

impl Application {
    pub fn new(title: String, width: u16, height: u16, delay: u32) -> Self {
        return Application {
            title: title,
            running: true,
            window: termgl::Window::new(width, height, delay),
            active_scene: Scene2D::new(Vec::new()),
            tp1:
        };
    }

    pub fn run(&mut self) {
        let mut delta: f32 = -1.0;
        while running {
            self.tp2 = Instant::now();


            self.tp2 = Instant::now();
        }
    }
}
