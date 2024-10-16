use chrono::{DateTime as ChronoDateTime, Utc};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::dog_model::Dog;
use super::owner_model::Owner;

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub cancelled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(req: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = ChronoDateTime::parse_from_rfc3339(&req.start_time)
            .map_err(|err| format!("Failed to parse start time: {}", err))?
            .with_timezone(&Utc)
            .into();

        let owner_id = ObjectId::parse_str(&req.owner)
            .map_err(|err| format!("Failed to parse owner ObjectId: {}", err))?;

        Ok(Self {
            _id: ObjectId::new(),
            owner: owner_id,
            start_time: DateTime::from(chrono_datetime),
            duration_in_minutes: req.duration_in_minutes,
            cancelled: false,
        })
    }
}
