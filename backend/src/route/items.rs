use crate::prelude::*;
use axum::extract::State;

pub async fn add_item(
    State(state): State<AppState>,
    Json(data): Json<NewItem>,
) -> impl IntoResponse {
    log::debug!(
        "Adding new item to store: {:?}, {:?}",
        data.item_name,
        data.store_id
    );

    let sql = sqlx::query!(
        "INSERT INTO ShoppingList (item, location) VALUES($1, $2)",
        data.item_name,
        data.store_id
    );
    let res = sql.execute(&state.db).await;

    let resp = match res {
        Ok(_) => ServerResponse::<bool>::ok(true),
        Err(e) => ServerResponse::<bool>::error(e.to_string()),
    };

    Json(resp)
}
