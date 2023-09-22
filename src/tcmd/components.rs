use super::*;

pub struct TagComponent {
    pub tag: String
}

impl IComponent for TagComponent {
    fn init(&mut self) {
        println!("Init Tag Component.");
    }
}

pub struct TransformComponent {
    pub pos: Vector2
}

impl TransformComponent {
    pub fn pos(&mut self) -> Vector2 {
        self.pos
    }
}

impl IComponent for TransformComponent {
    fn init(&mut self) {
        println!("Init Transform Component.");
    }
}

pub struct CharSpriteComponent {
    visible: bool,
    pub width: u16,
    pub height: u16,
    char_sprite: Vec<char>
}

impl IComponent for CharSpriteComponent {
    fn init(&mut self) {
        println!("Init Char Sprite Component.");
    }

    fn visible(&self) -> bool {
        self.visible
    }
    fn set_visible(&mut self, new_visible: bool) {
        self.visible = new_visible;
    }
}

impl CharSpriteComponent {
    pub fn char_sprite(&self) -> &Vec<char> {
        &self.char_sprite
    }
    pub fn set_char_sprite(&mut self, new_char_sprite: Vec<char>) -> &mut Self {
        self.char_sprite = new_char_sprite;
        self
    }
}
