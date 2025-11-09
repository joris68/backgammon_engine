use backgammon_engine::backgammonstate::{ STARTING_GAME_STATE, BackgammonState, gen_poss_next_states};

#[test]
fn test_invalid_state_input() {
    let game_state = BackgammonState {
        board: [
            1, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2,
        ],
        white_caught: 0,
        black_caught: 0,
        black_bearing: false,
        white_bearing: false,
        ended: false,
        black_outside: 0,
        white_outside: 0,
    };
    let dice = vec![1,2];
    let is_black = true;
    let res = gen_poss_next_states(game_state, is_black, dice.clone());
    assert!(res.is_err());
}

#[test]
fn test_invalid_dice_input() {
    let game_state = BackgammonState {
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
    let dice = vec![1,7];
    let is_black = true;
    let res = gen_poss_next_states(game_state, is_black, dice.clone());
    assert!(res.is_err());
}

#[test]
fn test_invalid_dice_input_2() {
    let game_state = BackgammonState {
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
    let dice = vec![4,4,4,3];
    let is_black = true;
    let res = gen_poss_next_states(game_state, is_black, dice.clone());
    assert!(res.is_err());
}