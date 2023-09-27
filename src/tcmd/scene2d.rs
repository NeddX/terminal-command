use super::*;

pub struct Scene2D {
    entities: Vec<Box<Entity>>,
    camera: Camera2D
}

impl Scene2D {
    pub fn new() -> Self {
        Scene2D {
            entities: Vec::new(),
            camera: Camera2D::new(Vector2::zero())
        }
    }

    pub fn add_entity<'a>(&'a mut self, args: Entity) -> &'a mut Box<Entity> {
        let mut ent = Box::new(args);
        ent.add_component(TransformComponent::new(Vector2::zero()));
        self.entities.push(ent);
        self.entities.last_mut().unwrap()
    }

    pub fn update(&mut self, window: &mut termgl::Window, delta_time: f32) -> &mut Self {
        // Update entities
        for e in self.entities.iter_mut() {
            e.update(delta_time);
        }

        // Render entities
        for e in self.entities.iter_mut() {
            if let Option::Some(c) = e.get_component::<CharSpriteComponent>() {
                let trans = e.get_component::<TransformComponent>().unwrap();
                let char_sprite = c.char_sprite();
                window.draw_at(
                    trans.pos.x as u16,
                    trans.pos.y as u16,
                    char_sprite[0]
                );
            }
        }
        self
    }
}
