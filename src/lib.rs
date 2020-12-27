#[path = "games/aces_up.rs"] mod aces_up;
#[path = "utils/deck.rs"] mod deck;

use aces_up::AcesUpGame;

pub fn play_aces_up()
{
    let mut game = AcesUpGame::new();

    game.start();
}