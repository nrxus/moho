use glm;
use num_traits::Zero;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;

pub struct State {
    pub(super) pressed_keys: HashSet<Keycode>,
    pub(super) pressed_buttons: HashSet<MouseButton>,
    pub(super) prev_pressed_keys: HashSet<Keycode>,
    pub(super) prev_pressed_buttons: HashSet<MouseButton>,
    pub(super) mouse_coords: glm::IVec2,
    pub(super) game_quit: bool,
}

impl State {
    pub fn new() -> State {
        State {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            prev_pressed_keys: HashSet::new(),
            prev_pressed_buttons: HashSet::new(),
            mouse_coords: glm::IVec2::zero(),
            game_quit: false,
        }
    }

    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn did_press_key(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.prev_pressed_keys.contains(&keycode)
    }

    pub fn did_click_mouse(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button) &&
        !self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn did_release_mouse(&self, mouse_button: MouseButton) -> bool {
        !self.pressed_buttons.contains(&mouse_button) &&
        self.prev_pressed_buttons.contains(&mouse_button)
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        self.pressed_buttons.contains(&mouse_button)
    }

    pub fn mouse_coords(&self) -> glm::IVec2 {
        self.mouse_coords
    }

    pub fn game_quit(&self) -> bool {
        self.game_quit
    }
}
