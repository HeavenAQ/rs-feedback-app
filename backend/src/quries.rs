use crate::{app_state, models::FeedbackModel, schema};
use actix_web::web;

pub async fn list_feedbacks(
    db: &web::Data<app_state::AppState>,
    limit: i32,
    offset: i32,
) -> Result<Vec<FeedbackModel>, sqlx::Error> {
    sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&db.db_pool)
    .await
}

pub async fn get_feedback(
    db: &web::Data<app_state::AppState>,
    id: &uuid::Uuid,
) -> Result<FeedbackModel, sqlx::Error> {
    sqlx::query_as!(FeedbackModel, "SELECT * FROM feedbacks WHERE id = $1", *id)
        .fetch_one(&db.db_pool)
        .await
}

pub async fn create_feedback(
    db: &web::Data<app_state::AppState>,
    body: &web::Json<schema::CreateFeedbackSchema>,
) -> Result<FeedbackModel, sqlx::Error> {
    sqlx::query_as!(
        FeedbackModel,
        "INSERT INTO feedbacks (text, rating) VALUES ($1, $2) RETURNING *",
        &body.text.to_string(),
        &body.rating,
    )
    .fetch_one(&db.db_pool)
    .await
}

pub async fn update_feedback(
    db: &web::Data<app_state::AppState>,
    id: &uuid::Uuid,
    body: web::Json<schema::UpdateFeedbackSchema>,
) -> Result<FeedbackModel, sqlx::Error> {
    let query_res = get_feedback(&db, &id).await?;
    sqlx::query_as!(
        FeedbackModel,
        "UPDATE feedbacks SET text=$2, rating=$3, updated_at=$4 WHERE id=$1 RETURNING *",
        *id,
        body.text.to_owned().unwrap_or(query_res.text),
        body.rating.to_owned().unwrap_or(query_res.rating),
        chrono::Utc::now(),
    )
    .fetch_one(&db.db_pool)
    .await
}

pub async fn delete_feedback(
    db: &web::Data<app_state::AppState>,
    id: &uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(FeedbackModel, "DELETE FROM feedbacks WHERE id = $1", *id)
        .execute(&db.db_pool)
        .await?
        .rows_affected();
    Ok(())
}
