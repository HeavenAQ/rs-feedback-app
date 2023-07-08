use crate::{app_state, quries, schema};
use crate::{quries::*, utils};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/healthcheck")]
async fn healthcheck_handler() -> impl Responder {
    const MESSAGE: &str = "Hello, World";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE }))
}

#[get("/feedbacks/")]
async fn list_feedback_handler(
    opts: web::Query<schema::FilterOptions>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let query_res = quries::list_feedbacks(&data, limit as i32, offset as i32).await;
    if let Err(err) = query_res {
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to query feedbacks: {}", err)
        }));
    }
    let feedback_list = query_res.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "results": feedback_list.len(),
        "data": feedback_list,
    }))
}

#[get("/feedbacks/{id}")]
async fn get_feedback_handler(
    id_uri: web::Path<uuid::Uuid>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let id = id_uri.into_inner();
    let query_res = quries::get_feedback(&data, &id).await;
    match query_res {
        Ok(feedback) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": {
                "feedback": feedback
            },
        })),
        Err(_err) => utils::not_found_response(&id),
    }
}

#[post("/feedback")]
async fn post_feedback_handler(
    body: web::Json<schema::CreateFeedbackSchema>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    match create_feedback(&data, &body).await {
        Ok(feedback) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": {
                "feedback": feedback
            },
        })),
        Err(_err) => HttpResponse::ExpectationFailed().json(json!({
            "status": "failure",
            "message": "Failed to create the feedback",
        })),
    }
}

#[patch("/feedbacks/{id}")]
async fn edit_feedback_handler(
    id_uri: web::Path<uuid::Uuid>,
    body: web::Json<schema::UpdateFeedbackSchema>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let query_res = quries::update_feedback(&data, &id_uri.into_inner(), body).await;
    match query_res {
        Ok(feedback) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": {
                "feedback": feedback
            },
        })),
        Err(_err) => HttpResponse::NotFound().json(json!({
            "status": "failure",
            "message": "Failed to query feedbacks: {}",
        })),
    }
}

#[delete("/feedbacks/{id}")]
async fn delete_feedback_handler(
    id_uri: web::Path<uuid::Uuid>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let id = id_uri.into_inner();
    match delete_feedback(&data, &id).await {
        Ok(_res) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": format!("{} has been deleted", &id)
        })),
        Err(_err) => utils::not_found_response(&id),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_feedback_handler)
        .service(list_feedback_handler)
        .service(edit_feedback_handler)
        .service(post_feedback_handler)
        .service(delete_feedback_handler)
        .service(healthcheck_handler);
    conf.service(scope);
}
