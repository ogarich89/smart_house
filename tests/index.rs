use std::collections::BTreeMap;

use smart_house::SmartHouse;

use smart_house::devices::{Devices, SmartLamp, SmartSocket, SmartSpeaker};
use smart_house::room::Room;

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
fn it_should_return_name() {
    let house = get_house();
    assert_eq!(house.name, "Test smart house");
}

#[test]
fn it_should_return_rooms_list() {
    let house = get_house();
    assert_eq!(
        format!("{:?}", house.get_rooms_list()),
        format!("{:?}", ["Kitchen"])
    );
}

#[test]
fn it_should_return_devices_list() {
    let house = get_house();
    assert_eq!(
        format!("{:?}", house.get_devices_list("Kitchen")),
        format!("{:?}", ["Socket", "Speaker"])
    );
}

#[test]
fn it_should_return_report() {
    let house = get_house();
    assert!(house
        .create_report()
        .contains("Test smart house report: \n\r"));
}

#[test]
fn it_should_add_room() {
    let mut house = get_house();
    let hall = Room {
        devices: BTreeMap::from([]),
    };
    house.add_room("Hall", hall);
    assert_eq!(
        format!("{:?}", house.get_rooms_list()),
        format!("{:?}", ["Hall", "Kitchen"])
    );
}
#[test]
fn it_should_remove_room() {
    let mut house = get_house();

    house.remove_room("Kitchen");
    assert_eq!(format!("{:?}", house.get_rooms_list()), format!("[]"));
}
#[test]
fn it_should_add_device() {
    let mut house = get_house();

    house.add_device(
        "Kitchen",
        "Lamp",
        Devices::SmartLamp(SmartLamp { is_enabled: true }),
    );
    assert_eq!(
        format!("{:?}", house.get_devices_list("Kitchen")),
        format!("{:?}", ["Lamp", "Socket", "Speaker"])
    );
}
#[test]
fn it_should_remove_device() {
    let mut house = get_house();
    house.remove_device("Kitchen", "Speaker");
    assert_eq!(
        format!("{:?}", house.get_devices_list("Kitchen")),
        format!("{:?}", ["Socket"])
    );
}
