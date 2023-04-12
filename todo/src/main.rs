use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/todos", get(todos_list).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn todos_list() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn todos_create() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn todos_update() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn todos_delete() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
