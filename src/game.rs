use super::Card;

use rand::{thread_rng, Rng};

#[derive(Debug)]


#[derive(Debug)]
pub struct Game<Name> {
    pub deck: Vec<Card>,
    pub pile: Vec<Card>,
    pub players: Vec<Player<Name>>,
}

impl<Name> Game<Name> {
    pub fn new() -> Self {
        let deck = (0..52).map(|n| Card::from(n)).collect::<Vec<_>>();
        Self {
            deck,
            pile: Vec::new(),
            players: Vec::new(),
        }
    }
    pub fn with_jokers() -> Self {
        let deck = (0..54).map(|n| Card::from(n)).collect::<Vec<_>>();
        Self {
            deck,
            pile: Vec::new(),
            players: Vec::new(),
        }
    }
    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.deck);
    }
    pub fn repack(&mut self) {
        let &mut Game{ref mut deck, ref mut players, ref mut pile} = self;

        deck.append(pile);
        for player in players {
            deck.append(&mut player.hand);
        }
    }
    pub fn add_player<N: Into<Name>>(&mut self, name: N) {
        self.players.push(Player {
            name: name.into(),
            hand: Vec::new(),
        });
    }
    pub fn deal_to_all(&mut self, hand_size: u8) {
        let &mut Game{ref mut deck, ref mut players, ..} = self;
        for _ in 0..hand_size {
            for player in players.iter_mut() {
                if let Some(card) = deck.pop() {
                    player.hand.push(card);
                } else {
                    panic!("No more cards in deck");
                }
            }
        }
    }
    pub fn draw(&mut self, player: usize) {
        let &mut Game{ref mut deck, ref mut players, ..} = self;
        if let Some(ref mut player) = players.get_mut(player) {
            if let Some(card) = deck.pop() {
                player.hand.push(card);
            } else {
                panic!("No more cards in deck");
            }
        } else {
            panic!("No such player");
        }
    }
    pub fn play(&mut self, player: usize, card: usize) {
        let &mut Game{ref mut pile, ref mut players, ..} = self;
        if let Some(ref mut player) = players.get_mut(player) {
            let card = player.hand.remove(card);
            pile.push(card);
        } else {
            panic!("No such player");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player<Name> {
    pub name: Name,
    pub hand: Vec<Card>,
}
