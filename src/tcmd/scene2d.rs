pub struct Scene2D {
    entities: Vec<crate::tcmd::Entity>,
    camera: crate::tcmd::Camera2D
}

impl Scene2D {
    pub fn new(entities: Vec<crate::tcmd::Entity>) -> Self {
        return crate::tcmd::Scene2D {
            entities: entities,
            camera: crate::tcmd::Camera2D::new(crate::tcmd::Vector2::zero())
        };
    }
}
