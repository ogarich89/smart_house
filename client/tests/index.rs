use std::collections::BTreeMap;

use client::SmartHouse;

use client::devices::{Devices, SmartLamp, SmartSocket, SmartSpeaker};
use client::room::Room;

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
async fn it_should_return_name() {
    let house = get_house().await;
    assert_eq!(house._id, "test_smart_house");
}

#[actix_rt::test]
async fn it_should_return_rooms_list() {
    let house = get_house().await;
    assert_eq!(
        format!("{:?}", house.get_rooms_list()),
        format!("{:?}", ["kitchen"])
    );
}

#[actix_rt::test]
async fn it_should_return_devices_list() {
    let house = get_house().await;
    assert_eq!(
        format!("{:?}", house.get_devices_list("kitchen")),
        format!("{:?}", ["socket", "speaker"])
    );
}

#[actix_rt::test]
async fn it_should_return_report() {
    let house = get_house().await;
    assert!(house
        .create_report()
        .contains("test_smart_house report: \n\r"));
}

#[actix_rt::test]
async fn it_should_add_room() {
    let mut house = get_house().await;
    let hall = Room {
        devices: BTreeMap::from([]),
    };
    house.add_room("hall", hall).await;
    assert_eq!(
        format!("{:?}", house.get_rooms_list()),
        format!("{:?}", ["hall", "kitchen"])
    );
}
#[actix_rt::test]
async fn it_should_remove_room() {
    let mut house = get_house().await;

    house.remove_room("kitchen").await;
    assert_eq!(format!("{:?}", house.get_rooms_list()), format!("[]"));
}
#[actix_rt::test]
async fn it_should_add_device() {
    let mut house = get_house().await;

    house
        .add_device(
            "kitchen",
            "lamp",
            Devices::SmartLamp(SmartLamp { is_enabled: true }),
        )
        .await;
    assert_eq!(
        format!("{:?}", house.get_devices_list("kitchen")),
        format!("{:?}", ["lamp", "socket", "speaker"])
    );
}
#[actix_rt::test]
async fn it_should_remove_device() {
    let mut house = get_house().await;
    house.remove_device("kitchen", "speaker").await;
    assert_eq!(
        format!("{:?}", house.get_devices_list("kitchen")),
        format!("{:?}", ["socket"])
    );
}
