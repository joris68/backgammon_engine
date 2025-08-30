use crate::backgammonmove::{BackgammonMove, Player};

#[cfg(test)]
mod test_Black {
    use super::*;

    #[test]
    fn is_black_bearing_test1() {
        let mut initial_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = BackgammonState::is_black_bearing(&mut initial_state);
        assert_eq!(result, false);
    }

    #[test]
    fn is_black_bearing_test2() {
        let mut initial_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = BackgammonState::is_black_bearing(&mut initial_state);
        assert_eq!(result, true);
    }

    #[test]
    fn apply_move_black_1() {
        let mut initial_state = STARTING_GAME_STATE.clone();
        let move_black = BackgammonMove::new(Player::Black, 0, 2);
        let new_state = BackgammonState::apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[2], 1);
    }

    fn apply_move_bearing_black_1() {
        let mut initial_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let move_black = BackgammonMove::new(Player::Black, 23, 25);
        let new_state = BackgammonState::apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[23], 3);
        assert_eq!(new_state.black_outside, 1);
    }

    fn apply_move_bearing_black_2() {
        let mut initial_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let move_black = BackgammonMove::new(Player::Black, 21, 23);
        let new_state = BackgammonState::apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[23], 5);
        assert_eq!(new_state.black_outside, 0);
    }
}

#[cfg(test)]
mod test_White {
    use super::*;

    #[test]
    fn is_white_bearing_test2() {
        let mut initial_state = BackgammonState {
            board: [
                -1, -12, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = BackgammonState::is_white_bearing(&mut initial_state);
        assert_eq!(result, false);
    }

    // #[test]
    // fn test_bearing_moves() {
    //     let mut initial_state = BackgammonState {
    //         board: [
    //             0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
    //         ],
    //         white_caught: 0,
    //         black_caught: 0,
    //         black_bearing: false,
    //         white_bearing: false,
    //         ended: false,
    //         black_outside: 0,
    //         white_outside: 0,
    //     };
    //     let dice = 3;
    //     let moves = BackgammonState::possible_bearing_moves_white(&initial_state, dice);
    //     assert_eq!(moves.len(), 1)
    // }
}

const LAST_FIELD_EXTENDED: i32 = 24;
const FIRST_FIELD_EXTENDED: i32 = -1;

const LAST_FIELD: i32 = 23;
const FIRST_FIELD: i32 = 0;
const BLACK_START_BEARING: i32 = 18;
const WHITE_START_BEARING: i32 = 6;

const STARTING_GAME_STATE: BackgammonState = BackgammonState {
    board: [
        2, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2,
    ],
    white_caught: 0,
    black_caught: 0,
    black_bearing: false,
    white_bearing: false,
    ended: false,
    black_outside: 0,
    white_outside: 0,
};

#[derive(Clone)]
pub struct BackgammonState {
    pub board: [i32; 24],
    pub white_caught: i32,
    pub black_caught: i32,
    pub black_bearing: bool,
    pub white_bearing: bool,
    pub ended: bool,
    pub black_outside: i32,
    pub white_outside: i32,
}

impl BackgammonState {
    fn new(
        board: [i32; 24],
        white_caught: i32,
        black_caught: i32,
        black_bearing: bool,
        white_bearing: bool,
        ended: bool,
        black_outside: i32,
        white_outside: i32,
    ) -> BackgammonState {
        BackgammonState {
            board,
            white_caught,
            black_caught,
            black_bearing,
            white_bearing,
            ended,
            black_outside,
            white_outside,
        }
    }

    fn apply_move_black(
        game_state: &BackgammonState,
        move_black: BackgammonMove,
    ) -> BackgammonState {
        let mut new_game_state: BackgammonState = game_state.clone();

        if move_black.from == -1 {
            new_game_state.black_caught -= 1;
            new_game_state.board[move_black.to as usize] += 1;
            new_game_state.black_bearing = Self::is_black_bearing(&new_game_state);
            new_game_state.white_bearing = Self::is_white_bearing(&new_game_state);
            return new_game_state;
        }

        new_game_state.board[move_black.from as usize] -= 1;

        if new_game_state.black_bearing && move_black.to > 23 {
            new_game_state.black_outside += 1;
            return new_game_state;
        }

        if game_state.board[move_black.to as usize] == -1 {
            new_game_state.white_caught += 1;
            new_game_state.board[move_black.from as usize] = 1;
            new_game_state.black_bearing = Self::is_black_bearing(&new_game_state); // might not be necessary to calculate that every time
            new_game_state.white_bearing = Self::is_white_bearing(&new_game_state);
            return new_game_state;
        }

        if new_game_state.board[move_black.to as usize] >= 0 {
            new_game_state.board[move_black.to as usize] += 1;
        }

        new_game_state.black_bearing = Self::is_black_bearing(&new_game_state);
        new_game_state.white_bearing = Self::is_white_bearing(&new_game_state);
        return new_game_state;
    }

    fn is_black_bearing(game_state: &BackgammonState) -> bool {
        if game_state.black_caught > 0 {
            return false;
        }
        let board_slice: &[i32] = &game_state.board[18..24];
        let sum_black_stones: i32 = board_slice.iter().filter(|&&x| x > 0).sum();
        (sum_black_stones + game_state.black_outside) == 15
    }

    fn is_white_bearing(game_state: &BackgammonState) -> bool {
        if game_state.white_caught > 0 {
            return false;
        }
        let board_slice: &[i32] = &game_state.board[0..6];
        let sum_white_stones: i32 = board_slice.iter().filter(|&&x| x < 0).sum();
        (-sum_white_stones + game_state.white_outside) == 15
    }

    fn valid_move_black(game_state: &BackgammonState, move_black: &BackgammonMove) -> bool {
        return Self::is_inbounce_black(game_state, move_black)
            && Self::moves_right_black(game_state, move_black);
    }

    fn moves_right_black(game_state: &BackgammonState, move_black: &BackgammonMove) -> bool {
        if game_state.black_bearing {
            move_black.to > 23 || Self::valid_to_field_black(game_state, move_black)
        } else {
            Self::valid_to_field_black(game_state, move_black)
        }
    }

    fn valid_to_field_black(game_state: &BackgammonState, move_black: &BackgammonMove) -> bool {
        game_state.board[move_black.from as usize] >= 0
            && game_state.board[move_black.to as usize] <= LAST_FIELD
            && (game_state.board[move_black.to as usize] >= 1
                || game_state.board[move_black.to as usize] == 0
                || game_state.board[move_black.to as usize] == -1)
    }

    fn is_inbounce_black(game_state: &BackgammonState, move_black: &BackgammonMove) -> bool {
        if game_state.black_bearing {
            move_black.from >= 18
        } else {
            Self::in_bounce(move_black)
        }
    }

    fn in_bounce(b_move: &BackgammonMove) -> bool {
        (b_move.from >= FIRST_FIELD && b_move.from <= LAST_FIELD)
            && (b_move.to >= FIRST_FIELD && b_move.to <= LAST_FIELD)
    }

    fn valid_move_white(game_state: &BackgammonState, move_white: &BackgammonMove) -> bool {
        return Self::is_inbounce_white(game_state, move_white);
        //&& Self::moves_rigth_white(game_state, move_white);
    }

    fn moves_right_white(game_state: &BackgammonState, move_white: &BackgammonMove) -> bool {
        if game_state.white_bearing {
            move_white.to < 0 ||
        }
    }

    fn is_inbounce_white(game_state: &BackgammonState, move_white: &BackgammonMove) -> bool {
        if game_state.white_bearing {
            move_white.from <= 6
        } else {
            Self::in_bounce(move_white)
        }
    }

    fn possible_bearing_moves_black(
        game_state: &BackgammonState,
        dice: i32,
    ) -> Vec<BackgammonMove> {
        return (18..23)
            .filter(|&x| game_state.board[x] > 0)
            .filter(|&x| {
                let m = BackgammonMove::new(Player::Black, x as i32, x as i32 + dice);
                Self::valid_move_black(game_state, &m)
            })
            .map(|x| BackgammonMove::new(Player::Black, x as i32, x as i32 + dice))
            .collect();
    }
}
