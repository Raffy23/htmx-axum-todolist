use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use axum_session::{SessionConfig, SessionSqlitePool, SessionStore};
use htmx_axum_todolist::{
    db::open_database, routes::todo_routes, state::AppState, tracing::init_tracing, DEFAULT_DB_URI,
};
use std::future::IntoFuture;
use tokio::sync::watch;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let db = open_database(DEFAULT_DB_URI).await.unwrap();

    let session_config = SessionConfig::default().with_table_name("Session");
    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(db.clone().into()), session_config)
            .await
            .unwrap();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let state = AppState::default(db.clone()).await;

    let app_router = todo_routes(session_store)
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .with_state(state.clone());

    let metrics_router =
        Router::new().route("/metrics", get(|| async move { metric_handle.render() }));

    let listen_address = "127.0.0.1:3000";
    let metrics_address = "127.0.0.1:9999";

    info!("Listening on");
    info!("    - http://{}", listen_address);
    info!("    - http://{}/metrics", metrics_address);

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    let metrics_listener = tokio::net::TcpListener::bind(metrics_address)
        .await
        .unwrap();

    let (tx_signal, rx_signal) = watch::channel::<bool>(false);

    let app_server = {
        let mut tmp_rx_signal = rx_signal.clone();

        axum::serve(listener, app_router).with_graceful_shutdown(async move {
            tmp_rx_signal.changed().await.ok();
        })
    };

    let metrics_server = {
        let mut tmp_rx_signal = rx_signal.clone();

        axum::serve(metrics_listener, metrics_router).with_graceful_shutdown(async move {
            tmp_rx_signal.changed().await.ok();
        })
    };

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Shutting down server ...");

        tx_signal.send(true).unwrap();
    });

    let _ = tokio::join!(app_server.into_future(), metrics_server.into_future(),);

    info!("Shutting down database ...");
    db.close().await;

    info!("Done, exiting ...");
}
