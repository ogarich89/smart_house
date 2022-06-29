use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub mod devices;
pub mod room;

use crate::devices::{DeviceInfoProvider, Devices};
use crate::room::{Room, Rooms};

pub struct SmartHouse {
    pub name: &'static str,
    rooms: HashMap<Rooms, Room>,
}

impl DeviceInfoProvider for SmartHouse {
    fn get_device_status(&self, room: &Rooms, name: &'static str) -> String {
        let result = self.get_device(room, name);
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
    pub fn new(name: &'static str, rooms: HashMap<Rooms, Room>) -> Self {
        SmartHouse { name, rooms }
    }

    fn get_room(&self, room: &Rooms) -> Option<&Room> {
        if self.rooms.contains_key(room) {
            Some(&self.rooms[room])
        } else {
            None
        }
    }

    pub fn get_rooms(&self) -> Keys<Rooms, Room> {
        self.rooms.keys()
    }

    fn get_device(&self, room: &Rooms, name: &'static str) -> Result<&Devices, String> {
        let result = self.get_room(room);
        match result {
            Some(room) => {
                if !room.devices.contains_key(name) {
                    Err(format!("Device '{}' not found!", name))
                } else {
                    Ok(&room.devices[name])
                }
            }
            None => Err(format!("The room '{:?}' is not exists!", room)),
        }
    }

    pub fn get_devices(&self, room: &Rooms) -> Keys<&'static str, Devices> {
        self.rooms[room].devices.keys()
    }

    pub fn create_report(&self) -> String {
        let mut report = format!("{} report: \n\r", &self.name);
        let rooms = self.get_rooms();
        rooms.for_each(|room| {
            report.push_str(&format!(" • {:?}:\n\r", room));
            let devices = self.get_devices(room);
            devices.enumerate().for_each(|(index, device)| {
                report.push_str(&format!(
                    "   {}) {}: {}\n\r",
                    index + 1,
                    device,
                    self.get_device_status(room, device)
                ))
            })
        });
        report
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::devices::{Devices, SmartSocket, SmartSpeaker};
    use crate::room::{Room, Rooms};
    use crate::{DeviceInfoProvider, SmartHouse};

    fn get_house() -> SmartHouse {
        let kitchen = Room {
            devices: HashMap::from([
                ("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 })),
                ("Speaker", Devices::SmartSpeaker(SmartSpeaker { volume: 3 })),
            ]),
        };
        SmartHouse::new(
            "Test smart house",
            HashMap::from([(Rooms::Kitchen, kitchen)]),
        )
    }

    #[test]
    fn it_should_return_device_status() {
        let house = get_house();
        assert_eq!(
            house.get_device_status(&Rooms::Kitchen, "Socket"),
            "voltage 110"
        );
    }
    #[test]
    fn it_should_return_message_device_not_found() {
        let house = get_house();
        assert_eq!(
            house.get_device_status(&Rooms::Kitchen, "Lamp"),
            "Device 'Lamp' not found!"
        );
    }
}
