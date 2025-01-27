use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use server_nano::Server;
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

/* 
#[get("/")]
fn handle_index() -> Json<Value> {
    Json(logic::info())
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    let response = logic::get_move(&move_req);

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}*/

fn main() {
    let mut app = Server::new();

    app.get("/", |_, res| {
        let info = logic::info();
        res.json(&info)
    });

    app.post("/start", |req, res| {
        let json_body = req.json_body().unwrap();
        // Convert from Value to GameState
        let start_req: GameState = serde_json::from_value(json_body).unwrap();
        logic::start(
            &start_req.game,
            &start_req.turn,
            &start_req.board,
            &start_req.you,
        );
        res.status_code(200, "OK");
        res.send("OK")
    });

    app.post("/move", |req, res| {
        let json_body = req.json_body().unwrap();
        let game_state: GameState = serde_json::from_value(json_body).unwrap();
        let move_response = logic::get_move(&game_state);
        res.json(&move_response)
    });

    app.post("/end", |req, res| {
        let json_body = req.json_body().unwrap();
        // Convert from Value to GameState
        let start_req: GameState = serde_json::from_value(json_body).unwrap();
        logic::end(
            &start_req.game,
            &start_req.turn,
            &start_req.board,
            &start_req.you,
        );
        res.status_code(200, "OK");
        res.send("OK")
    });

    // port from env::var("PORT") or 8000
    let port: String = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = format!("{}:{}", "0.0.0.0", port);
    app.listen(&addr).unwrap();
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
