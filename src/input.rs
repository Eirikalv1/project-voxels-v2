use glam::{vec2, Vec2};
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::ScreenSpace;

pub struct InputState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub right_mouse_button: bool,
    pub delta_mouse_position: Vec2,
    pub mouse_position: Vec2,
    previous_mouse_position: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            q: false,
            e: false,
            right_mouse_button: false,
            delta_mouse_position: Vec2::ZERO,
            mouse_position: Vec2::ZERO,
            previous_mouse_position: Vec2::ZERO,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent { state, physical_key, .. },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        self.w = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        self.s = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        self.a = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        self.d = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyQ) => {
                        self.q = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyE) => {
                        self.e = is_pressed;
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = vec2(position.x as f32, position.y as f32).to_screen_space(&800.0, &800.0);
            }
            WindowEvent::MouseInput { button, state, .. } => match button {
                MouseButton::Left => {
                    if *state == ElementState::Pressed {
                        self.right_mouse_button = true;
                    }
                    if *state == ElementState::Released {
                        self.right_mouse_button = false;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn after_main_events(&mut self) {
        self.delta_mouse_position = self.mouse_position - self.previous_mouse_position;
        self.previous_mouse_position = self.mouse_position;
    }
}
