use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
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

impl fmt::Display for BackgammonMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: from {} to {}",
            match self.player {
                Player::White => "White",
                Player::Black => "Black",
            },
            self.from,
            self.to
        )
    }
}


