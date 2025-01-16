use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Local;

use crate::{infrastructure::surreal::todo_repository::TodoRepository, entities::todo::Todo};

pub async fn create_todo_command(
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Todo already exists",
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let datetime = Local::now();
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    let todo = match repository.create_todo(todo.clone()).await.unwrap() {
        Some(todo) => todo,
        None => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": "Failed to create todo",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)));
        }
    };

    let json_response = serde_json::json!({
        "status": "success",
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
