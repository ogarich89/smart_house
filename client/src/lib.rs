pub mod devices;
pub mod room;

use crate::devices::{DeviceInfoProvider, Devices};
use crate::room::Room;
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::btree_map::Keys;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SmartHouse {
    pub _id: String,
    rooms: BTreeMap<String, Room>,
}

impl DeviceInfoProvider for SmartHouse {
    fn get_device_status(&self, room_name: &str, name: &str) -> String {
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

const API: &str = "http://localhost:8080";

impl SmartHouse {
    pub async fn new(name: &'static str, rooms: BTreeMap<String, Room>) -> SmartHouse {
        let client = Client::default();
        let house = SmartHouse {
            _id: name.to_string(),
            rooms,
        };

        let result = client
            .post(format!("{}{}", API, "/houses"))
            .send_json(&json!(house))
            .await;

        match result {
            Ok(_) => house,
            Err(_err) => panic!("Failed to initialize smart house!"),
        }
    }

    pub async fn delete(name: &'static str) -> String {
        let client = Client::default();

        let result = client
            .delete(format!("{}{}{}", API, "/houses/", name))
            .send()
            .await;

        match result {
            Ok(_) => format!("Smart house {} successfully deleted!", name),
            Err(_err) => panic!("Failed to initialize smart house!"),
        }
    }

    pub async fn add_room(&mut self, name: &'static str, room: Room) -> Option<Room> {
        if self.rooms.contains_key(name) {
            println!("The name of the room must be unique!");
            None
        } else {
            let client = Client::default();
            let result = client
                .post(format!(
                    "{}{}{}{}{}",
                    API, "/houses/", &*self._id, "/rooms/", name
                ))
                .send_json(&json!(room))
                .await;
            match result {
                Ok(_) => self.rooms.insert(name.to_string(), room),
                Err(_err) => panic!("Failed to add room!"),
            }
        }
    }

    pub async fn remove_room(&mut self, name: &'static str) -> Option<Room> {
        if !self.rooms.contains_key(name) {
            println!("The room {} not found!", name);
            None
        } else {
            let client = Client::default();
            let result = client
                .delete(format!(
                    "{}{}{}{}{}",
                    API, "/houses/", &*self._id, "/rooms/", name
                ))
                .send()
                .await;
            match result {
                Ok(_) => self.rooms.remove(name),
                Err(_err) => panic!("Failed to remove room!"),
            }
        }
    }

    pub async fn add_device(
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
                    let client = Client::default();
                    let result = client
                        .post(format!(
                            "{}{}{}{}{}{}{}",
                            API, "/houses/", &*self._id, "/rooms/", room_name, "/devices/", name
                        ))
                        .send_json(&json!(device))
                        .await;
                    match result {
                        Ok(_) => room.devices.insert(name.to_string(), device),
                        Err(_err) => panic!("Failed to add device!"),
                    }
                }
            }
            None => {
                println!("The room {} is not exists!", room_name);
                None
            }
        }
    }

    pub async fn remove_device(
        &mut self,
        room_name: &'static str,
        name: &'static str,
    ) -> Option<Devices> {
        match self.rooms.get_mut(room_name) {
            Some(room) => {
                if room.devices.contains_key(name) {
                    let client = Client::default();
                    let result = client
                        .delete(format!(
                            "{}{}{}{}{}{}{}",
                            API, "/houses/", &*self._id, "/rooms/", room_name, "/devices/", name
                        ))
                        .send()
                        .await;
                    match result {
                        Ok(_) => room.devices.remove(name),
                        Err(_err) => panic!("Failed to remove device!"),
                    }
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

    pub fn get_rooms_list(&self) -> Keys<String, Room> {
        self.rooms.keys()
    }

    pub fn get_devices_list(&self, room_name: &str) -> Keys<String, Devices> {
        self.rooms[room_name].devices.keys()
    }

    pub fn create_report(&self) -> String {
        let mut report = format!("{} report: \n\r", &self._id);
        let rooms_list = self.get_rooms_list();
        rooms_list.for_each(|room_name| {
            report.push_str(&*format!(" • {:?}:\n\r", room_name));
            let devices_list = self.get_devices_list(room_name);
            devices_list.enumerate().for_each(|(index, device_name)| {
                report.push_str(&*format!(
                    "   {}) {}: {}\n\r",
                    index + 1,
                    device_name,
                    self.get_device_status(room_name, device_name)
                ))
            })
        });
        report
    }

    fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    fn get_device(&self, room_name: &str, name: &str) -> Result<&Devices, String> {
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
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::devices::{Devices, SmartSocket, SmartSpeaker};
    use crate::room::Room;
    use crate::{DeviceInfoProvider, SmartHouse};

    async fn get_house() -> SmartHouse {
        let kitchen = Room {
            devices: BTreeMap::from([
                (
                    "socket".to_string(),
                    Devices::SmartSocket(SmartSocket { voltage: 110 }),
                ),
                (
                    "speaker".to_string(),
                    Devices::SmartSpeaker(SmartSpeaker { volume: 3 }),
                ),
            ]),
        };
        SmartHouse::new(
            "test_smart_house",
            BTreeMap::from([("kitchen".to_string(), kitchen)]),
        )
        .await
    }

    #[actix_rt::test]
    async fn it_should_return_device_status() {
        let house = get_house().await;
        assert_eq!(house.get_device_status("kitchen", "socket"), "voltage 110");
    }
    #[actix_rt::test]
    async fn it_should_return_message_device_not_found() {
        let house = get_house().await;
        assert_eq!(
            house.get_device_status("kitchen", "lamp"),
            "Device 'lamp' not found!"
        );
    }
}
