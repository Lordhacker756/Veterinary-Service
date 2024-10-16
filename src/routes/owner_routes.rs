use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::Database,
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                country: request.country.clone(),
            })
            .expect("Error converting request body to Owner Object"),
        )
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
