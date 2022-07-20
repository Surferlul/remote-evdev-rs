use std::ops::DerefMut;
use ron::de::from_str;
use evdev::{
    Key,
    InputId,
    SwitchType,
    RelativeAxisType,
    BusType,
    AttributeSet,
    AttributeSetRef,
    uinput::{
        VirtualDevice,
        VirtualDeviceBuilder
    }
};


fn deserialize_name(data: &str) -> String {
    from_str(data).expect("Couldn't deserialize name!")
}

fn deserialize_input_id(data: &str) -> InputId {
    let vals: [u16; 4] = from_str(data).expect("Couldn't deserialize InputId!");
    InputId::new(BusType(vals[0]), vals[1], vals[2], vals[3])
}

fn deserialize_supported_keys(data: &str, keys: &mut AttributeSetRef<Key>) {
    let vals: Vec<u16> = from_str(data).expect("Couldn't deserialize supported keys!");
    for key in vals.iter().map(|x| Key(*x)) {
        keys.insert(key);
    }
}

fn deserialize_supported_relative_axes(data: &str, relative_axes: &mut AttributeSetRef<RelativeAxisType>) -> Result<(), &'static str> {
    match from_str::<Option<Vec<u16>>>(data).expect("Couldn't deserialize relative axes") {
        Some(vals) => {
            for rel_axes_type in vals.iter().map(|x| RelativeAxisType(*x)) {
                relative_axes.insert(rel_axes_type);
            }
            Ok(())
        },
        None => Err("No relative axes support"),
    }
}

fn deserialize_supported_switches(data: &str, switches: &mut AttributeSetRef<SwitchType>) -> Result<(), &'static str> {
    match from_str::<Option<Vec<u16>>>(data).expect("Couldn't deserialize relative axes") {
        Some(vals) => {
            for switch_type in vals.iter().map(|x| SwitchType(*x)) {
                switches.insert(switch_type);
            }
            Ok(())
        },
        None => Err("No switch support"),
    }
}

fn deserialize(
    data: String,
    keys: &mut AttributeSetRef<Key>,
    relative_axes: &mut AttributeSetRef<RelativeAxisType>,
    switches: &mut AttributeSetRef<SwitchType>
) -> (String, InputId, Result<(), &'static str>, Result<(), &'static str>) {
    let de_data: [String; 5] = from_str(data.as_str()).expect("Couldn't deserialize device!");
    let name = deserialize_name(de_data[0].as_str());
    let input_id = deserialize_input_id(de_data[1].as_str());
    deserialize_supported_keys(de_data[2].as_str(), keys);
    (
        name,
        input_id,
        deserialize_supported_relative_axes(de_data[3].as_str(), relative_axes),
        deserialize_supported_switches(de_data[4].as_str(), switches)
    )
}

pub fn build_virtual_device(data: String) -> VirtualDevice {
    let mut keys: AttributeSet<Key> = AttributeSet::new();
    let mut relative_axes: AttributeSet<RelativeAxisType> = AttributeSet::new();
    let mut switches: AttributeSet<SwitchType> = AttributeSet::new();
    let (name, input_id, relative_axes_support, switch_support) = deserialize(data, keys.deref_mut(), relative_axes.deref_mut(), switches.deref_mut());
    let mut uinput_builder = VirtualDeviceBuilder::new().expect("Couldn't create new virtual device builder!")
        .name(name.as_str())
        .input_id(input_id)
        .with_keys(keys.deref_mut()).expect("Couldn't build virtual device with keys!");
    match relative_axes_support {
        Ok(_) => {},
        Err(_) => uinput_builder = uinput_builder.with_relative_axes(relative_axes.deref_mut()).expect("Couldn't build virtual device with relative axes!")
    }
    match switch_support {
        Ok(_) => {},
        Err(_) => uinput_builder = uinput_builder.with_switches(switches.deref_mut()).expect("Couldn't build virtual device with switches!")
    }    
    uinput_builder.build().expect("Couldn't build virtual device!")
}