use super::*;

pub trait IComponent {
    fn init(&mut self) { }
    fn update(&mut self, delta_time: f32) { }

    fn disabled(&self) -> bool { true }
    fn visible(&self) -> bool { false }
    fn name(&self) -> &str { "generic" }

    fn set_disabled(&mut self, new_disabled: bool) { }
    fn set_visible(&mut self, new_visible: bool) { }
}

pub struct Entity {
    pos: Vector2,
    components: Vec<Box<dyn IComponent>>
}

impl Entity {
    pub fn new(pos: Vector2) -> Self {
        return Entity {
            pos: pos,
            components: Vec::new()
        };
    }

    pub fn update(&mut self, delta_time: f32) -> &mut Self {
        for c in self.components.iter_mut() {
            c.update(delta_time);
        }
        return self;
    }

    pub fn components(&self) -> &Vec<Box<dyn IComponent>> {
        &self.components
    }
}
