use std::io::Error;

use crossterm::{self, ExecutableCommand};

// Wrappers around their corssterm equivilants.
pub struct KeyEvent(crossterm::event::KeyEvent);
pub struct MouseEvent(crossterm::event::MouseEvent);

pub type KeyEventDelegate = fn(event_args: KeyEvent);
pub type MouseEventDelegate = fn(event_args: MouseEvent);

pub struct Window {
    width: u16,
    height: u16,
    framebuffer: Vec<char>,
    stdout: std::io::Stdout,
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
            stdout: std::io::stdout(),
            delay: delay,
            event_on_key_down: Option::None,
            event_on_key_release: Option::None,
            event_on_key_repeat: Option::None,
            event_on_mouse_down: Option::None,
            event_on_mouse_release: Option::None,
            event_on_mouse_move: Option::None,
            disposed: false,
        };

        crossterm::terminal::enable_raw_mode().unwrap();
        inst.stdout.execute(crossterm::cursor::Hide).unwrap();

        inst
    }

    fn dispose(&mut self) {
        if !self.disposed {
            self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
            self.stdout.execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
            self.stdout.execute(crossterm::cursor::Show).unwrap();
            crossterm::terminal::disable_raw_mode().unwrap();
            self.disposed = true;
        }
    }

    pub fn process_events(&mut self) -> Result<(), Error> {
        if crossterm::event::poll(std::time::Duration::from_millis(self.delay as u64))? {
            match crossterm::event::read()? {
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
       Result::Ok(())
    }

    pub fn draw_at(&mut self, x: u16, y: u16, c: char) -> &mut Self {
        self.framebuffer[(y * self.width + x) as usize] = c;
        self
    }

    pub fn swap_buffer(&mut self) -> Result<&mut Self, Error> {
        self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        self.stdout.execute(crossterm::cursor::MoveTo(0, 0))?;

        for i in 0..(self.width * self.height) as usize {
            print!("{}", self.framebuffer[i]);
            if (i + 1) % self.width as usize == 0 {
                self.stdout.execute(crossterm::cursor::MoveTo(0, ((i + 1) / self.width as usize) as u16))?;
            }
        }

        self.framebuffer.fill(0 as char);
        Result::Ok(self)
    }

    pub fn attach_on_key_down(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        self
    }

    pub fn attach_on_key_release(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        self
    }

    pub fn attach_on_key_repeat(&mut self, callback: KeyEventDelegate) -> &mut Self {
        self.event_on_key_down = Option::Some(callback);
        self
    }

    pub fn attach_on_mouse_down(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_down = Option::Some(callback);
        self
    }

    pub fn attach_on_mouse_release(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_release = Option::Some(callback);
        self
    }

    pub fn attach_on_mouse_move(&mut self, callback: MouseEventDelegate) -> &mut Self {
        self.event_on_mouse_move = Option::Some(callback);
        self
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Call dispose upon destruction.
        self.dispose();
    }
}
