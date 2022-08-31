use std::collections::BTreeMap;

use client::devices::{Devices, SmartLamp, SmartSocket, SmartSpeaker, SmartThermometer};
use client::room::Room;
use client::SmartHouse;

#[actix_rt::main]
async fn main() {
    let kitchen = Room {
        devices: BTreeMap::from([
            (
                "socket".to_string(),
                Devices::SmartSocket(SmartSocket { voltage: 110 }),
            ),
            (
                "thermometer".to_string(),
                Devices::SmartThermometer(SmartThermometer { temperature: 22 }),
            ),
            (
                "speaker".to_string(),
                Devices::SmartSpeaker(SmartSpeaker { volume: 3 }),
            ),
        ]),
    };

    let hall = Room {
        devices: BTreeMap::from([
            (
                "socket".to_string(),
                Devices::SmartSocket(SmartSocket { voltage: 220 }),
            ),
            (
                "lamp".to_string(),
                Devices::SmartLamp(SmartLamp { is_enabled: true }),
            ),
        ]),
    };

    let bedroom = Room {
        devices: BTreeMap::from([(
            "socket".to_string(),
            Devices::SmartSocket(SmartSocket { voltage: 110 }),
        )]),
    };

    let mut house = SmartHouse::new(
        "smart_house",
        BTreeMap::from([
            ("bedroom".to_string(), bedroom),
            ("kitchen".to_string(), kitchen),
            ("hall".to_string(), hall),
        ]),
    )
    .await;

    house
        .add_room(
            "living_room",
            Room {
                devices: BTreeMap::from([(
                    "socket".to_string(),
                    Devices::SmartSocket(SmartSocket { voltage: 220 }),
                )]),
            },
        )
        .await;

    house.remove_room("kitchen").await;

    house
        .add_device(
            "living_room",
            "smart_speaker",
            Devices::SmartSpeaker(SmartSpeaker { volume: 7 }),
        )
        .await;

    house.remove_device("hall", "lamp").await;

    let report = house.create_report();
    println!("{}", report);

    let rooms_list = house.get_rooms_list();
    println!("{:?}", rooms_list);

    let devices_list = house.get_devices_list("living_room");
    println!("{:?}", devices_list);

    // SmartHouse::delete("smart_house").await;
}
