use backgammon_engine::backgammonstate::{ STARTING_GAME_STATE, BackgammonState};
use backgammon_engine::backgammonstate::generate_possible_next_states;
use rand_distr::{Distribution, Uniform};
use rand::{Rng, thread_rng};
use backgammon_engine::invariants::backgammonstate_invariant;
use log::{info, error};


fn generate_dice(rng: &mut impl Rng) -> Vec<i32> {
    let die_dist = Uniform::new_inclusive(1, 6).unwrap();
    let d1 = die_dist.sample(rng);
    let d2 = die_dist.sample(rng);

    if d1 == d2 {
        vec![d1, d1, d1, d1]
    } else {
        vec![d1, d2]
    }
}

fn pick_next_move(next_poss_states : &Vec<BackgammonState>, rng: &mut impl Rng) -> BackgammonState {
    let die_dist = Uniform::new_inclusive(0, next_poss_states.len() - 1).unwrap();
    let index = die_dist.sample(rng);
    return next_poss_states[index];
}   

#[test]
fn test_public_api() {
    let number_of_games = 100;
    let mut is_black = true;
    let mut rng = thread_rng();
    let mut current_game_state = STARTING_GAME_STATE;

    for x in 0..number_of_games {
        while !current_game_state.ended {
            let dice = generate_dice(&mut rng);
            let next_poss_states = generate_possible_next_states(current_game_state, is_black, dice.clone())
                            .expect("Failed to generate possible next states");

            for state in &next_poss_states {
                let res = backgammonstate_invariant(state);
                match res {
                    Ok(_) => {},
                    Err(err_msg) => panic!(
                        "Error: Backgammon state invariant violation detected.\n\
                        generate_moves function has produced an invalid game state.\n\
                        Details of the error: {}\n\n\
                        State prior to move generation:\n{:#?}\n\n\
                        Generated invalid state:\n{:#?}\n\n\
                        Dice values for this move:\n{:#?}\n\n\
                        Current turn: {:#?}",
                        err_msg,
                        current_game_state,
                        state,
                        dice.clone(),
                        is_black
                    ),
                }
            }
            current_game_state = pick_next_move(&next_poss_states, &mut rng);
            is_black = !is_black;
        }
        if x % 100 == 0 {
            println!("So many games already done: {}", x);

        }
    }
    println!("integration tests finished successfully");
}