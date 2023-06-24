use crate::prelude::*;

use axum::extract::State;

pub async fn add_location(
    State(state): State<AppState>,
    Json(data): Json<Locations>,
) -> impl IntoResponse {
    log::debug!("[add_location] Adding new location: {:?}", data.name);

    //  Check to see if entry is already present
    let sql = sqlx::query!(
        "SELECT id FROM locations WHERE LOWER(name)=LOWER($1)",
        data.name
    );
    let res = sql.fetch_optional(&state.db).await;

    match res {
        Ok(i) => match i {
            Some(_) => {
                return Json(ServerResponse::<bool>::error(String::from(
                    "Already found in database",
                )));
            }
            None => {}
        },

        Err(e) => return Json(ServerResponse::<bool>::error(e.to_string())),
    }

    let sql = sqlx::query!("INSERT INTO Locations (name) VALUES ($1)", data.name);
    let res = sql.execute(&state.db).await;

    let resp = match res {
        Ok(_) => ServerResponse::<bool>::ok(true),
        Err(e) => ServerResponse::<bool>::error(e.to_string()),
    };

    Json(resp)
}
