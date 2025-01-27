use rouille::try_or_400;
use rouille::Server;
use rouille::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod board_functions;
mod logic;
mod models;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: i32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Coord>,
    head: Coord,
    length: i32,
    latency: String,
    shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: i32,
    board: Board,
    you: Battlesnake,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MoveResponse {
    #[serde(rename = "move")]
    direction: String,
    shout: Option<String>,
}

fn main() {
  
    let port: String = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = format!("{}:{}", "0.0.0.0", port);

    let server = Server::new(addr, |request| {
        if request.method() == "GET" {
            if request.url() == "/" {
                return Response::json(&logic::info());
            } else {
                return Response::text("Unkown GET request");
            }
        } else if request.method() == "POST" {
            if request.url() == "/start" {
                let json_body = try_or_400!(rouille::input::json_input(request));
                // Convert from Value to GameState
                let start_req: GameState = serde_json::from_value(json_body).unwrap();
                logic::start(
                    &start_req.game,
                    &start_req.turn,
                    &start_req.board,
                    &start_req.you,
                );
                return Response::text("OK");
            } else if request.url() == "/move" {
                let json_body = try_or_400!(rouille::input::json_input(request));
                let game_state: GameState = serde_json::from_value(json_body).unwrap();
                let move_response = logic::get_move(&game_state);
                return Response::json(&move_response);
            } else if request.url() == "/end" {
                let json_body = try_or_400!(rouille::input::json_input(request));
                // Convert from Value to GameState
                let start_req: GameState = serde_json::from_value(json_body).unwrap();
                logic::end(
                    &start_req.game,
                    &start_req.turn,
                    &start_req.board,
                    &start_req.you,
                );
                return Response::text("OK");
            } else {
                return Response::text("Unkown POST request");
            }
        } else {
            Response::text("Unspported method")
        }
    }).unwrap();
    println!("Listening on {:?}", server.server_addr());
    server.run();
}
/*
#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
         */
