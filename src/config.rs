use std::env;
use std::path::Path;

pub struct DeviceInfo {
    pub path: String,
    pub device_type: String
}

pub struct NetConfig {
    pub is_server: bool,
    pub is_host: bool,
    pub port: i32,
    pub ip_address: String
}

pub fn get_config() -> (NetConfig, Vec<DeviceInfo>) {
    let args: Vec<String> = env::args().collect();
    
    let mut context = "";
    let mut cfg = NetConfig {
        is_server: false,
        is_host: true,
        port: 64654,
        ip_address: String::from("auto")
    };
    let mut devices_info = Vec::new();
    for i in &args[1..] {
        match i.as_str() {
            "-s" | "--server" => cfg.is_server = true,
            "-c" | "--client" => cfg.is_server = false,
            "-h" | "--host" => cfg.is_host = true,
            "-g" | "--guest" => cfg.is_host = false,
            "-p" | "--port" => context = "port",
            "-a" | "--ip-address" => context = "ip_address",
            "-d" | "--device" => {
                context = "device";
                devices_info.push(DeviceInfo {
                    path: String::from(""),
                    device_type: String::from("other")
                });
            },
            "--id" => context = "device_id",
            "--path" => context = "device_path",
            "--event" => context = "device_event",
            "--full-path" => context = "device_full_path",
            "--type" => context = "device_type",
            &_ => {
                if context.starts_with("device") {
                    let (key, tmp) = match context {
                        "device_id" => ("path", "/dev/input/by-id/"), 
                        "device_path" => ("path", "/dev/input/by-path/"),
                        "device_event" => ("path", "/dev/input/"),
                        "device_full_path" => ("path", ""),
                        "device_type" => {
                            match i.as_str() {
                                "pointer" | "keyboard" => ("type", ""),
                                &_ => panic!("Invalid device type {}!", i.as_str())
                            }
                        },
                        &_ => panic!("Unknown context!")
                    };
                    let mut value = tmp.to_owned();
                    let mut device_info = devices_info.last_mut().unwrap();
                    if key == "path" {
                        value = format!("{}{}", value, i);
                        if !Path::new(value.as_str()).exists() {
                            panic!("Path {} does not exist!", value)
                        }
                        device_info.path = value;
                    } else if key == "type" {
                        device_info.device_type = key.to_owned();
                    }
                } else {
                    match context {
                        "port" => {
                            match i.parse::<i32>() {
                                Ok(num) => cfg.port = num,
                                Err(_) => panic!("Invalid port value!")
                            }
                        },
                        "ip_address" => cfg.ip_address = i.to_owned(),
                        &_ => panic!("Invalid context!")
                    }
                }
            }, 
        }
    }
    if cfg.ip_address == "auto" {
        panic!("Not implemented yet!")
    }
    return (cfg, devices_info);
}