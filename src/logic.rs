// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use serde_json::Value;
use serde_json::json;

use crate::board_functions::{coord_in_direction, is_outside};
use crate::models::{Direction, DirectionResult, Outcome};
use crate::{Battlesnake, Board, Game, GameState, MoveResponse};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

fn next_move(game_state: &GameState) -> MoveResponse {
    let head = &game_state.you.body[0];

    // Loop over all possible direction and evaluate it
    let possible_directions = vec![
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];

    // Map over all possible direction and return the DirectionResult
    let direction_results: Vec<DirectionResult> = possible_directions
        .into_iter()
        .map(|direction| {
            let next_coord = coord_in_direction(head, &direction);
            let is_out_of_bounds = is_outside(&next_coord, &game_state.board);
            // Check that you don't collide with any snake
            // Add more checks if needed

            let outcome = if is_out_of_bounds {
                Outcome::Dead
            } else {
                Outcome::Alive
            };

            // Collect data to use for sorting

            // Defer direction to its own variable
            DirectionResult {
                direction,
                outcome,
                other_data: 0,
            }
        })
        .collect();


    // Filter out all safe moves
    let safe_moves: Vec<DirectionResult> = direction_results
        .into_iter()
        .filter(|dir| dir.outcome == Outcome::Alive)
        .collect::<Vec<DirectionResult>>();

    if safe_moves.is_empty() {
        println!(
            "MOVE {}: No safe moves detected! Moving down",
            game_state.turn
        );
        return MoveResponse {
            direction: String::from("down"),
            shout: None,
        };
    }

    // Sort you safe moves any way you like
    let next_move = safe_moves.iter().max_by_key(|dir| dir.other_data).unwrap();

    // Filter out all moves that are equally good
    let best_move_score = next_move.other_data;
    let equally_good_moves = safe_moves
        .iter()
        .filter(|dir| dir.other_data == best_move_score)
        .collect::<Vec<&DirectionResult>>();

    let mut small_rng = SmallRng::from_entropy();
    // Choose a random best move
    let random_best_move = if !equally_good_moves.is_empty() {
        let index = small_rng.gen_range(0..equally_good_moves.len());
        &equally_good_moves[index]
    } else {
        panic!("No equally good moves available");
    };

    println!("MOVE {}: {}", game_state.turn, random_best_move.direction);
    MoveResponse {
        direction: random_best_move.direction.to_string().to_lowercase(),
        shout: None,
    }
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(game_state: &GameState) -> Value {
    let next_move = next_move(game_state);
    return json!({ "move": next_move.direction });
}
