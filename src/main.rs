use axum::{Json, Router};
use axum::http::{header, HeaderName, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::{get, post};

use crate::gpt_api::Message;

mod log;
mod config;
mod gpt_api;

async fn chat(Json(data): Json<Message>) -> (StatusCode, AppendHeaders<[(HeaderName, &'static str); 4]>, Json<String>) {
    log_info!("提问: {}", data.msg);
    let gpt_res = gpt_api::new(data.msg).await;
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "application/json"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "POST, GET, OPTIONS, DELETE"),
        (header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"),
    ]);
    (StatusCode::default(), headers, Json::from(gpt_res))
}

async fn handler_404() -> impl IntoResponse {
    let json = serde_json::json!({
        "code": "404",
        "msg": "路径不存在"
    });
    (StatusCode::NOT_FOUND, Json(json))
}

#[tokio::main]
async fn main() {
    let url = format!("0.0.0.0:{}", config::get().await.server);

    log_info!("服务启动");
    let app = Router::new()
        .route("/", get(|| async { "This is an Void Web by Mu_zi_xi" }))
        .route("/chat", post(chat))
        .fallback(handler_404);

    // 启动服务
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.expect("服务启动错误");
}
