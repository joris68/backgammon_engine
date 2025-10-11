use std::fmt;
use log::info;
use std::error::Error;

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

fn outside_and_bearing(game_state: &BackgammonState) -> bool {
    if game_state.black_caught > 0 && game_state.black_bearing {
        return false;
    }

    if game_state.white_caught > 0 && game_state.white_bearing {
        return false;
    }

    true
}

fn backgammonstate_invariant(
    game_state_before: &BackgammonState,
    game_state_after: &BackgammonState,
) -> Result<(), Error> {
    if !_black_number_stones(game_state_after) {
        info!(
            "Game state before: {:?} Invalid Game State: {:?}",
            game_state_before, game_state_after
        );
        return Err;
    }

    if !_white_number_stones(game_state_after) {
        info!(
            "Game state before: {:?} Invalid Game State: {:?}",
            game_state_before, game_state_after
        );
        return Err;
    }

    if !_outside_and_bearing(game_state_after) {
        info!(
            "Game state before: {:?} Invalid Game State: {:?}",
            game_state_before, game_state_after
        );
        return Error
    }

    Ok(())
}