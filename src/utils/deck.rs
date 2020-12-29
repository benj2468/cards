use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

const MAX_CARD_RANK: u8 = 14;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
    Joker
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_str = match self {
            Suit::Heart => "❤",
            Suit::Club => "♣️",
            Suit::Spade => "♠️",
            Suit::Diamond => "♦️",
            Suit::Joker => "🃏"
        };
        write!(f, "{}", suit_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
    pub visible: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Result<Card, String> {
        if rank > MAX_CARD_RANK { return Err(format!("Cannot create a card with that rank, max rank is 13, {} provided", rank))}
        
        Ok(Card { suit, rank, visible: true })
    }

    pub fn new_visible(suit: Suit, rank: u8) -> Result<Card, String> {
        let mut card = Card::new(suit, rank).unwrap();

        card.visible = true;

        Ok(card)
    }

    pub fn is_ace(&self) -> bool
    {
        self.rank == MAX_CARD_RANK
    }

    pub fn set_visible(&mut self, new_val: bool)
    {
        self.visible = new_val;
    }

    pub fn see(mut self) -> Card
    {
        self.visible = true;

        self
    }

    pub fn hide(mut self) -> Card
    {
        self.visible = false;
        
        self
    }
    pub fn color(&self) -> bool
    {
        match self.suit
        {
            Suit::Club => true,
            Suit::Spade => true,
            Suit::Diamond => false,
            Suit::Heart => false,
            Suit::Joker => {
                if self.rank == 0 { return true }
                else { return false };
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.visible
        {
            return write!(f, "? ?")
        }
        let rank_str: String = match self.rank {
            13 => String::from(" K"),
            12 => String::from(" Q"),
            11 => String::from(" J"),
            14 => String::from(" A"),
            10 => String::from("10"),
            _ => format!(" {}", self.rank)
        };
        write!(f, "{}{}", rank_str, self.suit)
    }
}

pub struct Stack {
    cards: Vec<Card>,
    top: usize,
}

impl Stack {
    pub fn new_deck(with_joker: bool) -> Stack {
        let mut cards = vec![];
        for i in 2..15 {
            cards.push(Card::new(Suit::Club, i).unwrap());
            cards.push(Card::new(Suit::Spade, i).unwrap());
            cards.push(Card::new(Suit::Heart, i).unwrap());
            cards.push(Card::new(Suit::Diamond, i).unwrap());
        }
        if with_joker {
            cards.push(Card::new(Suit::Joker, 1).unwrap());
            cards.push(Card::new(Suit::Joker, 2).unwrap());
        }

        Stack { cards, top: 0 }
    }

    pub fn new_deck_reverse(with_joker: bool) -> Stack {
        let mut cards = vec![];
        for i in 0..4 {
            let suit_type = match i {
                0 => Suit::Club,
                1 => Suit::Spade,
                2 => Suit::Heart,
                3 => Suit::Diamond,
                _ => Suit::Joker
            };
            for rank in 2..15 {
                cards.push(Card::new(suit_type, rank).unwrap());
            };
        };
        if with_joker {
            cards.push(Card::new(Suit::Joker, 1).unwrap());
            cards.push(Card::new(Suit::Joker, 2).unwrap());
        }
        
        Stack { cards, top: 0 }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self) -> Card {
        let card = self.cards[self.top];

        self.top += 1;

        card
    }

    pub fn size(&self) -> usize {
        self.cards.len() - self.top
    }

    pub fn deal(&mut self, count: usize) -> Vec<Card> {
        let mut cards = vec![];

        for _ in 0..count {
            let card = self.draw();
            cards.push(card)
        }

        cards
    }

    pub fn top_card(&self) -> Option<&Card> {
        self.cards.get(self.top)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn new_card() {
        let card = Card::new_visible(Suit::Heart, 3);
        assert_eq!(format!("{}", card.unwrap()), " 3❤");
    }

    fn new_hidden_card() {
        let card = Card::new(Suit::Heart, 3);
        assert_eq!(format!("{}", card.unwrap()), "? ?");
    }

    #[test]
    fn deck() {
        let deck = Stack::new_deck(false);

        assert_eq!(deck.cards.len(), 52)
    }

    #[test]
    fn deck_with_joker() {
        let deck = Stack::new_deck(true);

        assert_eq!(deck.cards.len(), 54)
    }

    #[test]
    fn invalid_card() {
        let card = Card::new(Suit::Club, 15);
        match card {
            Err(_) => assert_eq!(true, true),
            Ok(_) => assert_eq!(false, true)
        }
    }

    #[test]
    fn shuffle() {
        let mut deck = Stack::new_deck(false);

        let first_before = deck.cards[0];

        deck.shuffle();

        let first_after = deck.cards[0];

        assert_ne!(first_before, first_after);
    }

    #[test]
    fn draw() {
        let mut deck = Stack::new_deck(false);

        deck.draw();

        let size_after = deck.size();

        assert_eq!(size_after, 51);
    }

    #[test]
    fn deal() {
        let mut deck = Stack::new_deck(false);

        deck.shuffle();

        let hand = deck.deal(5);

        assert_eq!(hand.len(), 5);
        assert_eq!(deck.size(), 47)
    }
}

