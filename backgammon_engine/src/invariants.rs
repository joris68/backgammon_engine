
use crate::backgammonstate::BackgammonState;


fn correct_amount_stones_black(game_state: &BackgammonState) -> bool {
    let mut stones = 0;
    for (_, x) in game_state.board.iter().enumerate() {
        if *x > 0 {
            stones += *x;
        }
    }
    (game_state.black_caught + stones + game_state.black_outside) == 15
}

fn correct_amount_stones_white(game_state: &BackgammonState) -> bool {
    let mut stones = 0;
    for (_, x) in game_state.board.iter().enumerate() {
        if *x < 0 {
            stones += x.abs();
        }
    }
    (game_state.white_caught + stones + game_state.white_outside) == 15
}

fn outside_and_bearing_black(game_state: &BackgammonState) -> bool {
    if game_state.black_caught > 0 && game_state.black_bearing {
        return false;
    }
    true
}

fn outside_and_bearing_white(game_state : &BackgammonState) -> bool {
    if game_state.white_caught > 0 && game_state.white_bearing {
        return false;
    }
    true
}
/// Implements several state invariants for the game state object. This is mostly used for testing. And for input validation.
pub fn backgammonstate_invariant(
    game_state: &BackgammonState
) -> Result<(), Box<dyn std::error::Error>> {
    let mut error = false;
    if !correct_amount_stones_black(game_state) {
        error = true;
    }
    if !correct_amount_stones_white(game_state) {
        error = true;
    }
    if !outside_and_bearing_black(game_state) {
        error = true;
    }
    if !outside_and_bearing_white(game_state) {
        error = true;
    }
    if error {
        return Err(Box::new(std::fmt::Error));
    }
    Ok(())
}