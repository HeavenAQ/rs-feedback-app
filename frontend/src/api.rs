use common::{ErrorResponse, Feedback, FeedbackListResponse, FeedbackResponse};
use reqwasm::http::{self, Response};

const API_URL: &'static str = "http://localhost:8080/api";

async fn request_err_msg(res: &Response, default_err_msg: String) -> String {
    match res.json::<ErrorResponse>().await {
        Ok(err) => err.message,
        Err(_) => default_err_msg,
    }
}

pub async fn create_feedback(feedback_data: &str) -> Result<Feedback, String> {
    let err_msg = "Failed to create feedback".to_string();
    let response = http::Request::post(&format!("{}/feedback", &API_URL))
        .header("Content-Type", "application/json")
        .body(feedback_data)
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(_) => return Err(err_msg),
    };

    if response.status() != 200 {
        return Err(request_err_msg(&response, err_msg).await);
    }

    // success
    match response.json::<FeedbackResponse>().await {
        Ok(feedback) => Ok(feedback.data.feedback),
        Err(err) => Err(format!("Failed to parse response:\n{}", err)),
    }
}

pub async fn get_single_feedback(id: &str) -> Result<Feedback, String> {
    let err_msg = format!("Failed to get feedback id: {}", id);
    let response = http::Request::get(&format!("{}/feedbacks/{}", &API_URL, id))
        .header("Content-Type", "application/json")
        .send()
        .await;

    let res = match response {
        Ok(res) => res,
        Err(_) => return Err(err_msg),
    };

    if res.status() != 200 {
        return Err(request_err_msg(&res, err_msg).await);
    }

    match res.json::<FeedbackResponse>().await {
        Ok(feedback) => Ok(feedback.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn list_feedbacks((page, limit): (i32, i32)) -> Result<Vec<Feedback>, String> {
    let err_msg = format!(
        "Failed to get feedbacks with page: {} and limit {}",
        page, limit
    );

    let response = http::Request::get(&format!(
        "{}/feedbacks/?limit={}&page={}",
        &API_URL, page, limit
    ))
    .header("Content-Type", "application/json")
    .send()
    .await;

    let res = match response {
        Ok(res) => res,
        Err(_) => return Err(err_msg),
    };

    if res.status() != 200 {
        return Err(request_err_msg(&res, err_msg).await);
    }

    match res.json::<FeedbackListResponse>().await {
        Ok(feedback) => Ok(feedback.data),
        Err(_) => return Err("Failed to parse response".to_string()),
    }
}

pub async fn patch_feedback(id: &str, feedback_data: &str) -> Result<Feedback, String> {
    let err_msg = format!("Failed to update feedback {}", id).to_owned();
    let response = http::Request::patch(&format!("{}/feedbacks/{}", &API_URL, id))
        .header("Content-Type", "application/json")
        .body(feedback_data)
        .send()
        .await;

    let res = match response {
        Ok(res) => res,
        Err(_) => return Err(err_msg),
    };

    if res.status() != 200 {
        return Err(request_err_msg(&res, err_msg).await);
    }

    match res.json::<FeedbackResponse>().await {
        Ok(feedback) => Ok(feedback.data.feedback),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn delete_feedback(id: &str) -> Result<(), String> {
    let err_msg = format!("Failed to delete feedback {}", id);
    let response = http::Request::delete(&format!("{}/feedbacks/{}", &API_URL, id))
        .header("Content-Type", "application/json")
        .send()
        .await;

    let res = match response {
        Ok(res) => res,
        Err(_) => return Err(err_msg),
    };

    if res.status() != 204 {
        return Err(request_err_msg(&res, err_msg).await);
    }

    Ok(())
}
