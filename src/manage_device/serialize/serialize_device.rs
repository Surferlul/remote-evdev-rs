use ron::ser::to_string;
use evdev::Device;

fn serialize_name(device: &Device) -> String {
    to_string(
        device.name().unwrap()
    ).expect("Error serializing name!")
}

fn serialize_input_id(device: &Device) -> String {
    let input_id = device.input_id();
    to_string(
        &[
            input_id.bus_type().0,
            input_id.vendor(),
            input_id.product(),
            input_id.version()
        ]
    ).expect("Error serializing InputId!")
}

fn serialize_supported_keys(device: &Device) -> String {
    to_string(
        &device.supported_keys().unwrap().iter().map(|x| x.0).collect::<Vec<u16>>()
    ).expect("Error serializing supported_keys!")
}

fn serialize_supported_relative_axes(device: &Device) -> String {
    to_string::<Option<Vec<u16>>>(
        &match device.supported_relative_axes() {
            Some(relative_axes) => Some(
                relative_axes.iter().map(|x| x.0).collect::<Vec<u16>>()
            ),
            None => None
        }
    ).expect("Error serializing supported_relative_axes!")
}

fn serialize_supported_switches(device: &Device) -> String {
    to_string::<Option<Vec<u16>>>(
        &match device.supported_switches() {
            Some(switches) => Some(
                switches.iter().map(|x| x.0).collect::<Vec<u16>>()
            ),
            None => None
        }
    ).expect("Error serializing supported_switches!")
}

pub fn serialize(device: &Device) -> String {
    to_string(
        &[
            serialize_name(device),
            serialize_input_id(device),
            serialize_supported_keys(device),
            serialize_supported_relative_axes(device),
            serialize_supported_switches(device)
        ]
    ).expect("Error serializing device!")
}