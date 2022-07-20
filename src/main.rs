#![feature(generic_associated_types)]

mod config;
mod epoll_struct;
mod streams;
mod handle_client;
mod manage_device;

use crate::{
    config::{
        get_config,
    },
    streams::get_streams,
    handle_client::handle_client,
};


fn main() {
    let (cfg, devices_info) = get_config();
    let ip = format!("{}:{}", cfg.ip_address, cfg.port);
    for stream in get_streams(cfg.is_server, ip) {
        handle_client(stream, &cfg, &devices_info);
    }
}