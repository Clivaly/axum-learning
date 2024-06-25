// Our Router File

mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variables::{hard_coded_path, path_variables};

// main function to create our routes
pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/11", get(hard_coded_path))
}
