use actix_web::web;
use client::SmartHouse;
use mongodb::{bson::doc, Database};

pub async fn create(house: web::Json<SmartHouse>, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .insert_one(house.into_inner(), None)
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}

pub async fn delete(id: web::Path<String>, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("houses")
        .delete_one(doc! { "_id": id.into_inner() }, None)
        .await
        .map_err(|err| format!("Database error: {:?}", err))?;
    Ok(())
}
