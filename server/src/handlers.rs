use crate::database;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use client::devices::Devices;
use client::room::Room;
use client::SmartHouse;
use mongodb::Database;

pub async fn create_house(mongo: Data<Database>, house: web::Json<SmartHouse>) -> HttpResponse {
    match database::houses::create(house, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn delete_house(mongo: Data<Database>, id: web::Path<String>) -> HttpResponse {
    match database::houses::delete(id, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn add_room(
    mongo: Data<Database>,
    room: web::Json<Room>,
    params: web::Path<(String, String)>,
) -> HttpResponse {
    let (id, room_id) = params.into_inner();
    match database::rooms::create(id, room_id, room, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn remove_room(
    mongo: Data<Database>,
    params: web::Path<(String, String)>,
) -> HttpResponse {
    let (id, room_id) = params.into_inner();
    match database::rooms::remove(id, room_id, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn add_device(
    mongo: Data<Database>,
    device: web::Json<Devices>,
    params: web::Path<(String, String, String)>,
) -> HttpResponse {
    let (id, room_id, device_id) = params.into_inner();
    match database::devices::create(id, room_id, device_id, device, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json(err)
        }
    }
}

pub async fn remove_device(
    mongo: Data<Database>,
    params: web::Path<(String, String, String)>,
) -> HttpResponse {
    let (id, room_id, device_id) = params.into_inner();
    match database::devices::remove(id, room_id, device_id, &mongo).await {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
