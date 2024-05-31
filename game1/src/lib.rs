use std::io;

use rand::Rng;

pub fn play_game() -> i64 {
    ui::clear_terminal();

    // choix du mot aléatoire
    let words = vec!["rust", "cargo", "programming", "cli", "training", "game"];

    let mut rng = rand::thread_rng();
    let nombre_secret = rng.gen_range(1..words.len());
    let target_word = words.get(nombre_secret).unwrap();

    let target_chars: Vec<char> = target_word.chars().collect();

    let mut correct_guesses: Vec<char> = vec![' '; target_word.len()];
    let mut misplaced_guesses: Vec<char>;
    let mut nombre_essai = 10;
    let score_final: i64;

  println!("Bienvenue dans le jeu Motus en Rust CLI");
  println!("Vous avez au total {} essais", nombre_essai);

  loop {
      // Demander au joueur de deviner le mot
      println!("Devinez le mot ({} lettres) !", target_word.len());
      let mut user_input = String::new();
      io::stdin().read_line(&mut user_input).expect("Erreur lors de la lecture");

      let user_input = user_input.trim();

      if !user_input.len().eq(&target_word.len()) {
        if nombre_essai > 0 {
            nombre_essai -= 1;
        }
        println!("Erreur de longueur de mot. il vous reste {} essai(s)", nombre_essai);
        continue;
      }
     

      let user_chars: Vec<char> = user_input.trim().chars().collect();

      misplaced_guesses = Vec::new();
      for (index, &user_char) in user_chars.iter().enumerate() {
          if let Some(char) = target_chars.get(index) {
           if char.eq(&user_char) {
            correct_guesses[index] = user_char;
           }else {
            correct_guesses[index] = ' ';
            if target_chars.contains(&user_char) {
                misplaced_guesses.push(user_char);
            } 
           }
          
            
          }   
        }

        // vérification du mot total correctement deviner
        if correct_guesses.iter().all(|&c| c != ' ') {
            println!("Felicitations, vous avez deviné le mot:  {} !", target_word);
            score_final = nombre_essai * 10;
            break;
        }

        println!("Lettres correctes et bien placée : {:?}", correct_guesses);
        println!("Lettres correctes mais mal placées : {:?}", misplaced_guesses);
        println!("Réessayez!\n");
        if nombre_essai > 0 {
            nombre_essai -= 1;
        }
        println!("Nombre d'essai(s) restant : {}", nombre_essai);

  }
  score_final
}


