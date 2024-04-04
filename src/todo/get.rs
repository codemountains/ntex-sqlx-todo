use crate::db::Db;
use crate::todo::Todo;
use ntex::web;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct GetTodoPath {
    todo_id: i32,
}

#[derive(Serialize)]
struct GetTodoResponse {
    todo: Todo,
}

#[web::get("/todos/{todo_id}")]
pub async fn get_todo(
    db: web::types::State<Db>,
    path: web::types::Path<GetTodoPath>,
) -> Result<impl web::Responder, web::Error> {
    println!("{:?}", path.todo_id);

    match select_one(&db.0, path.todo_id).await {
        Ok(todo) => {
            let resp = GetTodoResponse {
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
            Ok(web::HttpResponse::NotFound().into())
        }
    }
}

async fn select_one(db: &Pool<Postgres>, id: i32) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        r#"
            select
                id,
                title,
                status
            from
                todos
            where
                id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await
}
