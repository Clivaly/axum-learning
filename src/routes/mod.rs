mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_tasks;
mod hello_world;
mod partial_update_task;
mod update_tasks;
mod validate_with_serde;
mod create_user;

use axum::{
    routing::{delete, get, patch, post, put},
    Extension, Router,
};
use create_task::create_task;
use create_user::create_user;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_tasks;
use get_tasks::{get_all_tasks, get_task_by_id};
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use validate_with_serde::validate_with_serde;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world::hello_world))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_task_by_id))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_tasks))
        .route("/user", post(create_user))
        .layer(Extension(database))
}
