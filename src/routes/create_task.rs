use crate::database::tasks;
use crate::database::users::{self, Entity as Users};
use axum::extract::State;
use axum::{
    // headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json,
};

use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn create_task(
    // Extension(database): Extension<DatabaseConnection>,
    State(database): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        title: Set(request_task.title),
        priority: Set(request_task.priority),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    Ok(())
}
