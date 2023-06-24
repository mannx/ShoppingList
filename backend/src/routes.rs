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
        return Json(ServerResponse::<Vec<Locations>>::error(String::from(
            "Unable to fetch items",
        )));
    }

    let data = query.unwrap();
    Json(ServerResponse::<Vec<Locations>>::ok(data))
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
        return Json(ServerResponse::<HashMap<String, Vec<ShoppingData>>>::error(
            String::from("Unable to fetch items"),
        ));
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

    Json(ServerResponse::<HashMap<String, Vec<ShoppingData>>>::ok(
        map,
    ))
}

pub async fn get_location_list(State(state): State<AppState>) -> impl IntoResponse {
    let query = sqlx::query_as!(Locations, "SELECT * FROM Locations")
        .fetch_all(&state.db)
        .await;

    if query.is_err() {
        log::error!("Unable to fetch items from db!");
        return Json(ServerResponse::<Vec<Locations>>::error(String::from(
            "Unable to fetch items",
        )));
    }

    let data = query.unwrap();
    Json(ServerResponse::<Vec<Locations>>::ok(data))
}
