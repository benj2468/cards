pub trait Game {
    fn win(&self) -> bool;
    fn handle_input(&mut self);
}