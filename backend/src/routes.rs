#![allow(non_snake_case)]
use crate::prelude::*;

use axum::extract::State;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub async fn api_test(State(state): State<AppState>) -> impl IntoResponse {
    let query = sqlx::query_as!(Locations, "SELECT * FROM Locations")
        .fetch_all(&state.db)
        .await;

    if query.is_err() {
        log::error!("Unable to fetch items from db!");
        return Json(json!({"error": "unabel to fetch items"}));
    }

    let data = query.unwrap();

    let resp = json!(data);
    log::debug!("{:?}", resp);
    Json(resp)
}

pub async fn get_list(State(state): State<AppState>) -> impl IntoResponse {
    let query = sqlx::query_as!(
        ShoppingData,
        "SELECT shoppinglist.id, shoppinglist.item, locations.name AS location FROM shoppinglist INNER JOIN locations ON shoppinglist.location=locations.id"
    )
    .fetch_all(&state.db)
    .await;

    if query.is_err() {
        log::error!("Unable to fetch items from db!");
        return Json(ReturnError::<
            ServerResponse<HashMap<String, Vec<ShoppingData>>>,
        >("Unable to fetch items!"));
    }

    // for each item combine into a list for each location
    let mut map: HashMap<String, Vec<ShoppingData>> = HashMap::new();

    let data = query.unwrap();

    for n in data {
        let name = n.location.clone();
        match map.entry(name) {
            Entry::Vacant(_) => {
                map.insert(n.location.clone(), vec![n.clone()]);
            }
            Entry::Occupied(e) => {
                e.into_mut().push(n.clone());
            }
        }
    }

    let resp = serde_json::json!(ServerResponse::<HashMap::<String, Vec<ShoppingData>>> {
        error: false,
        message: None,
        data: Some(map),
    });
    Json(resp)
}

pub async fn get_location_list(State(state): State<AppState>) -> impl IntoResponse {
    let query = sqlx::query_as!(Locations, "SELECT * FROM Locations")
        .fetch_all(&state.db)
        .await;

    if query.is_err() {
        log::error!("Unable to fetch items from db!");
        return Json(ReturnError::<ServerResponse<Vec<Locations>>>(
            "Unable to fetch items!",
        ));
    }

    let data = query.unwrap();

    let resp = serde_json::json!(ServerResponse::<Vec<Locations>> {
        error: false,
        message: None,
        data: Some(data),
    });

    Json(resp)
}

pub fn ReturnError<T: serde::ser::Serialize>(msg: &str) -> serde_json::Value {
    serde_json::json!(ServerResponse::<T> {
        error: true,
        message: Some(msg.to_owned()),
        data: None,
    })
}
