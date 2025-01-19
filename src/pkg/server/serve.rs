use std::net::TcpListener;

use axum::{routing::get, Router};

use crate::{conf::settings, pkg::server::handlers::probes::livez, prelude::Result};



pub async fn listen() -> Result<()> {
    let listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", &settings.listen_port))
            .await
            .unwrap();
    tracing::info!("listening at {}", &settings.listen_port);
    let router = Router::new()
        .route("/", get(livez));
    axum::serve(listener, router)
        .await?;
    Ok(())
}
