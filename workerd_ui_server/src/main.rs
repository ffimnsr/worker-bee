use axum::{Json, Router, Server};
use axum::extract::{DefaultBodyLimit, State, FromRef};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::signal;

#[allow(dead_code)]
pub(crate) mod workerd_capnp {
    include!(concat!(env!("OUT_DIR"), "/workerd_capnp.rs"));
}

mod store;
mod workerd;

use store::{Store, Worker};

#[derive(FromRef, Clone)]
struct AppState {
    uptime: Arc<Instant>,
    store: Arc<Store>,
}

async fn health(State(uptime): State<Arc<Instant>>) -> impl IntoResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = serde_json::json!({
        "uptime": uptime.elapsed().as_secs(),
        "timestamp": timestamp,
        "status": "ok",
    });

    (StatusCode::OK, Json(payload))
}

async fn read_config(State(app): State<AppState>) -> impl IntoResponse {
    // app.store.create_worker(&Worker {
    //     name: "test".to_string(),
    //     service_worker_script: "test".to_string(),
    //     compatibility_date: "test".to_string(),
    // }).await.unwrap();

    let data = app.store.get_all().await;
    dbg!(data);

    let payload = serde_json::json!({
        "status": "ok",
    });
    (StatusCode::OK, Json(payload))
}

async fn write_config(State(app): State<AppState>) -> impl IntoResponse {
    let payload = serde_json::json!({
        "status": "ok",
    });
    (StatusCode::OK, Json(payload))
}

fn router(store: Arc<Store>) -> Router {
    let uptime = Arc::new(Instant::now());
    let app_state = AppState {
        uptime: uptime.clone(),
        store: store.clone(),
    };
    Router::new()
        .route("/health", get(health))
        .route("/config", get(read_config))
        .route("/config", post(write_config))
        // .route("/config", routing::post(set_config))
        .with_state(app_state)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10mb
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log::info!("signal received, starting graceful shutdown");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "workerd_ui_server=trace,tower_http=debug".into()),
        )
        .init();

    let store = Store::new().await;
    let store = Arc::new(store);
    let router = router(store.clone());

    Server::bind(&"127.0.0.1:3000".parse().expect("failed to parse address"))
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
