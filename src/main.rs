use cards;
use std::io;

fn main() -> io::Result<()> {
    cards::select_game()?;

    Ok(())
}
