use std::net::TcpStream;

mod handle_host;
mod handle_guest;

use crate::config::{NetConfig, DeviceInfo};

pub fn handle_client(stream: TcpStream, cfg: &NetConfig, devices_info: &Vec<DeviceInfo>) {
    if cfg.is_host {
        crate::handle_client::handle_host::handle_client(stream, devices_info).expect("Couldn't write to stream")
    } else {
        crate::handle_client::handle_guest::handle_client(stream).expect("Couldn't read from stream")
    }
}