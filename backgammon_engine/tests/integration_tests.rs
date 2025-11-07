use backgammon_engine::backgammonstate::{ STARTING_GAME_STATE, BackgammonState};
use backgammon_engine::backgammonstate::generate_possible_next_states;
use backgammon_engine::invariants::backgammonstate_invariant;

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
    let res = generate_possible_next_states(game_state, is_black, dice.clone());
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
    let res = generate_possible_next_states(game_state, is_black, dice.clone());
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
    let res = generate_possible_next_states(game_state, is_black, dice.clone());
    assert!(res.is_err());
}

// #[test]
// fn test_api_1() {
//     let game_state = BackgammonState {
//         board: [-1, -1, -1, -1, -1, -2, -1, -1, -1, -1, -1, -1, -2, 0, 0, 0, 0, 0, 2, 0, 3, 5, 1, 3],
//         white_caught: 0,
//         black_caught: 1,
//         black_bearing: false,
//         white_bearing: false,
//         ended: false,
//         black_outside: 0,
//         white_outside: 0,
//     };
//     let dice = vec![4,3];
//     let is_black = false;
//     let res = generate_possible_next_states(game_state, is_black, dice.clone());
//     println!({}, res.unwrap());
// }