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
        if !self.rooms[room].devices.contains_key(name) {
            name.to_string() + " not found"
        } else {
            match &self.rooms[room].devices[name] {
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
            }
        }
    }
}

impl SmartHouse {
    pub fn new(name: &'static str, rooms: HashMap<Rooms, Room>) -> Self {
        SmartHouse { name, rooms }
    }

    pub fn get_rooms(&self) -> Keys<Rooms, Room> {
        self.rooms.keys()
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
            "Lamp not found"
        );
    }
}
