use crate::prelude::*;

use axum::extract::State;

pub async fn add_location(State(state): State<AppState>, Json(data): Json<serde_json::Value>) {
    // pub fn add_location(Json(data): Json<serde_json::Value>) {
    log::debug!("[POST] Add Location {:?}", data);
}
