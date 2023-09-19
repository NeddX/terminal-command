use super::*;

pub struct Application {
    title: String,
    running: bool,
    renderer: Renderer2D
}

impl Application {
    pub fn new(title: String, width: u16, height: u16, delay: u32) -> Self {
        RENDERER = Renderer2D::new(width, height, delay);
        return Application {
            title: title,
            running: true,
            renderer: Renderer2d::new(width, height, delay)
        };
    }

    pub fn run(&mut self) {
        let mut window = self.renderer.window();
        let mut delta: f32 = -1.0;
        while self.running {
            // Event update
            window.process_events();

            // Logic update
            self.renderer.scene().update(delta);
            //self.active_scene.update(delta);

            // Render
            window.swap_buffer();
        }
    }
}
