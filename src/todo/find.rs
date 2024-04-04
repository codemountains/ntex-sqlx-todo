use crate::db::Db;
use crate::todo::Todo;
use ntex::web;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use std::fmt;

#[derive(Deserialize)]
struct FindTodosQuery {
    status: Option<FindTodosStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum FindTodosStatus {
    Working,
    Done,
}

impl fmt::Display for FindTodosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FindTodosStatus::Working => write!(f, "working"),
            FindTodosStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Serialize)]
struct FindTodosResponse {
    todos: Vec<Todo>,
}

#[web::get("/todos")]
pub async fn find_todos(
    db: web::types::State<Db>,
    query: web::types::Query<FindTodosQuery>,
) -> Result<impl web::Responder, web::Error> {
    match search(&db.0, query.0.status).await {
        Ok(todos) => {
            let resp = FindTodosResponse { todos };
            Ok(web::HttpResponse::Ok().json(&resp))
        }
        Err(e) => {
            eprintln!("{}", e);
            let resp = FindTodosResponse { todos: vec![] };
            Ok(web::HttpResponse::Ok().json(&resp))
        }
    }
}

async fn search(
    db: &Pool<Postgres>,
    search_param: Option<FindTodosStatus>,
) -> Result<Vec<Todo>, Error> {
    let search_status = if let Some(status) = search_param {
        status.to_string()
    } else {
        "".to_string()
    };

    sqlx::query_as!(
        Todo,
        r#"
            select
                *
            from
                todos
            where
                $1 = ''
                or
                ($1 != '' and status = $1)
        "#,
        search_status
    )
    .fetch_all(db)
    .await
}
