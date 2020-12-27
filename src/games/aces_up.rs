#[path = "../utils/deck.rs"] mod deck;
#[path = "../games/game.rs"] mod game;

use deck::{Stack, Card, Suit};
use std::collections::HashMap;
use text_io::read;
use game::Game;
use std::fmt;


pub struct AcesUpGame {
    deck: Stack,
    columns: Vec<Vec<Card>>
}

impl AcesUpGame {
    pub fn new() -> AcesUpGame
    {
        let deck = Stack::new_deck_reverse(false);

        AcesUpGame
        {
            deck,
            columns: vec![vec![],vec![],vec![],vec![]]
        }
    }

    pub fn start(&mut self) {
        self.deck.shuffle();

        self.play();
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
            let mut last_open: isize = -1;
            let mut last_ace: isize = -1;
            
            self.columns.iter().map(|c| c.last()).enumerate().for_each(|(i,c)| {
                match c {
                    Some(s) => if s.is_ace() { last_ace = i as isize; }
                    None => last_open = i as isize
                }
            });
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
        println!("Input a column to move from: (1-4,L-R) ");
        let command: usize = read!("{}\n");
        if command >= 1 
        && command <= 4 
        && (( toggle && self.columns[command].len() > 0) || (!toggle && self.columns[command].len() == 0))
        {
            return command
        }
        
        println!("Invalid input, try again");
        return self.get_move_input(toggle)
    }

    fn handle_move(&mut self, from: usize, to: usize)
    {
        let card = self.columns[from].pop().unwrap();
        self.columns[to].push(card);
    }
}

impl fmt::Display for AcesUpGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_column = self.columns.iter().map(|c| c.len()).max().unwrap();
        let mut lines = vec![];
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

        write!(f, "{}", lines.join("\n"))
    }
}

impl Game for AcesUpGame {
    fn play(&mut self) -> bool
    {
        self.for_each(|_| ());

        if self.win() { return true }
        else { return false }
    }

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

impl Iterator for AcesUpGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> 
    {
        for i in 0..4
        {
            self.columns[i].push(self.deck.draw())
        };

        self.handle_input();

        let size = self.deck.size();
        if size > 0 { return Some(size) }
        else { return None }
    }

    
}

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn clean()
    {
        let mut game = AcesUpGame::new();

        game.columns = vec![
            vec![Card { suit: Suit::Club, rank: 13 }],
            vec![Card { suit: Suit::Club, rank: 12 }],
            vec![Card { suit: Suit::Spade, rank: 5 }, Card { suit: Suit::Club, rank: 11 }],
            vec![Card { suit: Suit::Spade, rank: 6 }, Card { suit: Suit::Spade, rank: 14 }]
        ];

        game.clean();

        assert_eq!(game.columns, vec![
            vec![Card { suit: Suit::Club, rank: 13 }], 
            vec![], 
            vec![], 
            vec![Card { suit: Suit::Spade, rank: 6 }, Card { suit: Suit::Spade, rank: 14 }]
            ]);
    }

    #[test]
    fn win_false()
    {
        let mut game = AcesUpGame::new();

        game.columns = vec![
            vec![Card { suit: Suit::Club, rank: 13 }],
            vec![Card { suit: Suit::Club, rank: 12 }],
            vec![Card { suit: Suit::Spade, rank: 5 }, Card { suit: Suit::Club, rank: 11 }],
            vec![Card { suit: Suit::Spade, rank: 6 }, Card { suit: Suit::Spade, rank: 14 }]
        ];

        assert_eq!(game.win(), false)
    }
}