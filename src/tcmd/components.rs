use super::*;

struct TagComponent {
    tag: String
}

impl IComponent for TagComponent {
    fn init(&mut self) {
        println!("Init Tag Component.");
    }
}

struct TransformComponent {
    pos: Vector2
}

impl IComponent for TransformComponent {
    fn init(&mut self) {
        println!("Init Transform Component.");
    }
}

struct CharSpriteComponent {
    visible: bool,
    width: u16,
    height: u16,
    char_sprite: Vec<char>
}

impl IComponent for CharSpriteComponent {
    fn init(&mut self) {
        println!("Init Char Sprite Component.");
    }

    fn visible(&self) -> bool {
        self.visible
    }
    fn name(&self) -> &str {
        "sprite"
    }
    fn set_visible(&mut self, new_visible: bool) {
        self.visible = new_visible;
    }
}

impl CharSpriteComponent {
    fn char_sprite(&self) -> &Vec<char> {
        &self.char_sprite
    }
    fn width(&self) -> u16 {
        self.width
    }
    fn height(&self) -> u16 {
        self.height
    }
    fn set_char_sprite(&mut self, new_char_sprite: Vec<char>) -> &mut Self {
        self.char_sprite = new_char_sprite;
        self
    }
    fn set_width(&mut self, new_width: u16) -> &mut Self {
        self.width = new_width;
        self
    }
    fn set_height(&mut self, new_height: u16) -> &mut Self {
        self.height = new_height;
        self
    }
}
