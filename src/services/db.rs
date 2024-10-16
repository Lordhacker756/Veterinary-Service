use mongodb::Collection;

use crate::models::{booking_model::Booking, dog_model::Dog, owner_model::Owner};

pub struct Database {
    pub booking: Collection<Booking>,
    pub owner: Collection<Owner>,
    pub dog: Collection<Dog>,
}
