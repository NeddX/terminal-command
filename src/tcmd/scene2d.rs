use super::*;
use std::any::Any;

pub const MAX_ENTITY_COUNT: usize = 1024;
pub const MAX_COMPONENT_COUNT: usize = 256;
pub const MAX_COMPONENT_PER_ENTITY: usize = 30;

pub struct Scene2D {
    entities: Vec<Box<Entity>>,
    camera: Camera2D
}

impl Scene2D {
    pub fn new(window: &Window) -> Self {
        return Scene2D {
            entities: Vec::new(),
            camera: Camera2D::new(Vector2::zero())
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
