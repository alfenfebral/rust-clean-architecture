use axum::{
    routing::{get, post},
    Router,
};

use super::health_checker_handler;
use crate::usecases::{
    todo::commands::{
        create_todo_command::create_todo_command, delete_todo_command::delete_todo_command,
        update_todo_command::update_todo_command,
    },
    todo::queries::{
        get_all_todos_query::get_all_todos_query, get_todo_by_id_query::get_todo_by_id_query,
    },
};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/todos",
            post(create_todo_command).get(get_all_todos_query),
        )
        .route(
            "/api/todos/{id}",
            get(get_todo_by_id_query)
                .put(update_todo_command)
                .delete(delete_todo_command),
        )
}
