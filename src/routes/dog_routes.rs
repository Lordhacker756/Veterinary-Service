use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::dog_model::{Dog, DogRequest},
    services::db::Database,
};

#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> HttpResponse {
    match db
        .create_dog(
            Dog::try_from(DogRequest {
                owner: request.owner.clone(),
                name: request.name.clone(),
                age: request.age.clone(),
                breed: request.breed.clone(),
            })
            .expect("Error converting request body to Dog Object"),
        )
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
