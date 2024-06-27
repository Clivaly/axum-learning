// Our Router File

mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;


use axum::{
    body::Body, http::Method, routing::{get, post}, Extension, Router
};

use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::{hard_coded_path, path_variables};
use query_params::query_params;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

// main function to create our routes
pub fn create_routes() -> Router<Body> {
    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData{message: "Hello from Shared Data".to_owned()};

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/11", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
    }
