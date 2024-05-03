use rusqlite::Connection;


pub struct User {
    pub id: i64,
    pub name: String
}

pub fn get_user(conn: &Connection, username: &str) -> User {
    let option: Option<i64> = conn.query_row("SELECT id FROM users WhERE name = ?1", &[&username], |row| row.get(0)).ok();

    match option {
        Some(id) => {
             User {
                id,
                name: username.to_string()
            }
        },
        None => {
            User {
                id: create_user(conn, username),
                name: username.to_string()
            }
        }
    }
}

pub fn create_user(conn: &Connection, username: &str) -> i64 {
    conn.execute("INSERT INTO users (name) VALUES (?1)", &[&username]).expect("Erreur lors de la cr&ation d'un nouveau joueur");
    conn.last_insert_rowid()
}