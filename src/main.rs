mod db;
mod todo;

use crate::db::Db;
use crate::todo::create::create_todo;
use crate::todo::delete::delete_todo;
use crate::todo::find::find_todos;
use crate::todo::get::get_todo;
use crate::todo::update::update_todo;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let db = Db::new().await;

    web::HttpServer::new(move || {
        web::App::new()
            .state(db.clone())
            .service(find_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
