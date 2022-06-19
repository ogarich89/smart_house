use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub mod devices;
pub mod room;

use crate::devices::{DeviceInfoProvider, Devices};
use crate::room::{Room, Rooms};

pub struct SmartHouse {
    name: &'static str,
    rooms: HashMap<Rooms, Room>,
}

impl DeviceInfoProvider for SmartHouse {
    fn get_device_status(&self, room: &Rooms, name: &'static str) -> String {
        if !self.rooms[room].devices.contains_key(name) {
            String::from("not found")
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
    use crate::SmartHouse;

    fn get_house() -> SmartHouse {
        let kitchen = Room {
            devices: HashMap::from([
                ("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 })),
                ("Speaker", Devices::SmartSpeaker(SmartSpeaker { volume: 3 })),
            ]),
        };
        return SmartHouse::new(
            "Test smart house",
            HashMap::from([(Rooms::Kitchen, kitchen)]),
        );
    }

    #[test]
    fn it_should_return_name() {
        let house = get_house();
        assert_eq!(house.name, "Test smart house");
    }

    #[test]
    fn it_should_return_rooms_list() {
        let house = get_house();
        assert_eq!(
            format!("{:?}", house.get_rooms()),
            format!("{:?}", [Rooms::Kitchen])
        );
    }

    #[test]
    fn it_should_return_devices_list() {
        let house = get_house();
        assert!(format!("{:?}", house.get_devices(&Rooms::Kitchen)).contains("Speaker"));
        assert!(format!("{:?}", house.get_devices(&Rooms::Kitchen)).contains("Socket"));
    }

    #[test]
    fn it_should_return_report() {
        let house = get_house();
        assert!(house
            .create_report()
            .contains("Test smart house report: \n\r"));
    }
}
