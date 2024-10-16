use actix_web::{
    get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    models::booking_model::{Booking, BookingRequest},
    services::db::Database,
};

#[derive(Deserialize)]
pub struct BookingQuery {
    owner_id: String,
}

#[get("/booking")]
pub async fn get_bookings(db: Data<Database>, query: Query<BookingQuery>) -> HttpResponse {
    let owner_id = &query.owner_id;

    match db.get_bookings(owner_id).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    match db
        .create_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_in_minutes: request.duration_in_minutes.clone(),
            })
            .expect("Error converting request body to Booking Object"),
        )
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/booking/cancel/{id}")]
pub async fn cancel_bookings(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;

    match db.cancel_booking(id.as_str()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
