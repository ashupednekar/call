use axum::{routing::get, Router};

use crate::{conf::settings, pkg::server::handlers::{probes::livez, ui::handle_ui, ws::handle_ws}, prelude::Result};



pub async fn listen() -> Result<()> {
    let listener =
        tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", &settings.listen_port))
            .await
            .unwrap();
    let router = Router::new()
        .route("/ws/{from}/{to}/", get(handle_ws))
        .route("/livez/", get(livez))
        .route("/", get(handle_ui));
    tracing::info!("listening at :{}", &settings.listen_port);
    axum::serve(listener, router)
        .await?;
    Ok(())
}
