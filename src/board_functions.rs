use crate::models::Direction;
use crate::{Board, Coord};

pub fn is_outside(coord: &Coord, board: &Board) -> bool {
    let signed_height = board.height as i32;
    coord.y < 0 || coord.x < 0 || coord.x >= board.width || coord.y >= signed_height
}

pub fn coord_in_direction(start: &Coord, direction: &Direction) -> Coord {
    match direction {
        Direction::Up => Coord {
            x: start.x,
            y: start.y + 1,
        },
        Direction::Right => Coord {
            x: start.x + 1,
            y: start.y,
        },
        Direction::Down => Coord {
            x: start.x,
            y: start.y - 1,
        },
        Direction::Left => Coord {
            x: start.x - 1,
            y: start.y,
        },
    }
}

pub fn is_snake_part(coord: &Coord, board: &Board) -> bool {
    board.snakes.iter().any(|snake| {
        snake
            .body
            .iter()
            .any(|body_part| same_coord(body_part, &coord))
    })
}

pub fn same_coord(coord1: &Coord, coord2: &Coord) -> bool {
    coord1.x == coord2.x && coord1.y == coord2.y
}

pub fn closest_food(head: &Coord, board: &Board) -> Option<Coord> {
    if board.food.is_empty() {
        return None;
    }
    let mut sorted_food = board.food.clone();
    sorted_food
        // sort by shortest distance using distance(a, head) - distance(b, head)
        .sort_by(|a, b| distance(a, head).partial_cmp(&distance(b, head)).unwrap());
    let closest_food = sorted_food[0].clone();
    Some(closest_food)
}

pub fn distance(coord1: &Coord, coord2: &Coord) -> i32 {
    (coord1.x - coord2.x).abs() + (coord2.y - coord1.y).abs()
}
