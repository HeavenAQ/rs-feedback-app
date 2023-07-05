use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamOptions {
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFeedbackSchema {
    pub rating: i32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFeedbackSchema {
    pub rating: Option<i32>,
    pub text: Option<String>,
}
