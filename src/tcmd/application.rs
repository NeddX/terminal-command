use super::*;

pub struct Application {
    title: String,
    running: bool,
    window: Window,
    active_scene: Scene2D
}

impl Application {
    pub fn new(title: String, width: u16, height: u16, delay: u32) -> Self {
        return Application {
            title: title,
            running: true,
            window: Window::new(width, height, delay),
            active_scene: Scene2D::new()
        };
    }

    pub fn run(&mut self) {
        let mut delta: f32 = -1.0;
        while self.running {
            // Event update
            self.window.process_events();

            // Logic update
            self.active_scene.update(delta);

            // Render
            self.window.swap_buffer();
        }
    }
}
