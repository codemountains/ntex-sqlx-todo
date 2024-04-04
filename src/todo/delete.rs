use crate::db::Db;
use ntex::web;
use serde::Deserialize;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct DeleteTodoPath {
    todo_id: i32,
}

#[web::delete("/todos/{todo_id}")]
pub async fn delete_todo(
    db: web::types::State<Db>,
    path: web::types::Path<DeleteTodoPath>,
) -> Result<impl web::Responder, web::Error> {
    match delete(&db.0, path.todo_id).await {
        Ok(_) => Ok(web::HttpResponse::NoContent()),
        Err(e) => {
            eprintln!("{}", e);
            Ok(web::HttpResponse::InternalServerError())
        }
    }
}

async fn delete(db: &Pool<Postgres>, id: i32) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
            delete from
                todos
            where
                id = $1
        "#,
        id
    )
    .execute(db)
    .await
}
