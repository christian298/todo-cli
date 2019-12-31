use super::todo::Todo;
use rusqlite::{params, Connection, Result, NO_PARAMS};

pub struct TodoStore {
    conn: Connection,
}

impl TodoStore {
    pub fn new() -> TodoStore {
        let conn = Connection::open("todo.db").expect("Failed to connect DB");
        conn.execute(
            "CREATE TABLE if not exists todos(id integer primary key, title text not null, done boolean)",
            NO_PARAMS,
        ).expect("Failed to create table");

        TodoStore { conn }
    }

    pub fn add_todo(&self, title: String, done: bool) {
        self.conn
            .execute(
                "INSERT INTO todos (title, done) values(?1, ?2)",
                params!(title, done),
            )
            .expect("Failed to add todo");
    }

    pub fn read_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, done FROM todos")
            .expect("Failed getting todos");

        let rows = stmt
            .query_map(NO_PARAMS, |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2).unwrap_or(false),
                })
            })
            .unwrap();

        let mut todos = Vec::new();
        for row in rows {
            todos.push(row.unwrap());
        }

        Ok(todos)
    }

    pub fn toggle_status(&self, status: bool, id: u32) {
        match self.conn.execute(
            "UPDATE todos SET done = (?1) WHERE id = (?2)",
            params!(status, id),
        ) {
            Ok(updated) => println!("{} rows were updated", updated),
            Err(err) => println!("update failed: {}", err),
        }
    }
}
