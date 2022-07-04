use std::collections::BTreeMap;

use smart_house::devices::{Devices, SmartLamp, SmartSocket, SmartSpeaker, SmartThermometer};
use smart_house::room::Room;
use smart_house::SmartHouse;

fn main() {
    let kitchen = Room {
        devices: BTreeMap::from([
            ("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 })),
            (
                "Thermometer",
                Devices::SmartThermometer(SmartThermometer { temperature: 22 }),
            ),
            ("Speaker", Devices::SmartSpeaker(SmartSpeaker { volume: 3 })),
        ]),
    };

    let hall = Room {
        devices: BTreeMap::from([
            ("Socket", Devices::SmartSocket(SmartSocket { voltage: 220 })),
            ("Lamp", Devices::SmartLamp(SmartLamp { is_enabled: true })),
        ]),
    };

    let bedroom = Room {
        devices: BTreeMap::from([("Socket", Devices::SmartSocket(SmartSocket { voltage: 110 }))]),
    };

    let mut house = SmartHouse::new(
        "Smart house",
        BTreeMap::from([("Bedroom", bedroom), ("Kitchen", kitchen), ("Hall", hall)]),
    );

    house.add_room(
        "Living room",
        Room {
            devices: BTreeMap::from([(
                "Socket",
                Devices::SmartSocket(SmartSocket { voltage: 220 }),
            )]),
        },
    );

    house.remove_room("Kitchen");

    house.add_device(
        "Living room",
        "Smart Speaker",
        Devices::SmartSpeaker(SmartSpeaker { volume: 7 }),
    );

    house.remove_device("Hall", "Lamp");

    let report = house.create_report();
    println!("{}", report);

    let rooms_list = house.get_rooms_list();
    println!("{:?}", rooms_list);

    let devices_list = house.get_devices_list("Living room");
    println!("{:?}", devices_list);
}
