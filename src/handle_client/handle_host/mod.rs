use std::io::prelude::*;
use std::net::TcpStream;
use evdev::{
    Device,
    InputEvent
};
use byteorder::{BigEndian, WriteBytesExt};

mod device_collection;

use crate::{
    config::DeviceInfo,
    manage_device::serialize::{
        serialize_device::serialize,
        serialize_input_event
    },
    handle_client::handle_host::device_collection::DeviceCollection
};

fn send_device(mut stream: &TcpStream, device: &mut Device) -> std::io::Result<()>{
    let buf = serialize(&device);
    let buf_len = buf.as_bytes().len() as u64;
    stream.write_u64::<BigEndian>(buf_len)?;
    stream.write(buf.as_bytes())?;
    Ok(())
}

fn send_input_event(mut stream: &TcpStream, device_id: usize, input_event: InputEvent) -> std::io::Result<()> {
    let data = serialize_input_event(input_event);
    stream.write_u64::<BigEndian>(device_id as u64)?;
    stream.write_u16::<BigEndian>(data.0)?;
    stream.write_u16::<BigEndian>(data.1)?;
    stream.write_i32::<BigEndian>(data.2)?;
    Ok(())
}

pub fn handle_client(mut stream: TcpStream, devices_info: &Vec<DeviceInfo>) -> std::io::Result<()>{
    let mut collection = DeviceCollection::new();
    for device_info in devices_info {
        match Device::open(&device_info.path) {
            Ok(device) => collection.push(device),
            Err(_) => panic!("Couldn't open device {}", device_info.path)
        }
    }
    for device in collection.devices.iter_mut() {
        send_device(&stream, device)?;
    }
    stream.write_u64::<BigEndian>(0)?;
    for events in collection.fetch_events() {
        for (device_id, event) in events {
            send_input_event(&stream, device_id, event)?;
        }
    }
    stream.write_u64::<BigEndian>(u32::MAX as u64)?;
    stream.write_u16::<BigEndian>(0)?;
    stream.write_u16::<BigEndian>(0)?;
    stream.write_i32::<BigEndian>(0)?;
    for device in collection.devices.iter_mut() {
        device.ungrab().expect("Couldn't ungrab device {}!");
    }
    Ok(())
}