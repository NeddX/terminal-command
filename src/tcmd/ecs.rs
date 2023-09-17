use super::*;

pub struct Component {

}

pub struct Entity {
    tag: String,
    pos: Vector2,
    components: Vec<Component>
}
