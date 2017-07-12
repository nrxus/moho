mod state;

pub use self::state::State;

use sdl2::EventPump as SdlEventPump;
use sdl2::event::Event;

struct EventGenerator<E> {
    event_pump: E,
}

impl<E> EventGenerator<E> {
    fn new(event_pump: E) -> Self {
        EventGenerator {
            event_pump: event_pump,
        }
    }

    fn iter(&mut self) -> EventIterator<E> {
        EventIterator {
            event_pump: &mut self.event_pump,
        }
    }
}

struct EventIterator<'a, E: 'a> {
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

pub struct Manager<P> {
    pub current: State,
    event_generator: EventGenerator<P>,
}

impl<P: EventPump> Manager<P> {
    pub fn new(event_pump: P) -> Manager<P> {
        Manager {
            current: State::default(),
            event_generator: EventGenerator::new(event_pump),
        }
    }

    pub fn update(&mut self) -> &State {
        self.current.update(&mut self.event_generator);
        &self.current
    }
}
