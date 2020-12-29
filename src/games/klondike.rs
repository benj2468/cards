#[path = "../utils/deck.rs"] mod deck;
#[path = "../games/game.rs"] mod game;

use deck::{Stack, Card, Suit};
use std::collections::HashMap;
use text_io::read;
use game::Game;
use std::fmt;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};
use std::error::Error;

const COLUMNS: usize = 7;

#[derive(Copy, Clone, PartialEq)]
enum Position
{
    Top,
    Columns,
    Stack
}

#[derive(Copy, Clone)]
struct MoveCommand
{
    location: Position,
    i: usize
}

pub struct Klondike
{
    deck: Stack,
    discard: Vec<Card>,
    columns: Vec<Vec<Card>>,
    top_stacks: HashMap<Suit, Vec<Card>>,
}

impl Klondike
{
    fn new() -> Klondike
    {
        let mut deck = Stack::new_deck(false);

        deck.shuffle();

        let mut top_stacks = HashMap::new();
        top_stacks.insert(Suit::Club, vec![]);
        top_stacks.insert(Suit::Spade, vec![]);
        top_stacks.insert(Suit::Diamond, vec![]);
        top_stacks.insert(Suit::Heart, vec![]);
        let mut columns = vec![];
        for _i in 0..COLUMNS
        {
            columns.push(vec![]);
        }

        let mut adding = true;
        while adding
        {
            adding = false;
            for i in 0..COLUMNS
            {
                if columns[i].len() < i + 1
                { 
                    let mut card = deck.draw();
                    if columns[i].len() < i { card.set_visible(false) }
                    columns[i].push(card); 
                    adding = true;

                }
            }
        }

        Klondike { deck, top_stacks, columns, discard: vec![] }
    }

    pub fn play()
    {
        let mut game = Klondike::new();

        while !game.win()
        {
            game.handle_input();
        }
    }

    fn get_move_input(&self, toggle: bool) -> Option<MoveCommand>
    {

        let get_move_position = |min, max| {
            println!("Select a number between {} and {}", min, max);
            let position_i: usize = read!("{}\n");

            position_i - 1
        };

        let positions = ["Top", "Stack", "Columns", "Back"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&positions)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();

            match selection {
            Some(index) => 
            {
                println!("+ -------------------- + ");
                match positions[index] {
                    "Top" => return Some(MoveCommand { location: Position::Top, i: 1 }),
                    "Stack" => return Some(MoveCommand { location: Position::Stack, i: 1 }),
                    "Columns" => return Some(MoveCommand { location: Position::Columns, i: get_move_position(1,7) }),
                    "Back" => return None,
                    _ => {
                        println!("Invalid selection");
                        return self.get_move_input(toggle);
                    }
                };
            },
            None => {
                println!("Invalid selection");
                return self.get_move_input(toggle);
            }
        }
    }

    fn is_move_valid(&mut self, from: MoveCommand, to: MoveCommand) -> Result<bool, Box<dyn Error>>
    {
        let move_card = match from.location
        {
            Position::Columns => self.columns.get(from.i).unwrap().last().unwrap(),
            Position::Stack => self.deck.top_card().unwrap(),
            Position::Top => return Ok(false)
        };
        let opposite_suit = if move_card.color() { Suit::Club } else { Suit::Diamond };
        let to_card = match to.location
        {
            Position::Columns => self.columns[to.i].last(),
            Position::Top => self.top_stacks.get(&move_card.suit).unwrap().last(),
            Position::Stack => return Ok(false)
        };
        match to.location {
            Position::Stack => return Ok(false),
            Position::Columns => {
                match to_card {
                    Some(c) => {
                        return Ok((move_card.color() != c.color()) && (move_card.rank == c.rank - 1))
                    },
                    None => Ok(move_card.rank == 13)
                }
            },
            Position::Top => {
                match to_card {
                    Some(c) => {
                        return Ok((move_card.color() == c.color()) && (move_card.rank == c.rank - 1))
                    },
                    None => Ok(move_card.rank == 14)
                }
                
            }
        }
    }

    fn handle_move(&mut self) -> Result<(), String>
    {
        let from = match self.get_move_input(true) { Some(e) => e, None => return Err(String::from("Going back"))};
        let to = match self.get_move_input(false) { Some(e) => e, None => return Err(String::from("Going back"))};
    
        match self.is_move_valid(from, to)
        {
            Ok(v) => if !v { return Err(String::from("Unable to make that move")) },
            Err(e) => return Err(String::from("Unable to make that move"))
        };

        let move_card = match from.location
        {
            Position::Columns => {
                let card = self.columns[from.i].pop().unwrap();
                if let Some(next) = self.columns.get_mut(from.i).unwrap().last_mut() {
                    next.set_visible(true);
                };
                card
            },
            Position::Stack => self.deck.draw(),
            _ => return Err(String::from("Unable to make that move"))
        };
        
        match to.location
        {
            Position::Columns => self.columns[to.i].push(move_card),
            Position::Top => self.top_stacks.get_mut(&move_card.suit).unwrap().push(move_card),
            _ => return Err(String::from("Unable to make that move"))
        };

        Ok(())
    }

    fn handle_draw(&mut self)
    {
        for _ in 0..3 {
            self.discard.push(self.deck.draw());
        }
    }
}

impl fmt::Display for Klondike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_column = self.columns.iter().map(|c| c.len()).max().unwrap();
        let spade_str = match self.top_stacks.get(&Suit::Spade).unwrap().last() { Some(e) => e.to_string(), None => String::from("---") };
        let club_str = match self.top_stacks.get(&Suit::Club).unwrap().last() { Some(e) => e.to_string(), None => String::from("---") };
        let diamond_str = match self.top_stacks.get(&Suit::Diamond).unwrap().last() { Some(e) => e.to_string(), None => String::from("---") };
        let heart_str = match self.top_stacks.get(&Suit::Heart).unwrap().last() { Some(e) => e.to_string(), None => String::from("---") };

        let draw_stack_str = match self.deck.top_card() { Some(e) => e.see().to_string(), None => String::from("---") };
        let mut lines = vec![
            format!("🃏 : {} Cards remaining", self.deck.size()),
            
            format!(
                "+ TOP: ---{}---{}---{}---{}--- +", 
                spade_str, club_str, diamond_str, heart_str
                ),
            format!("+ --- Stack: {} --- +", draw_stack_str)
            ];
        for i in 0..max_column
        {
            let strings: Vec<String> = self.columns.iter().map(|c| c.get(i)).map(|c| {
                match c {
                    Some(m) => format!("{}", m),
                    None => format!("   ")
                }
            }).collect();
            lines.push(strings.join(" | "));
        };

        lines.push(format!("+ ------------------- +"));
        write!(f, "{}", lines.join("\n"))
    }
}

impl Game for Klondike
{
    fn win(&self) -> bool
    {
        for c in &self.columns
        {
            if c.len() < 14 { return false }
        }
        
        true
    }

    fn handle_input(&mut self)
    {

        let commands = ["Display", "Move", "Draw"];
        println!("Select a command: ");

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&commands)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();

        match selection {
            Some(index) => 
            {
                println!("+ -------------------- + ");
                match commands[index] {
                    "Display" => println!("{}", self),
                    "Move" => {
                        self.handle_move().unwrap_or_else(|err| {
                            println!("{}", err);
                            return self.handle_input()
                        })
                    },
                    "Draw" => self.handle_draw(),
                    _ => {
                        println!("Invalid selection");
                        self.handle_input();
                    }
                }
            }
            None => println!("User did not select anything")
        }


    }
}