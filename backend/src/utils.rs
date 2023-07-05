use actix_web::HttpResponse;
use serde_json::json;

pub fn not_found_response(id: &uuid::Uuid) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "status": "failure",
        "message": format!("Failed to query feedback {}", id).to_owned(),
    }))
}
