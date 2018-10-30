use crate::state::State as AppState;

use num_traits::Zero;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MouseCoords(glm::IVec2);

impl Default for MouseCoords {
    fn default() -> MouseCoords {
        MouseCoords(glm::IVec2::zero())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<MouseButton>,
    prev_pressed_keys: HashSet<Keycode>,
    prev_pressed_buttons: HashSet<MouseButton>,
    mouse_coords: MouseCoords,
    game_quit: bool,
}

impl State {
    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn did_press_key(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.prev_pressed_keys.contains(&keycode)
    }

    pub fn did_release_key(&self, keycode: Keycode) -> bool {
        !self.pressed_keys.contains(&keycode) && self.prev_pressed_keys.contains(&keycode)
    }

    pub fn did_click_mouse(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button)
            && !self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn did_release_mouse(&self, mouse_button: MouseButton) -> bool {
        !self.pressed_buttons.contains(&mouse_button)
            && self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button)
    }

    pub fn mouse_coords(&self) -> glm::IVec2 {
        self.mouse_coords.0
    }

    pub fn game_quit(&self) -> bool {
        self.game_quit
    }

    pub fn update(&mut self, events: impl Iterator<Item = Event>) -> AppState<&mut Self, ()> {
        self.prev_pressed_keys = self.pressed_keys.clone();
        self.prev_pressed_buttons = self.pressed_buttons.clone();

        for event in events {
            match event {
                Event::Quit { .. } => return AppState::Quit(()),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.pressed_keys.insert(keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.pressed_keys.remove(&keycode);
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_coords = MouseCoords(glm::ivec2(x, y));
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.pressed_buttons.insert(mouse_btn);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.pressed_buttons.remove(&mouse_btn);
                }
                _ => {}
            }
        }

        AppState::Running(self)
    }
}
