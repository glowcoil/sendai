pub mod backends;

pub use gouache;

use gouache::*;
use std::rc::Rc;

pub trait Component {
    fn size(&self, space: Vec2) -> Vec2;
    fn place(&mut self, rect: Rect);
    fn event(&mut self, event: Event, context: &mut Context) -> bool;
    fn draw(&self, frame: &mut Frame, context: &Context);
}

pub struct Context {
    pub cursor: Vec2,
    pub modifiers: Modifiers,
    pub mouse_captured: bool,
}

pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect { pos: Vec2::new(x, y), size: Vec2::new(width, height) }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.pos.x && point.x < self.pos.x + self.size.x &&
        point.y >= self.pos.y && point.y < self.pos.y + self.size.y
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    MouseMove,
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    Scroll(f32, f32),
    KeyDown(Key),
    KeyUp(Key),
    Char(char),
}

#[derive(Copy, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Default for Modifiers {
    fn default() -> Modifiers {
        Modifiers {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    GraveAccent,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    PrintScreen,
    ScrollLock,
    Pause,
    Backspace,
    Tab,
    CapsLock,
    Enter,
    Space,
    Insert,
    Delete,
    PageUp,
    PageDown,
    Home,
    End,
    Left,
    Right,
    Up,
    Down,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadDecimal,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadEnter,
    NumpadEquals,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightMeta,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct Button {
    rect: Rect,
    icon: Path,
    down: bool,
}

impl Button {
    pub fn new(icon: Path) -> Button {
        Button {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            icon,
            down: false,
        }
    }
}

impl Component for Button {
    fn size(&self, space: Vec2) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    fn place(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&self, frame: &mut Frame, context: &Context) {
        let color = if self.down {
            Color::rgba(0.141, 0.44, 0.77, 1.0)
        } else if self.rect.contains(context.cursor) {
            Color::rgba(0.54, 0.63, 0.71, 1.0)
        } else {
            Color::rgba(0.38, 0.42, 0.48, 1.0)
        };

        frame.draw_rect(self.rect.pos, self.rect.size, Mat2x2::id(), color);
        frame.draw_path(&self.icon, self.rect.pos, Mat2x2::id(), Color::rgba(1.0, 1.0, 1.0, 1.0));
    }

    fn event(&mut self, event: Event, context: &mut Context) -> bool {
        match event {
            Event::MouseDown(MouseButton::Left) => {
                if !context.mouse_captured && self.rect.contains(context.cursor) {
                    self.down = true;
                    context.mouse_captured = true;
                }
            }
            Event::MouseUp(MouseButton::Left) => {
                if self.down {
                    context.mouse_captured = false;
                    self.down = false;
                    if self.rect.contains(context.cursor) {
                        return true;
                    }
                }
            }
            _ => {}
        }
        false
    }
}

pub struct Textbox {
    rect: Rect,
    focus: bool,
    font: Rc<Font<'static>>,
    text: String,
}

impl Textbox {
    pub fn new(font: Rc<Font<'static>>) -> Textbox {
        Textbox {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            focus: false,
            font,
            text: String::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

impl Component for Textbox {
    fn size(&self, space: Vec2) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    fn place(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&self, frame: &mut Frame, context: &Context) {
        let color = if self.focus {
            Color::rgba(0.43, 0.50, 0.66, 1.0)
        } else {
            Color::rgba(0.21, 0.27, 0.32, 1.0)
        };

        frame.draw_rect(self.rect.pos, self.rect.size, Mat2x2::id(), color);
        frame.draw_text(&self.font, 14.0, &self.text, self.rect.pos, Mat2x2::id(), Color::rgba(1.0, 1.0, 1.0, 1.0));
    }

    fn event(&mut self, event: Event, context: &mut Context) -> bool {
        match event {
            Event::Char(c) => {
                self.text.push(c);
            }
            _ => {}
        }
        false
    }
}
