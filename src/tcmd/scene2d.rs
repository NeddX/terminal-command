use super::*;

pub struct Scene2D {
    entities: Vec<Entity>,
    camera: Camera2D
}

impl Scene2D {
    pub fn new(entities: Vec<Entity>) -> Self {
        return Scene2D {
            entities: entities,
            camera: Camera2D::new(Vector2::zero())
        };
    }
}
