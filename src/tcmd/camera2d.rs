pub struct Camera2D {
    pos: crate::tcmd::Vector2
}

impl Camera2D {
    pub fn new(pos: crate::tcmd::Vector2) -> Self {
        return crate::tcmd::Camera2D {
            pos: pos
        };
    }
}
