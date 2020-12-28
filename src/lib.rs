#[path = "games/aces_up.rs"] mod aces_up;
#[path = "utils/deck.rs"] mod deck;

use aces_up::AcesUpGame;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::collections::HashMap;


pub fn play_aces_up()
{
    let mut game = AcesUpGame::new();

    game.start();
}

pub fn select_game() -> std::io::Result<()> {
    let mut games = HashMap::new();
    games.insert(String::from("Aces Up"), play_aces_up);

    let items: Vec<&String> = games.keys().collect::<Vec<&String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => 
        {
            println!("User selected game : {}", items[index]);
            println!("+ -------------------- + ");
            games[items[index]]();
        }
        None => println!("User did not select anything")
    }

    Ok(())
}