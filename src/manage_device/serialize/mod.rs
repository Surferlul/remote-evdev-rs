use evdev::InputEvent;

pub mod serialize_device;

pub fn serialize_input_event(event: InputEvent) -> (u16, u16, i32) {
    (event.event_type().0, event.code(), event.value())
}