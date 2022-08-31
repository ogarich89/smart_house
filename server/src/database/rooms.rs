use actix_web::web;
use client::room::Room;
use client::SmartHouse;
use mongodb::{
    bson::{doc, to_document},
    Database,
};
pub async fn create(
    id: String,
    room_id: String,
    room: web::Json<Room>,
    mongo: &Database,
) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .update_one(
            doc! { "_id": id },
            doc! { "$set": { format!("rooms.{}", room_id): to_document(&room.into_inner()).unwrap() }},
            None,
        )
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}

pub async fn remove(id: String, room_id: String, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .update_one(
            doc! { "_id": id },
            doc! { "$unset": { format!("rooms.{}", room_id): "" }},
            None,
        )
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}
