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
    components: HashMap<TypeId, Box<dyn IComponent>>
}

impl Entity {
    pub fn new(pos: Vector2) -> Self {
        return Entity {
            pos: pos,
            components: HashMap::new()
        };
    }

    pub fn get_component<T, 'a>(&'a mut self) &'a mut {
        let id = TypeId::of::<T>();
        return self.components.get(id);
    }

    pub fn update(&mut self, delta_time: f32) -> &mut Self {
        for (_, v) in self.components.iter_mut() {
            v.update(delta_time);
        }
        return self;
    }
}
