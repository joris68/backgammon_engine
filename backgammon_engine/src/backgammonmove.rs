#[derive(PartialEq)]
pub enum Player {
    White,
    Black,
}

pub struct BackgammonMove {
    pub player: Player,
    pub from: i32,
    pub to: i32,
}

impl BackgammonMove {
    pub fn new(player: Player, from: i32, to: i32) -> BackgammonMove {
        BackgammonMove { player, from, to }
    }
}

impl PartialEq for BackgammonMove {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.player == other.player
    }
}
