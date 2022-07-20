use evdev::{Device, InputEvent};

use crate::manage_device::set_unblocking;

pub struct FetchDevicesEventsSynced<'a> {
    collection: &'a mut DeviceCollection
}

impl <'a>Iterator for FetchDevicesEventsSynced<'a> {

    type Item = Vec<(usize, InputEvent)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut events = Vec::new();
        for device_id in 0..self.collection.len() {
            match self.collection[device_id].fetch_events() {
                Ok(events_fetch) => {
                    for event in events_fetch {
                        events.push((device_id, event))
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return None
                }
            }
        };
        Some(events)
    }
}

pub struct DeviceCollection {
    pub devices: Vec<Device>
}

impl DeviceCollection {
    pub fn new() -> DeviceCollection {
        DeviceCollection {
            devices: Vec::new()
        }
    }

    pub fn push(&mut self, mut device: Device) {
        set_unblocking(&mut device);
        device.grab().expect("Couldn't grab device!");
        self.devices.push(device);
    }

    pub fn len(&mut self) -> usize {
        self.devices.len()
    }

    pub fn fetch_events(&mut self) -> FetchDevicesEventsSynced {
        FetchDevicesEventsSynced {collection: self}
    }
}

impl IntoIterator for DeviceCollection {
    type Item = Device;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.devices.into_iter()
    }
}

impl std::ops::Index<usize> for DeviceCollection {
    type Output = Device;

    fn index(&self, index: usize) -> &Self::Output {
        &self.devices[index]
    }
}

impl std::ops::IndexMut<usize> for DeviceCollection {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.devices[index]
    }
}