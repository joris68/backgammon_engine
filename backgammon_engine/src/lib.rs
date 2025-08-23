#[cfg(test)]
mod tests {
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
        let result = is_black_bearing(&mut initial_state);
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
        let result = is_black_bearing(&mut initial_state);
        assert_eq!(result, true);
    }

    #[test]
    fn is_white_bearing_test1() {
        let mut initial_state = BackgammonState {
            board: [
                1, 12, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = is_white_bearing(&mut initial_state);
        assert_eq!(result, true);
    }

    #[test]
    fn is_white_bearing_test2() {
        let mut initial_state = BackgammonState {
            board: [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 1, 4,
            ],
            white_caught: 0,
            black_caught: 0,
            black_bearing: false,
            white_bearing: false,
            ended: false,
            black_outside: 0,
            white_outside: 0,
        };
        let result = is_white_bearing(&mut initial_state);
        assert_eq!(result, false);
    }

    #[test]
    fn update_board_black_1() {
        let mut initial_state = STARTING_GAME_STATE.clone();
        let move_black = BackGammonMove::new(Player::Black, 0, 2);
        let new_state = update_board_move_black(&initial_state, move_black);
        assert_eq!(new_state.board[2] == 1);
    }
}

/////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct BackgammonState {
    board: [i32; 24],
    white_caught: i32,
    black_caught: i32,
    black_bearing: bool,
    white_bearing: bool,
    ended: bool,
    black_outside: i32,
    white_outside: i32,
}

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

enum Player {
    White,
    Black,
}

struct BackGammonMove {
    player: Player,
    from: i32,
    to: i32,
}

impl BackGammonMove {
    fn new(player: Player, from: i32, to: i32) -> BackGammonMove {
        BackGammonMove { player, from, to }
    }
}

/*
Black: is positive numbers
White negative numbers
*/

fn update_board_move_black(
    game_state: &mut BackgammonState,
    move_black: BackGammonMove,
) -> BackgammonState {
    let mut new_game_state: BackgammonState = game_state.clone();

    if move_black.from == -1 {
        new_game_state.black_caught -= 1;
        new_game_state.board[move_black.to] += 1;
        new_game_state.black_bearing = is_black_bearing(&new_game_state);
        new_game_state.white_bearing = is_white_bearing(&new_game_state);
        return new_game_state;
    }

    new_game_state.board[move_black.from] -= 1;

    if game_state.board[move_black] == -1 {
        new_game_state.white_caught += 1;
        new_game_state.board[move_black] = 1;
        new_game_state.black_bearing = is_black_bearing(&new_game_state);
        new_game_state.white_bearing = is_white_bearing(&new_game_state);
        return new_game_state;
    }

    if new_game_state.board[move_black.to] >= 0 {
        new_game_state.board[move_black.to] += 1;
    }

    new_game_state.black_bearing = is_black_bearing(&new_game_state);
    new_game_state.white_bearing = is_white_bearing(&new_game_state);
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
    (sum_white_stones + game_state.white_outside) == 15
}
