use crate::db::Db;
use crate::todo::Todo;
use ntex::web;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct UpdateTodoPath {
    todo_id: i32,
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    title: String,
    status: String,
}

#[derive(Serialize)]
struct UpdateTodoResponse {
    todo: Todo,
}

#[web::put("/todos/{todo_id}")]
pub async fn update_todo(
    db: web::types::State<Db>,
    path: web::types::Path<UpdateTodoPath>,
    todo: web::types::Json<UpdateTodoRequest>,
) -> Result<impl web::Responder, web::Error> {
    let target_todo = Todo {
        id: path.todo_id,
        title: todo.title.to_string(),
        status: todo.status.to_string(),
    };

    match update(&db.0, target_todo).await {
        Ok(todo) => {
            let resp = UpdateTodoResponse {
                todo: Todo {
                    id: todo.id,
                    title: todo.title,
                    status: todo.status,
                },
            };
            Ok(web::HttpResponse::Ok().json(&resp))
        }
        Err(e) => {
            eprintln!("{}", e);
            Ok(web::HttpResponse::InternalServerError().into())
        }
    }
}

async fn update(db: &Pool<Postgres>, todo: Todo) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        r#"
            update
                todos
            set
                title = $2,
                status = $3
            where
                id = $1
            returning
                *
        "#,
        todo.id,
        todo.title,
        todo.status
    )
    .fetch_one(db)
    .await
}
