pub trait Game {
    fn play(&mut self) -> bool;
    fn win(&self) -> bool;
    fn handle_input(&mut self);
}