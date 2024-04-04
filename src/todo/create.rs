use crate::db::Db;
use crate::todo::Todo;
use ntex::web;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
    status: String,
}

#[derive(Serialize)]
struct CreateTodoResponse {
    todo: Todo,
}

#[web::post("/todos")]
pub async fn create_todo(
    db: web::types::State<Db>,
    todo: web::types::Json<CreateTodoRequest>,
) -> Result<impl web::Responder, web::Error> {
    match insert(&db.0, todo.0).await {
        Ok(todo) => {
            let resp = CreateTodoResponse {
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

async fn insert(db: &Pool<Postgres>, todo: CreateTodoRequest) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        r#"
            insert into
                todos ("title", "status")
            values
                ($1, $2)
            returning
                *
        "#,
        todo.title,
        todo.status
    )
    .fetch_one(db)
    .await
}
