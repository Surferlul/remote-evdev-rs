use evdev::{
    InputEvent,
    EventType
};

pub mod deserialize_device;

pub fn deserialize_input_event(data: (u16, u16, i32)) -> InputEvent {
    InputEvent::new(EventType(data.0), data.1, data.2)
}