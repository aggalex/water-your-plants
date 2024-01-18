use crate::persistence::Transaction;

#[derive(Clone)]
pub struct Plant {
    id: u64,
    pub name: String,
    pub plant_profile_id: u64,
    pub soil_moisture: f32,
    pub last_watered: f32,
    pub environment_humidity: f32,
    pub environment_temperature: f32
}

#[derive(From, Clone)]
pub struct PlantDao<'r>(&'r Transaction<'r>);