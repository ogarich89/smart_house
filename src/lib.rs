use std::collections::btree_map::Keys;
use std::collections::BTreeMap;

pub mod devices;
pub mod room;

use crate::devices::{DeviceInfoProvider, Devices};
use crate::room::Room;

pub struct SmartHouse {
    pub name: &'static str,
    rooms: BTreeMap<&'static str, Room>,
}

impl DeviceInfoProvider for SmartHouse {
    fn get_device_status(&self, room_name: &'static str, name: &'static str) -> String {
        let result = self.get_device(room_name, name);
        match result {
            Ok(device) => match device {
                Devices::SmartLamp(lamp) => {
                    if lamp.is_enabled {
                        String::from("enabled")
                    } else {
                        String::from("disabled")
                    }
                }
                Devices::SmartSocket(socket) => "voltage ".to_owned() + &socket.voltage.to_string(),
                Devices::SmartSpeaker(speaker) => {
                    "volume ".to_owned() + &speaker.volume.to_string()
                }
                Devices::SmartThermometer(thermometer) => {
                    "temperature ".to_owned() + &thermometer.temperature.to_string()
                }
            },
            Err(error) => error,
        }
    }
}

impl SmartHouse {
    pub fn new(name: &'static str, rooms: BTreeMap<&'static str, Room>) -> Self {
        SmartHouse { name, rooms }
    }

    pub fn add_room(&mut self, name: &'static str, room: Room) -> Option<Room> {
        if self.rooms.contains_key(name) {
            println!("The name of the room must be unique!");
            None
        } else {
            self.rooms.insert(name, room)
        }
    }

    pub fn remove_room(&mut self, name: &'static str) -> Option<Room> {
        if !self.rooms.contains_key(name) {
            println!("The room {} not found!", name);
            None
        } else {
            self.rooms.remove(name)
        }
    }

    fn get_room(&self, name: &'static str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn add_device(
        &mut self,
        room_name: &'static str,
        name: &'static str,
        device: Devices,
    ) -> Option<Devices> {
        match self.rooms.get_mut(room_name) {
            Some(room) => {
                if room.devices.contains_key(name) {
                    println!("The name of the device must be unique!");
                    None
                } else {
                    room.devices.insert(name, device)
                }
            }
            None => {
                println!("The room {} is not exists!", room_name);
                None
            }
        }
    }

    pub fn remove_device(
        &mut self,
        room_name: &'static str,
        name: &'static str,
    ) -> Option<Devices> {
        match self.rooms.get_mut(room_name) {
            Some(room) => {
                if room.devices.contains_key(name) {
                    room.devices.remove(name)
                } else {
                    println!("Device '{}' not found!", name);
                    None
                }
            }
            None => {
                println!("The room {} is not exists!", room_name);
                None
            }
        }
    }

    fn get_device(&self, room_name: &'static str, name: &'static str) -> Result<&Devices, String> {
        let result = self.get_room(room_name);
        match result {
            Some(room) => {
                if !room.devices.contains_key(name) {
                    Err(format!("Device '{}' not found!", name))
                } else {
                    Ok(&room.devices[name])
                }
            }
            None => Err(format!("The room {} is not exists!", room_name)),
        }
    }

    pub fn get_rooms_list(&self) -> Keys<&'static str, Room> {
        self.rooms.keys()
    }

    pub fn get_devices_list(&self, room_name: &'static str) -> Keys<&'static str, Devices> {
        self.rooms[room_name].devices.keys()
    }

    pub fn create_report(&self) -> String {
        let mut report = format!("{} report: \n\r", &self.name);
        let rooms_list = self.get_rooms_list();
        rooms_list.for_each(|room_name| {
            report.push_str(&format!(" • {:?}:\n\r", room_name));
            let devices_list = self.get_devices_list(room_name);
            devices_list.enumerate().for_each(|(index, device_name)| {
                report.push_str(&format!(
                    "   {}) {}: {}\n\r",
                    index + 1,
                    device_name,
                    self.get_device_status(room_name, device_name)
                ))
            })
        });
        report
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::devices::{Devices, SmartSocket, SmartSpeaker};
    use crate::room::Room;
    use crate::{DeviceInfoProvider, SmartHouse};

    fn get_house() -> SmartHouse {
        let kitchen = Room {
            devices: BTreeMap::from([
                ("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 })),
                ("Speaker", Devices::SmartSpeaker(SmartSpeaker { volume: 3 })),
            ]),
        };
        SmartHouse::new("Test smart house", BTreeMap::from([("Kitchen", kitchen)]))
    }

    #[test]
    fn it_should_return_device_status() {
        let house = get_house();
        assert_eq!(house.get_device_status("Kitchen", "Socket"), "voltage 110");
    }
    #[test]
    fn it_should_return_message_device_not_found() {
        let house = get_house();
        assert_eq!(
            house.get_device_status("Kitchen", "Lamp"),
            "Device 'Lamp' not found!"
        );
    }
}
