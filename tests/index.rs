use std::collections::HashMap;

use smart_house::SmartHouse;

use smart_house::devices::{Devices, SmartSocket, SmartSpeaker};
use smart_house::room::{Room, Rooms};

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
