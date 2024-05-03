use rusqlite::Connection;

pub const GAMES: [&str; 3] = ["Jeu1", "Jeu2", "Jeu3"];

struct Game {
    id: i64,
    name: String
}

pub fn add_game(conn: &Connection, game_name: &str) {
    if get_bame_by_name(conn, game_name).is_none() {
        conn.execute("INSERT INTO games (name) VALUES (?1)", &[&game_name]).expect("Error lors de l'ajout du jeu");
    }

}

fn get_bame_by_name(conn: &Connection, name: &str) -> Option<Game> {
    conn.query_row("SELECT id, name FROM games WHERE name = ?1", &[&name], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?
        })
    }).ok()
}