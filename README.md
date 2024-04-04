# Todo App using Ntex and SQLx

Todo application using [Ntex](https://ntex.rs/) and [SQLx](https://github.com/launchbadge/sqlx).

## Endpoints

- **GET** `/todos?status={status_code}`: Get and search for a list of todos.
- **POST** `/todos`: Create a new todo.
- **GET** `/todos/{id}`: Get details of a todo.
- **PUT** `/todos/{id}`: Update a todo.
- **DELETE** `/todos/{id}`: Delete a todo.

## Debug

```shell
DATABASE_URL={your_db_url} cargo run
```
