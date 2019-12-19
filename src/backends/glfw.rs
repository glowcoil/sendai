use crate::*;
use crate::gouache::{*, renderers:: GlRenderer};

use glfw::Context;

const FRAME: std::time::Duration = std::time::Duration::from_micros(1_000_000 / 60);

pub fn run<C: Component>(root: &mut C) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    let (mut window, _) = glfw.create_window(800, 600, "justitracker", glfw::WindowMode::Windowed).unwrap();

    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut cache = Cache::new();
    let mut renderer = GlRenderer::new();

    let mut context = crate::Context {
        cursor: Vec2::new(-1.0, -1.0),
        modifiers: Modifiers::default(),
        mouse_captured: false,
    };

    root.layout(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut last_frame = std::time::Instant::now();
    while !window.should_close() {
        let mut frame = Frame::new(&mut cache, &mut renderer, 800.0, 600.0);
        frame.clear(Color::rgba(0.1, 0.15, 0.2, 1.0));

        root.render(&mut frame);

        frame.finish();

        window.swap_buffers();

        let elapsed = last_frame.elapsed();
        if elapsed < FRAME {
            std::thread::sleep(FRAME - elapsed);
        }
        last_frame = std::time::Instant::now();

        glfw.poll_events_unbuffered(|_, (_, event)| {
            use glfw::WindowEvent;
            match event {
                WindowEvent::Size(width, height) => {
                    root.layout(Rect::new(0.0, 0.0, width as f32, height as f32));
                }
                WindowEvent::MouseButton(button, action, modifiers) => {
                    if let Some(button) = match button {
                        glfw::MouseButton::Button1 => Some(MouseButton::Left),
                        glfw::MouseButton::Button2 => Some(MouseButton::Middle),
                        glfw::MouseButton::Button3 => Some(MouseButton::Right),
                        _ => None,
                    } {
                        match action {
                            glfw::Action::Press | glfw::Action::Repeat => {
                                root.handle(Event::MouseDown(button), &mut context);
                            }
                            glfw::Action::Release => {
                                root.handle(Event::MouseUp(button), &mut context);
                            }
                        }
                    }
                }
                WindowEvent::CursorPos(x, y) => {
                    context.cursor = Vec2::new(x as f32, y as f32);
                    root.handle(Event::MouseMove, &mut context);
                }
                WindowEvent::Scroll(dx, dy) => {
                    root.handle(Event::Scroll(dx as f32, dy as f32), &mut context);
                }
                WindowEvent::Key(key, _scancode, action, modifiers) => {
                    if let Some(key) = glfw_key(key) {
                        match action {
                            glfw::Action::Press | glfw::Action::Repeat => {
                                root.handle(Event::KeyDown(key), &mut context);
                            }
                            glfw::Action::Release => {
                                root.handle(Event::KeyUp(key), &mut context);
                            }
                        }
                    }
                }
                WindowEvent::Char(c) => {
                    root.handle(Event::Char(c), &mut context);
                }
                WindowEvent::CharModifiers(c, modifiers) => {
                    root.handle(Event::Char(c), &mut context);
                }
                _ => {}
            }
            None
        });
    }
}

fn glfw_key(key: glfw::Key) -> Option<Key> {
    match key {
        glfw::Key::Space => Some(Key::Space),
        glfw::Key::Apostrophe => Some(Key::Apostrophe),
        glfw::Key::Comma => Some(Key::Comma),
        glfw::Key::Minus => Some(Key::Minus),
        glfw::Key::Period => Some(Key::Period),
        glfw::Key::Slash => Some(Key::Slash),
        glfw::Key::Num0 => Some(Key::Key0),
        glfw::Key::Num1 => Some(Key::Key1),
        glfw::Key::Num2 => Some(Key::Key2),
        glfw::Key::Num3 => Some(Key::Key3),
        glfw::Key::Num4 => Some(Key::Key4),
        glfw::Key::Num5 => Some(Key::Key5),
        glfw::Key::Num6 => Some(Key::Key6),
        glfw::Key::Num7 => Some(Key::Key7),
        glfw::Key::Num8 => Some(Key::Key8),
        glfw::Key::Num9 => Some(Key::Key9),
        glfw::Key::Semicolon => Some(Key::Semicolon),
        glfw::Key::Equal => Some(Key::Equals),
        glfw::Key::A => Some(Key::A),
        glfw::Key::B => Some(Key::B),
        glfw::Key::C => Some(Key::C),
        glfw::Key::D => Some(Key::D),
        glfw::Key::E => Some(Key::E),
        glfw::Key::F => Some(Key::F),
        glfw::Key::G => Some(Key::G),
        glfw::Key::H => Some(Key::H),
        glfw::Key::I => Some(Key::I),
        glfw::Key::J => Some(Key::J),
        glfw::Key::K => Some(Key::K),
        glfw::Key::L => Some(Key::L),
        glfw::Key::M => Some(Key::M),
        glfw::Key::N => Some(Key::N),
        glfw::Key::O => Some(Key::O),
        glfw::Key::P => Some(Key::P),
        glfw::Key::Q => Some(Key::Q),
        glfw::Key::R => Some(Key::R),
        glfw::Key::S => Some(Key::S),
        glfw::Key::T => Some(Key::T),
        glfw::Key::U => Some(Key::U),
        glfw::Key::V => Some(Key::V),
        glfw::Key::W => Some(Key::W),
        glfw::Key::X => Some(Key::X),
        glfw::Key::Y => Some(Key::Y),
        glfw::Key::Z => Some(Key::Z),
        glfw::Key::LeftBracket => Some(Key::LeftBracket),
        glfw::Key::Backslash => Some(Key::Backslash),
        glfw::Key::RightBracket => Some(Key::RightBracket),
        glfw::Key::GraveAccent => Some(Key::GraveAccent),
        glfw::Key::Enter => Some(Key::Enter),
        glfw::Key::Tab => Some(Key::Tab),
        glfw::Key::Backspace => Some(Key::Backspace),
        glfw::Key::Insert => Some(Key::Insert),
        glfw::Key::Delete => Some(Key::Delete),
        glfw::Key::Right => Some(Key::Right),
        glfw::Key::Left => Some(Key::Left),
        glfw::Key::Down => Some(Key::Down),
        glfw::Key::Up => Some(Key::Up),
        glfw::Key::PageUp => Some(Key::PageUp),
        glfw::Key::PageDown => Some(Key::PageDown),
        glfw::Key::Home => Some(Key::Home),
        glfw::Key::End => Some(Key::End),
        glfw::Key::CapsLock => Some(Key::CapsLock),
        glfw::Key::ScrollLock => Some(Key::ScrollLock),
        glfw::Key::NumLock => Some(Key::NumLock),
        glfw::Key::PrintScreen => Some(Key::PrintScreen),
        glfw::Key::Pause => Some(Key::Pause),
        glfw::Key::F1 => Some(Key::F1),
        glfw::Key::F2 => Some(Key::F2),
        glfw::Key::F3 => Some(Key::F3),
        glfw::Key::F4 => Some(Key::F4),
        glfw::Key::F5 => Some(Key::F5),
        glfw::Key::F6 => Some(Key::F6),
        glfw::Key::F7 => Some(Key::F7),
        glfw::Key::F8 => Some(Key::F8),
        glfw::Key::F9 => Some(Key::F9),
        glfw::Key::F10 => Some(Key::F10),
        glfw::Key::F11 => Some(Key::F11),
        glfw::Key::F12 => Some(Key::F12),
        glfw::Key::F13 => Some(Key::F13),
        glfw::Key::F14 => Some(Key::F14),
        glfw::Key::F15 => Some(Key::F15),
        glfw::Key::Kp0 => Some(Key::Numpad0),
        glfw::Key::Kp1 => Some(Key::Numpad1),
        glfw::Key::Kp2 => Some(Key::Numpad2),
        glfw::Key::Kp3 => Some(Key::Numpad3),
        glfw::Key::Kp4 => Some(Key::Numpad4),
        glfw::Key::Kp5 => Some(Key::Numpad5),
        glfw::Key::Kp6 => Some(Key::Numpad6),
        glfw::Key::Kp7 => Some(Key::Numpad7),
        glfw::Key::Kp8 => Some(Key::Numpad8),
        glfw::Key::Kp9 => Some(Key::Numpad9),
        glfw::Key::KpDecimal => Some(Key::NumpadDecimal),
        glfw::Key::KpDivide => Some(Key::NumpadDivide),
        glfw::Key::KpMultiply => Some(Key::NumpadMultiply),
        glfw::Key::KpSubtract => Some(Key::NumpadSubtract),
        glfw::Key::KpAdd => Some(Key::NumpadAdd),
        glfw::Key::KpEnter => Some(Key::NumpadEnter),
        glfw::Key::KpEqual => Some(Key::NumpadEquals),
        glfw::Key::LeftShift => Some(Key::LeftShift),
        glfw::Key::LeftControl => Some(Key::LeftControl),
        glfw::Key::LeftAlt => Some(Key::LeftAlt),
        glfw::Key::LeftSuper => Some(Key::LeftMeta),
        glfw::Key::RightShift => Some(Key::RightShift),
        glfw::Key::RightControl => Some(Key::RightControl),
        glfw::Key::RightAlt => Some(Key::RightAlt),
        glfw::Key::RightSuper => Some(Key::RightMeta),
        _ => None,
    }
}
