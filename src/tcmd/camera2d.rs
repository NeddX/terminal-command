use super::*;

pub struct Camera2D {
    pos: Vector2
}

impl Camera2D {
    pub fn new(pos: Vector2) -> Self {
        return Camera2D {
            pos: pos
        };
    }
}
