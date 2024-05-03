use std::io;
use rusqlite::{Connection, Result};

mod models;
use models::user::get_user;
use models::score::{add_score, show_alls_scores};
mod database;
use database::sqlite::initiate_db;



fn main() -> Result<()> {

    let conn = Connection::open("scores.sqlite").expect("Erreur lors de l'accès à la BDD");
    initiate_db(&conn);
   
   println!("Veuillez entrer votre prénom: ");
   let mut username = String::new();
   io::stdin().read_line(&mut username).expect("Erreur lors de la lecture de l'entrée");
   let user = get_user(&conn, username.trim());
   add_score(&conn, user.id, 1, 64);
   

   loop {

        // Affichage du menu principale
        println!("Menu principale pour l'utilisateur {} :", username);
        println!("1. Jouer au jeu 1");
        println!("2. Jouer au jeu 2");
        println!("3. Jouer au jeu 3");
        println!("4. Affichage de mes scores");
        println!("5. Quitter");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erreur lors de la lecture de l'entrée");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) =>  {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        match choice {
            4 => show_alls_scores(&conn, &user)?,
            5 => {
                println!(" Au revoir !");
                break Ok(());
            },
            _ => println!("Chiffre non valide. Veuillez entrer un chiffre entre 1 et 5")
            
        }

   }
}
