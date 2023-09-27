use super::*;

pub struct Application {
    running: bool,
    window: termgl::Window,
    scene: Scene2D
}

impl Application {
    pub fn new(width: u16, height: u16, delay: u32) -> Self {
        Application {
            running: true,
            window: termgl::Window::new(width, height, delay),
            scene: Scene2D::new()
        }
    }

    pub fn run(&mut self) {
        let mut delta: f32 = -1.0;
        self.scene.add_entity(Entity::new())
                  .add_component(CharSpriteComponent::new(1, 1, vec!['@']));
        while self.running {
            // Event update
            _ = self.window.process_events();

            // Logic update
            self.scene.update(&mut self.window, delta);

            // Render
            _ = self.window.swap_buffer();
        }
    }
}
