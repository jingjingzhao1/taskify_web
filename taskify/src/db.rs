use rusqlite::{Connection, Result,Error};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub progress: u8,
}

impl Todo {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: None,
            title: title.to_string(),
            description: description.to_string(),
            progress: 0,
        }
    }
}


pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(filename: String) -> Self {
        let set = Self {
            conn: match Connection::open(filename) {
                Ok(conn) => conn,
                Err(_) => Connection::open_in_memory().unwrap(),
            },
        };

        set.init().unwrap();
        set
    }

    pub fn init(&self) -> Result<(), Error> {
        let stmt = "
        CREATE TABLE IF NOT EXISTS todo (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title  TEXT UNIQUE NOT NULL,
            description TEXT,
            progress INTEGER DEFAULT 0
        );
        ";
        match self.conn.execute(stmt, ()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn insert(&self, item: Todo) -> Result<Todo, Error> {
        let mut stmt = match self
            .conn
            .prepare("INSERT INTO todo (title, description, progress) VALUES (?1, ?2, ?3);")
        {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let res = stmt.execute([&item.title, &item.description,&item.progress.to_string()]);
        match res {
            Ok(_) => {
                let last_id = self.conn.last_insert_rowid() as u32;
                Ok(Todo {
                    id: Some(last_id),
                    title: item.title,
                    description: item.description,
                    progress: item.progress,
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let mut stmt = match self.conn.prepare("SELECT id, title, description, progress FROM todo;") {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let res = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                progress: row.get(3).unwrap(),
            })
        });
        match res {
            Ok(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    list.push(row.unwrap());
                }
                Ok(list)
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_by_id(&self, id: u32) -> Result<Todo, Error> {
        let mut stmt = match self
            .conn
            .prepare("SELECT id, title, description, progress FROM todo WHERE id = ?1;")
        {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let res = stmt.query_map([id], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                progress: row.get(3).unwrap(),
            })
        });
        match res {
            Ok(rows) => {
                let mut rows = rows;
                Ok(rows.next().unwrap().unwrap())
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_todo(&self, item: &Todo) -> Result<(), Error> {
        let mut stmt = match self.conn.prepare("UPDATE todo SET title = ?1, description = ?2, progress = ?3 WHERE id = ?4;") {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let res = stmt.execute([&item.title, &item.description, &item.progress.to_string(), &item.id.unwrap().to_string()]);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(rusqlite::Error::InvalidParameterName(format!("Failed to update todo: {}", err))),
        }
    }

    pub fn delete_by_id(&self, id: u32) -> Result<(), Error> {
        let mut stmt = match self.conn.prepare("DELETE FROM todo WHERE id = ?1;") {
            Ok(stmt) => stmt,
            Err(err) => return Err(err),
        };
        let res = stmt.execute([id]);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    


}