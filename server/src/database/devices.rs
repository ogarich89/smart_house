use actix_web::web;
use client::devices::Devices;
use client::SmartHouse;
use mongodb::{
    bson::{doc, to_document},
    Database,
};
pub async fn create(
    id: String,
    room_id: String,
    device_id: String,
    device: web::Json<Devices>,
    mongo: &Database,
) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .update_one(
            doc! { "_id": id },
            doc! { "$set": { format!("rooms.{}.devices.{}", room_id, device_id): to_document(&device.into_inner()).unwrap() }},
            None,
        )
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}

pub async fn remove(
    id: String,
    room_id: String,
    device_id: String,
    mongo: &Database,
) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .update_one(
            doc! { "_id": id },
            doc! { "$unset": { format!("rooms.{}.devices.{}", room_id, device_id): "" }},
            None,
        )
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}
