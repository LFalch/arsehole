use super::Card;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientPacket {
    HelloIm(String),
    Play(usize),
    Pass,
    BeginGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerPacket {
    Welcome,
    Hand(Vec<Card>),
    NewPlayer(String),
    OtherPlayers(Vec<(String, usize)>),
    PlayerHandUpdate(usize, usize),
    PileUpdate(Card, usize),
    DeckUpdate(usize),
    Turn(usize),
}
