use std::borrow::BorrowMut;

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
        let type_id = TypeId::of::<T>();
        match self.components.get(&type_id) {
            Option::Some(component) => {
                let try_borrow = component.try_borrow_mut().unwrap();
                return Option::Some(RefMut::map(try_borrow, |x| -> &mut T {
                    (**x).downcast_mut::<T>().unwrap()
                }));
            },
            Option::None => {
                return Option::None;
            }
        }
    }

    pub fn add_component<T: IComponent + 'static>(&mut self, args: T) -> Option<RefMut<T>> {
        let c = RefCell::new(Box::new(args));
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, c);
        let try_borrow = self.components.get(&type_id).unwrap().try_borrow_mut().unwrap();
        return Option::Some(RefMut::map(try_borrow, |x| -> &mut T {
            (**x).downcast_mut::<T>().unwrap()
        }));
    }

    pub fn update(&mut self, delta_time: f32) -> &mut Self {
        for (_, v) in self.components.iter_mut() {
            let a = v.get_mut();
            //v.borrow_mut().update(delta_time);
        }
        return self;
    }
}
