#[derive(Copy, Clone)]
pub struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        return Vector2 {
            x: x,
            y: y
        };
    }

    pub fn zero() -> Self {
        return Vector2 {
            x: 0,
            y: 0
        };
    }
}
