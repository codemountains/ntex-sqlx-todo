use serde::{Deserialize, Serialize};

pub mod create;
pub mod delete;
pub mod find;
pub mod get;
pub mod update;

#[derive(Deserialize, Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    status: String,
}
