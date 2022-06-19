use std::collections::HashMap;

use smart_house::devices::{Devices, SmartLamp, SmartSocket, SmartSpeaker, SmartThermometer};
use smart_house::room::{Room, Rooms};
use smart_house::SmartHouse;

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
