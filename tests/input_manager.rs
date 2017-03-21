extern crate moho;
extern crate glm;
extern crate sdl2;

use moho::input_manager::*;
use sdl2::keyboard::{Keycode, NOMOD};
use sdl2::event::Event;
use sdl2::mouse::{MouseState, MouseButton};

struct MockEventPump {
    streams: Vec<Option<Event>>,
}

impl EventPump for MockEventPump {
    fn poll_event(&mut self) -> Option<Event> {
        self.streams.pop().unwrap()
    }
}

macro_rules! key_event {
    ($t:ident, $e:expr) => {
        {
            Event::$t {
                keycode: Some($e),
                timestamp: 0,
                window_id: 0,
                scancode: None,
                repeat: false,
                keymod: NOMOD,
            }
        }
    };
}

macro_rules! mouse_event {
    ($t:ident, $e:expr) => {
        {
            Event::$t {
                mouse_btn: $e,
                timestamp: 0,
                window_id: 0,
                which: 0,
                x: 0,
                y: 0,
            }
        }
    };
}

#[test]
fn press_keys() {
    let streams = vec![None,
                       Some(key_event!(KeyDown, Keycode::Up)),
                       Some(key_event!(KeyDown, Keycode::Down))];

    let mut subject = InputManager::new(MockEventPump { streams: streams });

    // Nothing is set before
    assert_eq!(subject.is_key_down(Keycode::Down), false);
    assert_eq!(subject.is_key_down(Keycode::Up), false);

    subject.update();

    // Both keys are set after
    assert_eq!(subject.is_key_down(Keycode::Down), true);
    assert_eq!(subject.is_key_down(Keycode::Up), true);
}

#[test]
fn release_keys() {
    let streams = vec![None,
                       Some(key_event!(KeyUp, Keycode::Down)),
                       None,
                       Some(key_event!(KeyDown, Keycode::Down)),
                       Some(key_event!(KeyDown, Keycode::Up))];

    let mut subject = InputManager::new(MockEventPump { streams: streams });
    subject.update();

    // Both keys set after
    assert_eq!(subject.is_key_down(Keycode::Down), true);
    assert_eq!(subject.is_key_down(Keycode::Up), true);
    subject.update();

    // Only the one released unset after
    assert_eq!(subject.is_key_down(Keycode::Down), false);
    assert_eq!(subject.is_key_down(Keycode::Up), true);
}

#[test]
fn did_press_key() {
    let streams = vec![None,
                       Some(key_event!(KeyUp, Keycode::Down)),
                       Some(key_event!(KeyDown, Keycode::Up)),
                       None,
                       Some(key_event!(KeyDown, Keycode::Down))];

    let mut subject = InputManager::new(MockEventPump { streams: streams });

    // Nothing has been pressed
    assert_eq!(subject.did_press_key(Keycode::Down), false);
    assert_eq!(subject.did_press_key(Keycode::Up), false);

    // Down key is pressed
    subject.update();
    assert_eq!(subject.did_press_key(Keycode::Down), true);
    assert_eq!(subject.did_press_key(Keycode::Up), false);

    // Up key is pressed - Down key has not been released yet
    subject.update();
    assert_eq!(subject.did_press_key(Keycode::Down), false);
    assert_eq!(subject.did_press_key(Keycode::Up), true);
}

#[test]
fn mouse_coords() {
    let streams = vec![None,
                       Some(Event::MouseMotion {
                                timestamp: 0,
                                window_id: 0,
                                which: 0,
                                mousestate: MouseState::from_sdl_state(0),
                                x: 50,
                                y: 30,
                                xrel: 0,
                                yrel: 0,
                            })];

    let mut subject = InputManager::new(MockEventPump { streams: streams });
    subject.update();
    assert_eq!(subject.mouse_coords(), glm::ivec2(50, 30));
}

#[test]
fn mouse_clicks() {
    let streams = vec![None,
                       Some(mouse_event!(MouseButtonDown, MouseButton::Right)),
                       None,
                       Some(mouse_event!(MouseButtonDown, MouseButton::Left))];

    let mut subject = InputManager::new(MockEventPump { streams: streams });

    // Nothing has been clicked
    assert_eq!(subject.did_click_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_click_mouse(MouseButton::Left), false);

    // Left button is click
    subject.update();
    assert_eq!(subject.did_click_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_click_mouse(MouseButton::Left), true);

    // Right button is clicked - left button is still pressed but not a recent click
    subject.update();
    assert_eq!(subject.did_click_mouse(MouseButton::Right), true);
    assert_eq!(subject.did_click_mouse(MouseButton::Left), false);
}

#[test]
fn mouse_releases() {
    let streams = vec![None,
                       Some(mouse_event!(MouseButtonDown, MouseButton::Right)),
                       None,
                       Some(mouse_event!(MouseButtonUp, MouseButton::Left)),
                       None,
                       Some(mouse_event!(MouseButtonDown, MouseButton::Left))];

    let mut subject = InputManager::new(MockEventPump { streams: streams });

    // Nothing has been clicked
    assert_eq!(subject.did_release_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_release_mouse(MouseButton::Left), false);

    // Left button is click
    subject.update();
    assert_eq!(subject.did_release_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_release_mouse(MouseButton::Left), false);

    // Left button is released
    subject.update();
    assert_eq!(subject.did_release_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_release_mouse(MouseButton::Left), true);

    // Right button is clicked; left button is not clicked and not released recently
    subject.update();
    assert_eq!(subject.did_release_mouse(MouseButton::Right), false);
    assert_eq!(subject.did_release_mouse(MouseButton::Right), false);
}
