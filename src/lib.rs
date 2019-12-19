pub mod backends;
pub mod event;

pub use event::*;

pub use gouache;

use gouache::*;
use std::rc::Rc;

pub trait Component {
    fn layout(&mut self, rect: Rect) -> Rect;
    fn render(&self, frame: &mut Frame);
    fn handle(&mut self, event: Event, context: &mut Context) -> bool;
}

pub struct Context {
    pub cursor: Vec2,
    pub modifiers: Modifiers,
    pub mouse_captured: bool,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rect {
        Rect { left, top, right, bottom }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left && point.x < self.right &&
        point.y >= self.top && point.y < self.bottom
    }

    pub fn pos(&self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.right - self.left, self.bottom - self.top)
    }
}

pub struct Button {
    rect: Rect,
    icon: Path,
    hover: bool,
    down: bool,
}

impl Button {
    pub fn new(icon: Path) -> Button {
        Button {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            icon,
            hover: false,
            down: false,
        }
    }
}

impl Component for Button {
    fn layout(&mut self, rect: Rect) -> Rect {
        self.rect = rect;
        self.rect
    }

    fn render(&self, frame: &mut Frame) {
        let color = if self.down {
            Color::rgba(0.141, 0.44, 0.77, 1.0)
        } else if self.hover {
            Color::rgba(0.54, 0.63, 0.71, 1.0)
        } else {
            Color::rgba(0.38, 0.42, 0.48, 1.0)
        };

        frame.draw_rect(self.rect.pos(), self.rect.size(), Mat2x2::id(), color);
        frame.draw_path(&self.icon, self.rect.pos(), Mat2x2::id(), Color::rgba(1.0, 1.0, 1.0, 1.0));
    }

    fn handle(&mut self, event: Event, context: &mut Context) -> bool {
        match event {
            Event::MouseMove => {
                self.hover = self.rect.contains(context.cursor);
            }
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
    fn layout(&mut self, rect: Rect) -> Rect {
        self.rect = rect;
        self.rect
    }

    fn render(&self, frame: &mut Frame) {
        let color = if self.focus {
            Color::rgba(0.43, 0.50, 0.66, 1.0)
        } else {
            Color::rgba(0.21, 0.27, 0.32, 1.0)
        };

        frame.draw_rect(self.rect.pos(), self.rect.size(), Mat2x2::id(), color);
        frame.draw_text(&self.font, 14.0, &self.text, self.rect.pos(), Mat2x2::id(), Color::rgba(1.0, 1.0, 1.0, 1.0));
    }

    fn handle(&mut self, event: Event, context: &mut Context) -> bool {
        match event {
            Event::Char(c) => {
                self.text.push(c);
            }
            _ => {}
        }
        false
    }
}
