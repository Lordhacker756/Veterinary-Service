mod models;
mod routes;
mod services;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::{
    booking_routes::{cancel_bookings, create_booking, get_bookings},
    dog_routes::create_dog,
    owner_routes::create_owner,
};
use services::init::init_database;
use std::error::Error;
use std::result::Result;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Supp bro")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    match init_database().await {
        Ok(db) => {
            println!("Database Connected Successfully...");
            let db_data = Data::new(db);

            HttpServer::new(move || {
                App::new()
                    .app_data(db_data.clone())
                    .service(hello)
                    .service(create_owner)
                    .service(create_dog)
                    .service(create_booking)
                    .service(cancel_bookings)
                    .service(get_bookings)
            })
            .bind(("localhost", 5001))?
            .run()
            .await?;

            println!("Server started successfully...");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to initialize the database: {}", e);
            Err(e)
        }
    }
}
