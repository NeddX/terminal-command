use super::*;

pub trait IComponent : AsAny {
    fn init(&mut self) { }
    fn update(&mut self, delta_time: f32) { }

    fn disabled(&self) -> bool { true }
    fn visible(&self) -> bool { false }

    fn set_disabled(&mut self, new_disabled: bool) { }
    fn set_visible(&mut self, new_visible: bool) { }
}

pub struct Entity {
    pos: Vector2,
    components: HashMap<TypeId, RefCell<Box<dyn IComponent>>>
}

impl Entity {
    pub fn new(pos: Vector2) -> Self {
        return Entity {
            pos: pos,
            components: HashMap::new()
        };
    }

    pub fn get_component<T: IComponent + 'static>(&self) -> Option<RefMut<T>> {
        let id = TypeId::of::<T>();
        match self.components.get(&id) {
            Option::Some(comp_cell) => {
                return Some(RefMut::map(comp_cell.borrow_mut(), |t| {
                    t.downcast_mut::<T>().unwrap()
                }));
            },
            _ => {
                return Option::None;
            }
        }
    }

    pub fn add_component<T: IComponent + 'static>(&mut self, args: T) -> RefMut<T> {
        let c = RefCell::new(Box::new(args));
        self.components.insert(TypeId::of::<T>(), c);
        return RefMut::map(self.components.values().last().unwrap().borrow_mut(),
                           |t| {
                               t.downcast_mut::<T>().unwrap()
                           });
    }

    pub fn update(&mut self, delta_time: f32) -> &mut Self {
        for (_, v) in self.components.iter_mut() {
            v.borrow_mut().update(delta_time);
        }
        return self;
    }
}
