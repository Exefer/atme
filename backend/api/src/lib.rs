mod frontend;

use anyhow::Ok;
use atme_service::Mutation as MutationCore;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use entity::{charge_records, prelude::*};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::{env, net::SocketAddr};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

// TODO: Use JSON for all handlers response body

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&db, None).await.unwrap();

    let state = AppState { db };

    let app = Router::new()
        .route(
            "/api/test",
            get(async || {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                "text data after 3 seconds"
            }),
        )
        .route(
            "/api/phone/charge-records/delete/{id}",
            delete(delete_charge_record_handler),
        )
        .route(
            "/api/phone/charge-records/plug",
            post(create_charge_record_handler),
        )
        .route(
            "/api/phone/charge-records/unplug",
            post(finalize_charge_record_handler),
        )
        .route(
            "/api/phone/charge-records",
            get(list_charge_records_handler),
        )
        .with_state(state)
        .fallback(frontend::static_handler);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct ChargePercentagePayload {
    charge_percentage: i32,
}

async fn create_charge_record_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChargePercentagePayload>,
) -> (StatusCode, &'static str) {
    MutationCore::create_charge_record(&state.db, payload.charge_percentage)
        .await
        .unwrap();
    (StatusCode::OK, "Charge record created")
}

async fn finalize_charge_record_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChargePercentagePayload>,
) -> (StatusCode, &'static str) {
    if MutationCore::update_last_charge_record(&state.db, payload.charge_percentage)
        .await
        .is_err()
    {
        (StatusCode::NOT_FOUND, "Cannot find charge record")
    } else {
        (StatusCode::OK, "Finalized charge record")
    }
}

async fn list_charge_records_handler(
    State(state): State<AppState>,
) -> Json<Vec<charge_records::Model>> {
    let result: Vec<charge_records::Model> = ChargeRecords::find()
        .into_model()
        .all(&state.db)
        .await
        .unwrap();
    Json(result)
}

async fn delete_charge_record_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, &'static str) {
    if MutationCore::delete_charge_record(&state.db, id)
        .await
        .is_err()
    {
        (StatusCode::NOT_FOUND, "Cannot find charge record")
    } else {
        (StatusCode::OK, "Deleted charge record")
    }
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
