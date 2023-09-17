use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand
    //event::{self, Event, KeyCode, KeyEvent, MouseEvent, KeyModifiers}
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

// Wrappers around their corssterm equivilants.
pub struct KeyEvent(crossterm::event::KeyEvent);
pub struct MouseEvent(crossterm::event::MouseEvent);

type KeyEventDelegate = fn(event_args: KeyEvent);
type MouseEventDelegate = fn(event_args: MouseEvent);

pub struct Window {
    width: u16,
    height: u16,
    framebuffer: Vec<char>,
    stdout: io::Stdout,
    delay: u32,
    event_on_key_down: Option<KeyEventDelegate>,
    event_on_key_release: Option<KeyEventDelegate>,
    event_on_key_repeat: Option<KeyEventDelegate>,
    event_on_mouse_down: Option<MouseEventDelegate>,
    event_on_mouse_release: Option<MouseEventDelegate>,
    event_on_mouse_move: Option<MouseEventDelegate>,
    disposed: bool
}

impl Window {
    pub fn new(width: u16, height: u16, delay: u32) -> Self {
        let mut inst = Window {
            width: width,
            height: height,
            framebuffer: vec!['\0'; (width * height) as usize],
            stdout: io::stdout(),
            delay: delay,
            event_on_key_down: Option::None,
            event_on_key_release: Option::None,
            event_on_key_repeat: Option::None,
            event_on_mouse_down: Option::None,
            event_on_mouse_release: Option::None,
            event_on_mouse_move: Option::None,
            disposed: false,
        };

        terminal::enable_raw_mode().unwrap();
        inst.stdout.execute(cursor::Hide).unwrap();

        return inst;
    }

    fn dispose(&mut self) {
        if !self.disposed {
            self.stdout.execute(terminal::Clear(ClearType::All)).unwrap();
            self.stdout.execute(cursor::MoveTo(0, 0)).unwrap();
            self.stdout.execute(cursor::Show).unwrap();
            terminal::disable_raw_mode().unwrap();
            self.disposed = true;
        }
    }

    fn process_events(&mut self) {
        if crossterm::event::poll(Duration::from_millis(self.delay as u64)).unwrap() {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    match key_event.kind {
                        crossterm::event::KeyEventKind::Press => {
                            if let Option::Some(callback) = self.event_on_key_down {
                                callback(KeyEvent(key_event));
                            }
                        },
                        crossterm::event::KeyEventKind::Release => {
                            if let Option::Some(callback) = self.event_on_key_release {
                                callback(KeyEvent(key_event));
                            }
                        },
                        crossterm::event::KeyEventKind::Repeat => {
                            if let Option::Some(callback) = self.event_on_key_repeat {
                                callback(KeyEvent(key_event));
                            }
                        }
                    }
                },
                crossterm::event::Event::Mouse(mouse_event) => {
                    // Process mosue event
                },
                crossterm::event::Event::Resize(w, h) => {
                    // Process resize event
                },
                _ => {  }
            }
        }
    }

    pub fn draw_at(&mut self, x: u16, y: u16, c: char) -> &mut Self {
        self.framebuffer[(y * self.width + x) as usize] = c;
        return self;
    }

    pub fn swap_buffer(&mut self) -> &mut Self {
        self.stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        self.stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        for i in 0..(self.width * self.height) as usize {
            print!("{}", self.framebuffer[i]);
            if (i + 1) % self.width as usize == 0 {
                self.stdout.execute(cursor::MoveTo(0, ((i + 1) / self.width as usize) as u16)).unwrap();
            }
        }

        self.framebuffer.clear();
        return self;
    }

    pub fn attach_on_key_down(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        return self;
    }

    pub fn attach_on_key_release(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        return self;
    }

    pub fn attach_on_key_repeat(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        return self;
    }

    pub fn attach_on_mouse_down(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_down = Option::Some(callback);
        return self;
    }

    pub fn attach_on_mouse_release(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_release = Option::Some(callback);
        return self;
    }

    pub fn attach_on_mouse_move(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_move = Option::Some(callback);
        return self;
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Call dispose upon destruction.
        self.dispose();
    }
}
