use crate::food_api::Api;

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    client: Api,
}

impl AppState {
    pub fn new(client: Api) -> Self {
        Self { client }
    }
}