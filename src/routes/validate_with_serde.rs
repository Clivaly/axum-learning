use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    // dbg!(&user);
    RequestUser {
        username: user.username,
        password: user.password,
    };
}
