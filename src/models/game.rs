use rusqlite::Connection;
use crate::models::user::User;
use crate::models::score::add_score;


pub const GAMES: [&str; 3] = ["Jeu1", "Jeu2", "Jeu3"];

struct Game {
    id: i64,
    name: String
}

pub fn add_game(conn: &Connection, game_name: &str) {
    if get_game_by_name(conn, game_name).is_none() {
        conn.execute("INSERT INTO games (name) VALUES (?1)", &[&game_name]).expect("Error lors de l'ajout du jeu");
    }

}

fn get_game_by_name(conn: &Connection, name: &str) -> Option<Game> {
    conn.query_row("SELECT id, name FROM games WHERE name = ?1", &[&name], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?
        })
    }).ok()
}

pub fn play_game(conn: &Connection, user: &User, game_name: &str) -> Result<i64, &'static str> {

    if let Some(game) = get_game_by_name(conn, game_name) {
        let score: i64;
        match game.name.as_str() {
            "Jeu1" => {
                score = game1::play_game();
            }
            _=> return Err("Game not found")
        }
        add_score(conn, user.id, game.id, score);
        Ok(score)
    } else {
        return Err("Game not found")
    }
}