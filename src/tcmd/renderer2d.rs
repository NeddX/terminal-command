use super::*;

struct Renderer2D {
    window: termgl::Window,
    scene:
}

impl Renderer2D {
    pub fn new(width: u16, height: u16, delay: u16) -> Self {
        return termgl::Renderer2D {
            window: termgl::Window::new(width, height, delay)
        };
    }

    pub fn window(&mut self) -> &mut termgl::Window {
        self.window
    }
}
