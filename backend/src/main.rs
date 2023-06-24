mod route;
mod routes;
mod types;

mod prelude {
    pub use axum::body::{boxed, Body};
    pub use axum::http::{Response, StatusCode};
    pub use axum::{response::IntoResponse, routing::get, routing::post, Json, Router};
    pub use clap::Parser;
    pub use dotenv::dotenv;
    pub use serde_json::json;
    pub use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
    pub use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    pub use std::path::PathBuf;
    pub use std::str::FromStr;
    pub use tokio::fs;
    pub use tower::{ServiceBuilder, ServiceExt};
    pub use tower_http::services::ServeDir;
    pub use tower_http::trace::TraceLayer;

    pub use common::server_response::*;
    pub use common::*;

    #[derive(Clone)]
    pub struct AppState {
        pub db: Pool<Postgres>,
    }
}

use crate::prelude::*;

use crate::route::items::*;
use crate::route::location::*;
use crate::routes::*;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "wasm test server")]
struct Opt {
    // set log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    // set listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    // list port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    // static dir
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // setup logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    // read in our .env file to get db information
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // connect to the database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            log::info!("connection to db successful");
            pool
        }
        Err(err) => {
            log::error!("failed to connect to db: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/get", get(get_list))
        .route("/api/location/list", get(get_location_list))
        .route("/api/location/add", post(add_location))
        .route("/api/item/add", post(add_item))
        .route("/api/test", get(api_test))
        .fallback_service(get(|req| async move {
            match ServeDir::new(&opt.static_dir).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };

                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .with_state(AppState { db: pool.clone() })
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("unable to start server");
}

async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD app";

    Json(json!({"status": "success", "message": MESSAGE}))
}
