#[path = "games/aces_up.rs"] mod aces_up;
#[path = "games/klondike.rs"] mod klondike;
#[path = "games/game.rs"] mod game;
#[path = "utils/deck.rs"] mod deck;

use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::collections::HashMap;



pub fn select_game() -> std::io::Result<()> {

    let games = ["Aces Up", "Klondike"];


    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&games)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => 
        {
            println!("User selected game : {}", games[index]);
            println!("+ -------------------- + ");
            match games[index] {
                "Aces Up" => aces_up::AcesUpGame::play(),
                "Klondike" => klondike::Klondike::play(),
                _ => {
                    println!("Invalid selection");
                    select_game();
                }
            }
        }
        None => println!("User did not select anything")
    }

    Ok(())
}