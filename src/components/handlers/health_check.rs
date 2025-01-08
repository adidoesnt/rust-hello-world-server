use axum::Json;
use serde_json::{json, Value};

pub async fn handler() -> Json<Value> {
    let response_body: Value = json!({
        "message": "Server health OK"
    });

    Json(response_body)
}
