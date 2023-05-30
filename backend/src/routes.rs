use crate::prelude::*;
use common::*;

use axum::extract::State;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub async fn get_list(State(state): State<AppState>) -> impl IntoResponse {
    // let query = sqlx::query_as!(ShoppingList, "SELECT * FROM shoppinglist",)
    let query = sqlx::query_as!(
        ShoppingData,
        "SELECT shoppinglist.id, shoppinglist.item, locations.name FROM shoppinglist INNER JOIN locations ON shoppinglist.location=locations.id"
    )
    .fetch_all(&state.db)
    .await;

    if query.is_err() {
        let msg = "Unable to fetch items";
        log::error!("Unable to fetch items from db!");
        return Json(json!({"status":"error","message":msg}));
    }

    // for each item combine into a list for each location
    let mut map: HashMap<String, Vec<ShoppingData>> = HashMap::new();

    let data = query.unwrap();

    for n in data {
        // if !data.contains_key(n.name) {
        //     // add new entry
        //     data.insert(n.name, vec![n.item]);
        // } else {
        //     data.entry(n.name).and_modify().push(n.item);
        // }
        // match map.entry(n.name.clone()) {
        let name = n.name.clone();
        match map.entry(name) {
            Entry::Vacant(_) => {
                map.insert(n.name.clone(), vec![n.clone()]);
            }
            Entry::Occupied(e) => {
                e.into_mut().push(n.clone());
            }
        }
    }

    // let resp = serde_json::json!(ServerResponse::Ok(query.unwrap()));
    let resp = serde_json::json!(ServerResponse::<HashMap::<String, Vec<ShoppingData>>> {
        error: false,
        message: None,
        // data: Some(query.unwrap()),
        data: Some(map),
    });
    Json(resp)
}
