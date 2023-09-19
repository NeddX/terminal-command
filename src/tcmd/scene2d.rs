use super::*;

pub const MAX_ENTITY_COUNT: usize = 1024;
pub const MAX_COMPONENT_COUNT: usize = 256;
pub const MAX_COMPONENT_PER_ENTITY: usize = 30;

pub struct Scene2D<'a> {
    entities: Vec<Box<Entity>>,
    camera: Camera2D,
    renderer: &'a mut Renderer2D
}

impl<'a> Scene2D<'a> {
    pub fn new(renderer: &'a mut Renderer2D) -> Self {
        return Scene2D {
            entities: Vec::new(),
            camera: Camera2D::new(Vector2::zero()),
            renderer: renderer
        };
    }

    pub fn update(&mut self, delta_time: f32) -> &mut Self {
        // Update entities
        for e in self.entities.iter_mut() {
            e.update(delta_time);
        }

        // Render entities

        for e in self.entities.iter_mut() {
            for c in e.components().iter() {
                // temporary!
                if c.name() == "sprite" {
                    // need reference to termgl to be able to render
                    //self.window_ref.draw_at();
                }
            }
        }
        return self;
    }
}
