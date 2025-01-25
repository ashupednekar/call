use axum::{routing::get, Router};

use crate::{conf::settings, pkg::server::handlers::{probes::livez, ws::handle_ws}, prelude::Result};



pub async fn listen() -> Result<()> {
    let listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", &settings.listen_port))
            .await
            .unwrap();
    let router = Router::new()
        .route("/livez/", get(livez))
        .route("/ws/", get(handle_ws));
    tracing::info!("listening at :{}", &settings.listen_port);
    axum::serve(listener, router)
        .await?;
    Ok(())
}
