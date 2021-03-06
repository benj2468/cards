#[path = "../utils/deck.rs"] mod deck;
#[path = "../games/game.rs"] mod game;

use game::Game;
use deck::{Stack, Card};
use std::collections::HashMap;
use text_io::read;
use std::fmt;


pub struct AcesUpGame 
{
    deck: Stack,
    columns: Vec<Vec<Card>>
}

impl AcesUpGame 
{
    pub fn new() -> AcesUpGame
    {
        let mut deck = Stack::new_deck_reverse(false);

        deck.shuffle();

        AcesUpGame
        {
            deck,
            columns: vec![vec![],vec![],vec![],vec![]]
        }
    }

    pub fn play()
    {
        let mut game = AcesUpGame::new();

        while game.deck.size() > 0
        {
            game.turn()
        }

        if game.win() 
        { 
            println!("You won 😀");
        }
        else 
        { 
            println!("You lost 😥");
        }
    }

    pub fn turn(&mut self)
    {
        for i in 0..4
        {
            let mut card = self.deck.draw();
            card.set_visible(true);
            self.columns[i].push(card);
        };

        self.handle_input();
    }

    fn clean(&mut self)
    {
        loop 
        {
            let mut tops = HashMap::new();
            let mut to_remove = vec![];
            for (i, top) in self.columns.iter().enumerate().filter_map(|(i,c)| 
                {
                    if c.len() > 0 { return Some((i,c.last().unwrap())) }
                    else { return None }
                })
            {
                let entry = tops.entry(top.suit).or_insert((top.rank, i));
                if entry.0 < top.rank
                {
                    to_remove.push(entry.1.clone());
                    *entry = (top.rank, i);
                }
                else if entry.0 > top.rank
                {
                    to_remove.push(i)
                }
            }
            if to_remove.len() == 0
            {
                break;
            }
            for i in to_remove
            {
                self.columns[i].pop();
            }
        }
    }

    fn try_move_aces(&mut self)
    {
        let mut last_move_dest: isize = -1;
        loop
        {
            let (last_open, last_ace) = self.can_move_positions();

            if last_move_dest == last_ace
            {
                break
            }
            else if last_open >= 0 && last_ace >= 0
            {
                let ace = self.columns[last_ace as usize].pop().unwrap();
                self.columns[last_open as usize].push(ace);
                last_move_dest = last_open;
            }
            else
            {
                break
            }
        }
    }

    // set toggle to true if getting a "from", set to false if getting a "to"
    fn get_move_input(&self, toggle: bool) -> usize
    {
        let from_to_str = if toggle { "from" } else { "to" };
        println!("Input a column to move {}: (1-4,L-R) ", from_to_str);
        let command: usize = read!("{}\n");
        let comm_index = command - 1;
        if comm_index <= 3
        && (( 
                toggle && self.columns[comm_index].len() >= 1 
            ) || (
                !toggle && self.columns[comm_index].len() == 0)
            )
        {
            return comm_index
        }
        
        println!("Invalid input, try again");
        return self.get_move_input(toggle)
    }

    fn handle_move(&mut self, from: usize, to: usize)
    {
        let card = self.columns[from].pop().unwrap();
        self.columns[to].push(card);
    }

    fn can_move_positions(&self) -> (isize, isize)
    {
        let mut last_open: isize = -1;
        let mut last_ace: isize = -1;
        
        self.columns.iter().map(|c| c.last()).enumerate().for_each(|(i,c)| {
            match c {
                Some(s) => if s.is_ace() { last_ace = i as isize; }
                None => last_open = i as isize
            }
        });

        (last_open, last_ace)
    }

    fn can_move(&self) -> bool
    {
        let (last_open, last_ace) = self.can_move_positions();
        println!("{}, {}", last_open, last_ace);
        last_open > -1 && last_ace > -1
    }
}

impl fmt::Display for AcesUpGame 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_column = self.columns.iter().map(|c| c.len()).max().unwrap();
        let mut lines = vec![
            format!("🃏 : {} Cards remaining", self.deck.size()),
            format!("+ ------------------- +")];
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

impl game::Game for AcesUpGame 
{
    fn win(&self) -> bool
    {
        for i in 0..4 
        {
            if !self.columns[i][0].is_ace() { return false }
            if self.columns[i].len() > 1 { return false }
        }

        return true
    }

    fn handle_input(&mut self)
    {
        println!("Input a command: (D: display board, M: move, C: clean, N: next) ");
        let command: String = read!("{}\n");
        let command = command.as_str();
        match command {
            "D" => {
                println!("{}", self);
                self.handle_input();
            },
            "M" => {
                if !self.can_move()
                {
                    println!("Sorry, you cannot move on the current board.");
                    self.handle_input();
                }
                let from = self.get_move_input(true);
                let to = self.get_move_input(false);
                self.handle_move(from, to);
                self.handle_input();
            },
            "C" => {
                self.clean();
                self.handle_input();
            }
            "N" => return,
            _ => {
                println!("Invalid command, try again");
                return self.handle_input()
            }
        }
    }
}

#[cfg(test)]
mod test 
{
    use super::*;
    use deck::{Suit};

    #[test]
    fn clean()
    {
        let mut game = AcesUpGame::new();

        game.columns = vec![
            vec![Card { suit: Suit::Club, rank: 13, visible: false }],
            vec![Card { suit: Suit::Club, rank: 12, visible: false }],
            vec![Card { suit: Suit::Spade, rank: 5, visible: false }, Card { suit: Suit::Club, rank: 11, visible: false }],
            vec![Card { suit: Suit::Spade, rank: 6, visible: false }, Card { suit: Suit::Spade, rank: 14, visible: false }]
        ];

        game.clean();

        assert_eq!(game.columns, vec![
            vec![Card { suit: Suit::Club, rank: 13, visible: false }], 
            vec![], 
            vec![], 
            vec![Card { suit: Suit::Spade, rank: 6, visible: false }, Card { suit: Suit::Spade, rank: 14, visible: false }]
            ]);
    }

    #[test]
    fn win_false()
    {
        let mut game = AcesUpGame::new();

        game.columns = vec![
            vec![Card { suit: Suit::Club, rank: 13, visible: false }],
            vec![Card { suit: Suit::Club, rank: 12, visible: false }],
            vec![Card { suit: Suit::Spade, rank: 5, visible: false }, Card { suit: Suit::Club, rank: 11, visible: false }],
            vec![Card { suit: Suit::Spade, rank: 6, visible: false }, Card { suit: Suit::Spade, rank: 14, visible: false }]
        ];

        assert_eq!(game.win(), false)
    }
}