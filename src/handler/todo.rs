use std::sync::Arc;

use axum::{
    body::BoxBody,
    extract::Extension,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::*, Set};

use serde_json::json;
use todo::models::{ApiError, PostTodoRequest, PostTodoResponse, Todo, TodoResponse};

use crate::entity::{prelude::Todos, todos};
use crate::handler::auth::Auth;

pub async fn get_todo_list(Extension(state): Extension<Arc<crate::AppState>>) -> Response<BoxBody> {
    let todo_list = Todos::find().all(&state.db).await;

    match todo_list {
        Ok(todo_list) => (
            StatusCode::OK,
            Json(TodoResponse {
                todo_list: serde_json::from_str::<Vec<Todo>>(json!(todo_list).to_string().as_str())
                    .unwrap(),
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                status: i32::from(StatusCode::BAD_REQUEST.as_u16()),
                title: format!("{:?}", err),
            }),
        )
            .into_response(),
    }
}

pub async fn post_todo(
    Extension(state): Extension<Arc<crate::AppState>>,
    Json(params): Json<PostTodoRequest>,
    _: Auth,
) -> Response<BoxBody> {
    let todo = todos::ActiveModel {
        todo: Set(params.todo),
        ..Default::default()
    };
    let result = Todos::insert(todo).exec(&state.db).await;

    match result {
        Ok(result) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PostTodoResponse {
                last_insert_id: result.last_insert_id as i32,
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                status: i32::from(StatusCode::BAD_REQUEST.as_u16()),
                title: format!("{:?}", err),
            }),
        )
            .into_response(),
    }
}
