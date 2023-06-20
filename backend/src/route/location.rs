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
        Ok(i) => {
            match i {
                Some(_) => {
                    // we have found a match
                    return Json(ServerResponse::<bool> {
                        error: true,
                        message: Some(String::from("Already found in database")),
                        data: None,
                    });
                }
                None => {}
            }
        }

        Err(e) => return Json(ServerResponse::<bool>::new(true, e.to_string())),
    }

    let sql = sqlx::query!("INSERT INTO Locations (name) VALUES ($1)", data.name);
    let res = sql.execute(&state.db).await;

    let resp = match res {
        Ok(_) => ServerResponse::<bool> {
            error: false,
            message: None,
            data: Some(true),
        },
        Err(e) => ServerResponse::<bool> {
            error: true,
            message: Some(e.to_string()),
            data: None,
        },
    };

    Json(resp)
}
