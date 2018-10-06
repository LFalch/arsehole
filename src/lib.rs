extern crate rand;

pub mod game;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs = 0,
    Spades = 1,
    Hearts = 2,
    Diamonds = 3
}

impl Suit {
    pub fn is_red(&self) -> bool {
        use Suit::*;
        match self {
            Hearts | Diamonds => true,
            Clubs | Spades => false,
        }
    }
    pub fn is_black(&self) -> bool {
        use Suit::*;
        match self {
            Clubs | Spades => true,
            Hearts | Diamonds => false,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Ace = 0,
    N2 = 1,
    N3 = 2,
    N4 = 3,
    N5 = 4,
    N6 = 5,
    N7 = 6,
    N8 = 7,
    N9 = 8,
    N10 = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Joker = 13,
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl From<u8> for Card {
    fn from(n: u8) -> Self {
        let (rank, suit);
        if n < 52 {
            rank = n % 13;
            suit = n / 13;
        } else {
            rank = 13;
            suit = 3-(n-52);
        }

        let suit = match suit {
            0 => Suit::Clubs,
            1 => Suit::Spades,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => panic!("No such suit")
        };
        let rank = match rank {
            0 => Rank::Ace,
            1 => Rank::N2,
            2 => Rank::N3,
            3 => Rank::N4,
            4 => Rank::N5,
            5 => Rank::N6,
            6 => Rank::N7,
            7 => Rank::N8,
            8 => Rank::N9,
            9 => Rank::N10,
            10 => Rank::Jack,
            11 => Rank::Queen,
            12 => Rank::King,
            13 => Rank::Joker,
            _ => panic!("No such rank")
        };

        Self {
            rank,
            suit
        }
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        let Card{rank, suit} = self;
        if let Rank::Joker = rank {
            52 + suit as u8
        } else {
            suit as u8 * 13 + rank as u8
        }
    }
}

use std::cmp::Ordering;

trait Ranker {
    fn beats(c1: &Card, c2: &Card) -> Ordering {
        match (c1.rank as u8, c2.rank as u8) {
            (0, 13) => Ordering::Less,
            (13, 0) => Ordering::Greater,
            (0, _) => Ordering::Greater,
            (_, 0) => Ordering::Less,
            (n, n1) => n.cmp(&n1),
        }
     }
}

struct Krig;

impl Ranker for Krig {}

struct Roevhul;

fn roevhul_rank(r: u8) -> u8 {
    if r < 2 {
        r + 13
    } else if r == 10 || r == 13 {
        20
    } else {
        r
    }
}

impl Ranker for Roevhul {
    fn beats(c1: &Card, c2: &Card) -> Ordering {
        roevhul_rank(c1.rank as u8).cmp(&roevhul_rank(c2.rank as u8))
    }
}
