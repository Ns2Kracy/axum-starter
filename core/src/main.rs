use axum::Extension;
use core::{api, context::Context};
use prisma::prisma::{self};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    let app = axum::Router::new()
        .nest("/api", api::router())
        .layer(Extension(Context { db }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
