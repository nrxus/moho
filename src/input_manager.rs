use std::collections::HashSet;

use glm;
use num_traits::Zero;
use sdl2::EventPump as SdlEventPump;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

struct EventGenerator<E: EventPump> {
    event_pump: E,
}

impl<E: EventPump> EventGenerator<E> {
    fn new(event_pump: E) -> Self {
        EventGenerator { event_pump: event_pump }
    }

    fn iter(&mut self) -> EventIterator<E> {
        EventIterator { event_pump: &mut self.event_pump }
    }
}

struct EventIterator<'a, E: EventPump + 'a> {
    event_pump: &'a mut E,
}

impl<'a, E: EventPump> Iterator for EventIterator<'a, E> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}

pub trait EventPump {
    fn poll_event(&mut self) -> Option<Event>;
}

impl EventPump for SdlEventPump {
    fn poll_event(&mut self) -> Option<Event> {
        self.poll_event()
    }
}

pub struct InputManager<P: EventPump> {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<MouseButton>,
    prev_pressed_keys: HashSet<Keycode>,
    prev_pressed_buttons: HashSet<MouseButton>,
    mouse_coords: glm::IVec2,
    event_generator: EventGenerator<P>,
    game_quit: bool,
}

impl<P: EventPump> InputManager<P> {
    pub fn new(event_pump: P) -> InputManager<P> {
        InputManager {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            prev_pressed_keys: HashSet::new(),
            prev_pressed_buttons: HashSet::new(),
            mouse_coords: glm::IVec2::zero(),
            event_generator: EventGenerator::new(event_pump),
            game_quit: false,
        }
    }

    pub fn update(&mut self) {
        self.prev_pressed_keys = self.pressed_keys.clone();
        self.prev_pressed_buttons = self.pressed_buttons.clone();

        for event in self.event_generator.iter() {
            match event {
                Event::Quit { .. } => {
                    self.game_quit = true;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.pressed_keys.insert(keycode);
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.pressed_keys.remove(&keycode);
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_coords = glm::ivec2(x, y);
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
