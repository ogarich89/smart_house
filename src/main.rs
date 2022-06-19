use std::collections::hash_map::Keys;
use std::collections::HashMap;

struct SmartHouse {
    name: &'static str,
    rooms: HashMap<Rooms, Room>,
}

struct Room {
    devices: HashMap<&'static str, Devices>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Rooms {
    Hall,
    Kitchen,
    Bedroom,
}

enum Devices {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
    SmartSpeaker(SmartSpeaker),
    SmartLamp(SmartLamp),
}

impl SmartHouse {
    fn new(name: &'static str, rooms: HashMap<Rooms, Room>) -> Self {
        SmartHouse { name, rooms }
    }

    fn get_rooms(&self) -> Keys<Rooms, Room> {
        self.rooms.keys()
    }

    fn get_devices(&self, room: &Rooms) -> Keys<&'static str, Devices> {
        self.rooms[room].devices.keys()
    }

    fn create_report(&self) -> String {
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

trait DeviceInfoProvider {
    fn get_device_status(&self, room: &Rooms, name: &'static str) -> String;
}

struct SmartSocket {
    voltage: i32,
}

struct SmartThermometer {
    temperature: i32,
}

struct SmartSpeaker {
    volume: i32,
}

struct SmartLamp {
    is_enabled: bool,
}

impl DeviceInfoProvider for SmartHouse {
    fn get_device_status(&self, room: &Rooms, name: &'static str) -> String {
        if !self.rooms[room].devices.contains_key(name) {
            format!("not found")
        } else {
            match &self.rooms[room].devices[name] {
                Devices::SmartLamp(lamp) => format!(
                    "{}",
                    if lamp.is_enabled {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ),
                Devices::SmartSocket(socket) => {
                    format!("{}", "voltage ".to_owned() + &socket.voltage.to_string())
                }
                Devices::SmartSpeaker(speaker) => {
                    format!("{}", "volume ".to_owned() + &speaker.volume.to_string())
                }
                Devices::SmartThermometer(thermometer) => format!(
                    "{}",
                    "temperature ".to_owned() + &thermometer.temperature.to_string()
                ),
            }
        }
    }
}

fn main() {
    let kitchen = Room {
        devices: HashMap::from([
            ("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 })),
            (
                "Thermometer",
                Devices::SmartThermometer(SmartThermometer { temperature: 22 }),
            ),
            ("Speaker", Devices::SmartSpeaker(SmartSpeaker { volume: 3 })),
        ]),
    };

    let hall = Room {
        devices: HashMap::from([
            ("Socket", Devices::SmartSocket(SmartSocket { voltage: 220 })),
            ("Lamp", Devices::SmartLamp(SmartLamp { is_enabled: true })),
        ]),
    };

    let bedroom = Room {
        devices: HashMap::from([("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 }))]),
    };

    let house = SmartHouse::new(
        "Smart house",
        HashMap::from([
            (Rooms::Bedroom, bedroom),
            (Rooms::Kitchen, kitchen),
            (Rooms::Hall, hall),
        ]),
    );

    let report = house.create_report();
    println!("{}", report)
}
