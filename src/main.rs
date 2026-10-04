use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Sehat {
    status: &'static str,
    db: &'static str,
}

async fn sehat() -> Json<Sehat> {
    Json(Sehat { status: "ok", db: "pedaree" })
}

pub fn rute() -> Router {
    Router::new().route("/kesehatan", get(sehat))
}

#[tokio::main]
async fn main() {
    let app = rute();
    let pendengar = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("gagal mengikat port");
    axum::serve(pendengar, app).await.expect("server berhenti");
}
