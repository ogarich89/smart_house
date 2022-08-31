mod database;
mod handlers;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.database("smart_houses")))
            .service(web::resource("/houses").route(web::post().to(handlers::create_house)))
            .service(web::resource("/houses/{id}").route(web::delete().to(handlers::delete_house)))
            .service(
                web::resource("/houses/{id}/rooms/{room_id}")
                    .route(web::post().to(handlers::add_room))
                    .route(web::delete().to(handlers::remove_room)),
            )
            .service(
                web::resource("/houses/{id}/rooms/{room_id}/devices/{device_id}")
                    .route(web::post().to(handlers::add_device))
                    .route(web::delete().to(handlers::remove_device)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
