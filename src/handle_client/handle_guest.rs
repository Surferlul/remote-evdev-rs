use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, ReadBytesExt};
use evdev::{
    InputEvent,
    uinput::VirtualDevice
};

use crate::{
    manage_device::deserialize::{
        deserialize_device::build_virtual_device,
        deserialize_input_event
    }
};

fn receive_device(mut stream: &TcpStream) -> std::io::Result<Option<VirtualDevice>> {
    let buf_size = stream.read_u64::<BigEndian>()?;
    if buf_size == 0 {
        return Ok(None)
    };
    let mut buf = vec![0; buf_size as usize];
    stream.read_exact(&mut buf)?;
    let data = String::from_utf8(buf).expect("Couldn't decode utf8");
    Ok(Some(build_virtual_device(data)))
}

fn receive_input_event(mut stream: &TcpStream) -> std::io::Result<(usize, InputEvent)> {
    Ok(
        (
            stream.read_u64::<BigEndian>()? as usize,
            deserialize_input_event(
                (
                    stream.read_u16::<BigEndian>()?,
                    stream.read_u16::<BigEndian>()?,
                    stream.read_i32::<BigEndian>()?
                )
            )
        )
    )
}

pub fn handle_client(stream: TcpStream) -> std::io::Result<()>{
    let mut devices = Vec::new();
    while let Some(device) = receive_device(&stream)? {
        devices.push(device);
    }
    loop {
        let (device_id, input_event) = receive_input_event(&stream)?;
        if device_id as u64 == u32::MAX as u64 {
            break
        }
        devices[device_id].emit_raw(&[input_event]).expect("Couldn't emit input event");
    }
    Ok(())
}