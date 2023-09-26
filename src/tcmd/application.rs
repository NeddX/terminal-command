use super::*;

pub struct Application {
    running: bool,
    window: termgl::Window,
    scene: Scene2D
}

impl Application {
    pub fn new(width: u16, height: u16, delay: u32) -> Self {
        return Application {
            running: true,
            window: termgl::Window::new(width, height, delay),
            scene: Scene2D::new()
        };
    }

    pub fn run(&mut self) {
        let mut delta: f32 = -1.0;
        // FIXME: Doesn't work, throws an incomprehensible error
        // which is a classic one for all systems programming languages and
        // rust definetly doesn't solve it.
        self.scene.add_entity(Entity::new(Vector2::new(3, 3)))
                  .add_component(CharSpriteComponent::new(2, 2, vec!['@']));
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
