use crate::backgammonmove::{BackgammonMove, Player};
use std::fmt;
use std::collections::HashSet;
use crate::invariants::backgammonstate_invariant;


#[cfg(test)]
mod test_black {
    use super::*;

    #[test]
    fn test_is_black_bearing_1() {
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
        let result = is_black_bearing(&mut initial_state);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_black_bearing_2() {
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
        let result = is_black_bearing(&mut initial_state);
        assert_eq!(result, true);
    }

    #[test]
    fn test_apply_move_black_1() {
        let initial_state = STARTING_GAME_STATE.clone();
        let move_black = BackgammonMove::new(Player::Black, 0, 2);
        let new_state = apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[2], 1);
    }

    #[test]
    fn test_apply_move_bearing_black_1() {
        let initial_state = BackgammonState {
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
        let new_state = apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[23], 3);
        assert_eq!(new_state.black_outside, 1);
    }

    #[test]
    fn test_apply_move_bearing_black_2() {
        let initial_state = BackgammonState {
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
        let new_state = apply_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[23], 5);
        assert_eq!(new_state.black_outside, 0);
    }

    #[test]
    fn test_generate_black_moves_1() {
        let mut game_state = BackgammonState {
            board: [
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let dice = vec![1];
        let states = generate_black_game_states(&mut game_state, vec![1]);
       // println!("{:?}", game_state);
        assert_eq!(states.len() , 1);
        assert_eq!(states[0].board[2],  1);
        assert_eq!(states[0].board[1],  0);
    }

    #[test]
    fn test_generate_black_moves_2() {
        let mut game_state = BackgammonState {
            board: [
                0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let dice = vec![1];
        let states = generate_black_game_states(&mut game_state, vec![1]);
        assert_eq!(states.len() , 2);
    }

    #[test]
    fn test_valid_moves_1() {
        let game_state = BackgammonState {
            board: [
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 1, 2);
        let valid = valid_move_black(&game_state, &m);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_2() {
        let game_state = BackgammonState {
            board: [
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 1,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, -1, 2);
        let valid = valid_move_black(&game_state, &m);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_3() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 21, 24);
        let valid = valid_move_black(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_4() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 21, 24);
        let valid = valid_move_black(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_5() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 22, 20);
        let valid = valid_move_black(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_6() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, -1, 2);
        let valid = valid_move_black(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_7() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 5, 3);
        let valid = valid_move_black(&game_state, &m);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_bearing_1() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 22, 25);
        let valid = valid_move_black(&game_state, &m);
        assert_eq!(valid , true);
        let after_state = apply_move_black(&game_state, m);
        println!("{:?}", game_state);
        assert_eq!(after_state.black_outside , 1);
        assert_eq!(after_state.black_bearing , true);
    }

    #[test]
    fn test_valid_moves_bearing_2() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::Black, 22, 23);
        let valid = valid_move_black(&game_state, &m);
        assert_eq!(valid , true);
        let after_state = apply_move_black(&game_state, m);
        println!("{:?}", game_state);
        assert_eq!(after_state.black_outside , 0);
        assert_eq!(after_state.black_bearing , true);
    }
}

#[cfg(test)]
mod test_white {
    use super::*;

    #[test]
    fn is_white_bearing_test2() {
        let mut initial_state = BackgammonState {
            board: [
                -1, -12, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = is_white_bearing(&mut initial_state);
        assert_eq!(result, false);
    }

    #[test]
    fn test_to_string_method() {

        let initial_state = BackgammonState {
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
        println!("{}", initial_state);
    }


    #[test]
    fn test_get_unused_items_1() {
        let dice = vec![1,2];
        let used_dice = vec![1];
        let unused_dice = get_unused_dice(&dice, &used_dice);

        assert_eq!(unused_dice.len(), 1);
        assert_eq!(unused_dice[0], 2);
    }

    #[test]
    fn test_get_unused_items_2() {
        let dice = vec![1,1,1,1];
        let used_dice = vec![1];
        let unused_dice = get_unused_dice(&dice, &used_dice);

        assert_eq!(unused_dice.len(), 3);
        assert_eq!(unused_dice, vec![1,1,1]);
    }

     #[test]
    fn test_valid_moves_1() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 22, 21);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_2() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 1,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 24, 22);
        let valid = valid_move_white(&game_state, &m);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_3() {
        let game_state = BackgammonState {
            board: [
                0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: true,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 2, -1);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , true);
    }

    #[test]
    fn test_valid_moves_4() {
        let game_state = BackgammonState {
            board: [
                0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 2, -1);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_5() {
        let game_state = BackgammonState {
            board: [
                0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 2, 4);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_6() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 24, 2);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_valid_moves_7() {
        let game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 2, 5);
        let valid = valid_move_white(&game_state, &m);
        println!("{:?}", game_state);
        assert_eq!(valid , false);
    }

    #[test]
    fn test_generate_white_moves_1() {
        let mut game_state = BackgammonState {
            board: [
                0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let dice = vec![1];
        let states = generate_white_game_states(&mut game_state, vec![1]);
        println!("{:?}", game_state);
        assert_eq!(states.len() , 1);
        assert_eq!(states[0].board[0],  -1);
        assert_eq!(states[0].board[1],  0);
    }

    #[test]
    fn test_generate_white_moves_2() {
        let mut game_state = BackgammonState {
            board: [
                0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let dice = vec![1];
        let states = generate_white_game_states(&mut game_state, vec![1]);
        println!("{:?}", game_state);
        assert_eq!(states.len() , 2);
    }

    #[test]
    fn test_valid_moves_8() {
        let state = BackgammonState {
                board: [
                    2,
                    0,
                    0,
                    0,
                    0,
                    -5,
                    0,
                    -3,
                    0,
                    0,
                    0,
                    3,
                    -5,
                    0,
                    0,
                    1,
                    3,
                    1,
                    5,
                    0,
                    0,
                    0,
                    0,
                    -2,
                ],
                white_caught: 0,
                black_caught: 0,
                black_bearing: false,
                white_bearing: false,
                ended: false,
                black_outside: 0,
                white_outside: 0,
        };
        
        let m = BackgammonMove::new(Player::White, 5, 0);
        let valid = valid_move_white(&state, &m);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_valid_dice_1() {
        let game_state = STARTING_GAME_STATE;
        let dice = vec![1,2,3,4];
        let res = gen_poss_next_states(&game_state, true, &dice);
        assert!(res.is_err());
    }

    #[test]
    fn test_valid_dice_2() {
        let game_state = STARTING_GAME_STATE;
        let dice = vec![1,2];
        let res = gen_poss_next_states(&game_state, true, &dice);
        assert!(res.is_ok());
    }

    #[test]
    fn test_valid_dice_3() {
        let game_state = STARTING_GAME_STATE;
        let dice = vec![1,7];
        let res = gen_poss_next_states(&game_state, true, &dice);
        assert!(res.is_err());
    }

    #[test]
    fn test_valid_dice_4() {
        let game_state = STARTING_GAME_STATE;
        let dice = vec![4,4,4,4];
        let res = gen_poss_next_states(&game_state, true, &dice);
        assert!(res.is_ok());
    }

    #[test]
    fn test_bearing_white() {
        let mut game_state = BackgammonState {
            board: [
                0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let a : bool = is_white_bearing(&game_state);
        assert_eq!(a, true);
    }

     #[test]
    fn test_valid_moves_bearing_1() {
        let game_state = BackgammonState {
            board: [
                0, 0, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: true,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 2, 1);
        let valid = valid_move_white(&game_state, &m);
        assert_eq!(valid , true);
        let after_state = apply_move_white(&game_state, m);
        println!("{:?}", game_state);
        assert_eq!(after_state.white_outside , 0);
        assert_eq!(after_state.white_bearing , true);
    }

    #[test]
    fn test_valid_moves_bearing_2() {
        let game_state = BackgammonState {
            board: [
                0, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: true,
            white_bearing: true,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let m = BackgammonMove::new(Player::White, 1, -2);
        let valid = valid_move_white(&game_state, &m);
        assert_eq!(valid , true);
        let after_state = apply_move_white(&game_state, m);
        println!("{:?}", game_state);
        assert_eq!(after_state.white_outside , 1);
        assert_eq!(after_state.white_bearing , true);
    }

}


const LAST_FIELD: i32 = 23;
const FIRST_FIELD: i32 = 0;
/// The starting game start for each game of Backgammon.
pub const STARTING_GAME_STATE: BackgammonState = BackgammonState {
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


/// The object that captures the current state of the game.
/// The board array contains most of the information for the game state. To reduce
/// recurring calculations some information are "cached" with booleans inside the object.
#[derive(Clone, Hash, PartialEq, Eq, Copy, Debug)]
pub struct BackgammonState {
    /// Black checkers are represented by positive integers, white checker by negative integer.
    /// If a field is empty it is indecated by a zero.
    pub board: [i32; 24],
    /// the amount of checkers from the white side that have been beaton by black and have yet not been inserted back into the game
    pub white_caught: i32,
    /// the amount of black checkers that have been beaton by black and have yet not been inserted back into the game
    pub black_caught: i32,
    /// indicates that all checkers are in the home board of black and can be removed from the board
    pub black_bearing: bool,
    /// indicates that all checkers are in the home board of white and can be removed from the board
    pub white_bearing: bool,
    /// if the game has ended or not
    pub ended: bool,
    /// the amount of black checkers already removed from the board
    pub black_outside: i32,
    /// the amount of black checkers already removed from the board
    pub white_outside: i32,
}


impl fmt::Display for BackgammonState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Board: {:?}, White Caught: {}, Black Caught: {}, Black Bearing: {}, White Bearing: {}, Ended: {}, Black Outside: {}, White Outside: {}",
            self.board,
            self.white_caught,
            self.black_caught,
            self.black_bearing,
            self.white_bearing,
            self.ended,
            self.black_outside,
            self.white_outside
        )
    }
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
}

    fn generate_black_moves(
        game_state: &BackgammonState, 
        dice: Vec<i32>
    ) -> Vec<BackgammonState> {
        let mut dice_used = Vec::new();
        let mut new_game_state = game_state.clone();
        if new_game_state.black_caught > 0 {
            let (updated_game_state, used_dice) = insert_stones_black(&new_game_state, &dice);
            new_game_state = updated_game_state;
            dice_used = used_dice;
        }

        if dice_used.len() == dice.len() ||  new_game_state.black_caught > 0 {
            return vec![new_game_state.clone()];
        }

        return generate_black_game_states(&mut new_game_state, get_unused_dice(&dice, &dice_used));
    } 
    
    fn generate_black_game_states( game_state:  &mut BackgammonState, dice : Vec<i32> ) -> Vec<BackgammonState> {
        let mut all_states: HashSet<BackgammonState> = HashSet::new();
        fn backtrack_states(inner_state : BackgammonState, dice : Vec<i32>, all_states: &mut HashSet<BackgammonState>) -> () {
            if inner_state.ended {
                if !all_states.contains(&inner_state) {
                    all_states.insert(inner_state.clone());
                }
                return;
            }
            if dice.len() == 0 {
                if !all_states.contains(&inner_state) {
                    all_states.insert(inner_state.clone());
                }
                return;
            }
            for (index, &value) in inner_state.board.iter().enumerate() {
                if value > 0 {
                    let poss_moves : Vec<(BackgammonMove, usize)> = dice
                                    .iter()
                                    .enumerate()
                                    .map(|(i, d)| {
                                        let mm = BackgammonMove::new(Player::Black, index as i32, index as i32 + *d);
                                        (mm, i)
                                    })
                                    .filter(|m| valid_move_black(&inner_state, &m.0))
                                    .collect();             
                    for m in poss_moves.iter() {
                        let mut dice_copy = dice.clone();
                        dice_copy.remove(m.1);
                        let new_state = apply_move_black(&inner_state, m.0);
                        backtrack_states(new_state.clone(), dice_copy, all_states)
                    }
                }
            }
        }
        backtrack_states(game_state.clone(), dice, &mut all_states);
        let all_poss_states : Vec<BackgammonState> = all_states.iter().cloned().collect();
        if all_poss_states.len() == 0 {
            return vec![game_state.clone()];
        } else {
            return all_poss_states
        }
    }

    fn get_unused_dice(dice : &Vec<i32>, used_dice : &Vec<i32> ) -> Vec<i32> {

        let mut game_dice = dice.clone();
        for &d_used in used_dice {
             if let Some(index) = dice.iter().position(|&x| x == d_used) {
                game_dice.remove(index);
            }
        }
        return game_dice;
    }


    fn generate_moves_white(
        game_state: &BackgammonState, 
        dice: Vec<i32>
    ) -> Vec<BackgammonState> {
        let mut dice_used = Vec::new();
        let mut new_game_state = game_state.clone();

        if new_game_state.white_caught > 0 {
            let (updated_game_state, used_dice) = insert_stones_white(&new_game_state, &dice);
            new_game_state = updated_game_state;
            dice_used = used_dice;
        }

        if dice_used.len() == dice.len() ||  new_game_state.white_caught > 0 {
            return vec![new_game_state.clone()];
        }

        return generate_white_game_states(&mut new_game_state, get_unused_dice(&dice, &dice_used));
    } 

    fn generate_white_game_states(game_state:  &mut BackgammonState, dice : Vec<i32> ) -> Vec<BackgammonState> {
        let mut all_states: HashSet<BackgammonState> = HashSet::new();
        fn backtrack_states(inner_state : BackgammonState, dice : Vec<i32>, all_states: &mut HashSet<BackgammonState>) -> () {
            if inner_state.ended {
                if !all_states.contains(&inner_state) {
                    all_states.insert(inner_state.clone());
                }
                return;
            }
            if dice.len() == 0 {
                if !all_states.contains(&inner_state) {
                    all_states.insert(inner_state.clone());
                }
                return;
            }
            for (index, &value) in inner_state.board.iter().enumerate() {
                if value < 0 {
                    let poss_moves : Vec<(BackgammonMove, usize)> = dice
                                    .iter()
                                    .enumerate()
                                    .map(|(i, d)| {
                                        let mm = BackgammonMove::new(Player::White, index as i32, index as i32 - *d); 
                                        (mm, i)
                                    })
                                    .filter(|m| valid_move_white(&inner_state, &m.0))
                                    .collect();
                          
                    for m in poss_moves.iter() {
                        let mut dice_copy = dice.clone();
                        dice_copy.remove(m.1);
                        let new_state = apply_move_white(&inner_state, m.0);
                        backtrack_states(new_state.clone(), dice_copy, all_states)
                    }
                }
            }
        }
        backtrack_states(game_state.clone(), dice, &mut all_states);
        let all_poss_states : Vec<BackgammonState> = all_states.iter().cloned().collect();
        if all_poss_states.len() == 0 {
            return vec![game_state.clone()];
        } else {
            return all_poss_states
        }
    }

    fn is_state_valid(game_state : &BackgammonState) -> bool {
        match backgammonstate_invariant(game_state) {
            Ok(_)=> true,
            Err(_) => false,
        }
    }

    fn four_dice(dice : &Vec<i32>) -> bool {
        if dice.len() == 2 {
            return true
        } else {
            return dice.windows(2).all(|w| w[0] == w[1])
        }
    }

    fn correct_dice_given(dice : &Vec<i32>) -> bool {
        let count = dice.iter().filter(|&&x| x >= 0 && x <= 6).count();
        return count == dice.len() && (dice.len() == 2 || dice.len() == 4) && four_dice(&dice)
    }
    /// Implements the state transition function for the game of Backgammon.
    /// function: `f(s, d) -> s'` for the input game state `s`. And the random vector of dice `d`.  
    /// It gives back all possible next states from the given game state with the thrown dice.
    /// The resulting vector always has len > 0, even when no possible moves are possible with the current dice and 
    /// and game state, the input game state will be given back. The vector does not implement a specific ordering.
    /// The function returns an error when the game state input to the function is invalid or the vector of dice given to the function are invalid.
    /// # Simulate an Example game
    ///  This simulates an example game in which black starts and the same dice are used for every move and the strategy of taking the first entry of the next state array
    ///  for both sides. 
    /// ```
    /// use backgammon_engine::backgammonstate::{STARTING_GAME_STATE, BackgammonState, gen_poss_next_states};
    ///
    ///fn pick_next_move(next_poss_states : &Vec<BackgammonState>) -> BackgammonState {
    ///      return next_poss_states[0];
    ///}
    ///
    ///fn generate_dice() -> Vec<i32> {
    ///   return vec![1,2]
    ///}
    ///
    ///fn simulate_game() {
    ///      let mut current_game_state = STARTING_GAME_STATE;
    ///      let mut is_black = true;
    ///      while !current_game_state.ended {
    ///              let dice = generate_dice();
    ///              let next_poss_states = gen_poss_next_states(&current_game_state, is_black, &dice)
    ///                                     .expect("Failed to generate possible next states");
    ///               current_game_state = pick_next_move(&next_poss_states);
    ///               is_black = !is_black;
    ///      }  
    /// }
    ///
    /// fn main() {
    ///      simulate_game();
    /// }
    /// ```
    /// More detailed documentation can be found in the readme.md in the corresponding Github project.
    pub fn gen_poss_next_states(game_state : &BackgammonState, is_black : bool, dice : &Vec<i32>) -> Result<Vec<BackgammonState>, Box<dyn std::error::Error>> {
        if !is_state_valid(game_state) {
            return Err("Invalid game state given".into())
        }
        if !correct_dice_given(dice) {
            return Err("Invalid Dice given".into())
        }
        if is_black {
            Ok (generate_black_moves(&game_state, dice.clone()))
        } else {
            Ok (generate_moves_white(&game_state, dice.clone()))
        }
    }  

    fn insert_stones_black(
        game_state: &BackgammonState,
        dice: &Vec<i32>,
    ) -> (BackgammonState, Vec<i32>) {
        let mut dice_used = Vec::new();
        let mut new_game_state = game_state.clone(); 
        for &d in dice {
            if game_state.black_caught > 0 && game_state.board[(d - 1) as usize] >= 0 {
            let move_black =  BackgammonMove::new(Player::Black, -1, d - 1);
                let game_state_after_apply = apply_move_black(
                    &new_game_state,
                    move_black
                );
                new_game_state = game_state_after_apply;
                dice_used.push(d);
                if new_game_state.black_caught == 0 {
                    break;
                }
            }
        }
        (new_game_state, dice_used)
    }


    fn insert_stones_white(
    game_state: &BackgammonState,
    dice: &Vec<i32>,
    ) -> (BackgammonState, Vec<i32>) {
        let mut dice_used = Vec::new();
        let mut new_game_state = game_state.clone();
        for &d in dice {
            if game_state.white_caught > 0 && game_state.board[(23 - d - 1) as usize] <= 0 {
                let move_white =  BackgammonMove::new(Player::White, 24, 23 - d - 1);
                let game_state_after_apply = apply_move_white(
                    &new_game_state,
                    move_white
                );
                new_game_state = game_state_after_apply;
                dice_used.push(d);
                if new_game_state.white_caught == 0 {
                    break;
                }
            }
        }
        (new_game_state, dice_used)
    }

    fn apply_move_black(
        game_state: &BackgammonState,
        move_black: BackgammonMove,
    ) -> BackgammonState {
        let mut new_game_state: BackgammonState = game_state.clone();

        if move_black.from == -1 {
            new_game_state.black_caught -= 1;
            new_game_state.board[move_black.to as usize] += 1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state);
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }

        new_game_state.board[move_black.from as usize] -= 1;

        if new_game_state.black_bearing && move_black.to > 23 {
            new_game_state.black_outside += 1;
            if new_game_state.black_outside == 15 {
                new_game_state.ended = true
            }
            return new_game_state;
        }

        if game_state.board[move_black.to as usize] == -1 {
            new_game_state.white_caught += 1;
            new_game_state.board[move_black.to as usize] = 1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state); // might not be necessary to calculate that every time
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }

        if new_game_state.board[move_black.to as usize] >= 0 {
            new_game_state.board[move_black.to as usize] += 1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state);
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }
        // should not happen
        return new_game_state;
    }

    fn apply_move_white(
        game_state: &BackgammonState,
        move_white: BackgammonMove,
    ) -> BackgammonState {
        let mut new_game_state: BackgammonState = game_state.clone();

        if move_white.from == 24 {
            new_game_state.white_caught -= 1;
            new_game_state.board[move_white.to as usize] -= 1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state);
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }
        new_game_state.board[move_white.from as usize] += 1;

        if new_game_state.white_bearing && move_white.to < 0 {
            new_game_state.white_outside += 1;
            if new_game_state.white_outside == 15 {
                new_game_state.ended = true
            }
            return new_game_state;
        }

        if game_state.board[move_white.to as usize] == 1 {
            new_game_state.black_caught += 1;
            new_game_state.board[move_white.to as usize] = -1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state);
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }

        if new_game_state.board[move_white.to as usize] <= 0 {
            new_game_state.board[move_white.to as usize] -= 1;
            new_game_state.black_bearing = is_black_bearing(&new_game_state);
            new_game_state.white_bearing = is_white_bearing(&new_game_state);
            return new_game_state;
        }
        // should not happen
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

    fn valid_move_black(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        mv.player == Player::Black && mv.from < mv.to
        && is_inbounds_black(state, mv)
        && moves_right_black(state, mv)
    }

    fn moves_right_black(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        if state.black_bearing {
            mv.to > 23 || valid_to_field_black(state, mv)
        } else {
            valid_to_field_black(state, mv)
        }
    }

    fn valid_to_field_black(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        let field = state.board[mv.to as usize];
        field == 0 || field == -1 || field >= 1
    }

    fn is_inbounds_black(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        if state.black_bearing {
            mv.from >= 18
        } else if state.black_caught > 0 {
            mv.from == -1
        } else {
            in_bounce(mv)
        }
    }
    
    fn valid_move_white(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        mv.player == Player::White && mv.to < mv.from
        && is_inbounds_white(state, mv)
        && moves_right_white(state, mv)
    }

    fn moves_right_white(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        if state.white_bearing {
            mv.to < 0 || valid_to_field_white(state, mv)
        } else {
            valid_to_field_white(state, mv)
        }
    }

    fn valid_to_field_white(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        let field = state.board[mv.to as usize];
        field <= -1 || field == 0 || field == 1
    }

    fn is_inbounds_white(state: &BackgammonState, mv: &BackgammonMove) -> bool {
        if state.white_bearing {
            mv.from <= 6
        } else if state.white_caught > 0 {
            mv.from == 24
        } else {
            in_bounce(mv)
        }
    }

    fn in_bounce(mv: &BackgammonMove) -> bool {
        mv.from >= FIRST_FIELD && mv.from <= LAST_FIELD
        && mv.to >= FIRST_FIELD && mv.to <= LAST_FIELD
    }