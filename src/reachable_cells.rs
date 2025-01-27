use crate::models::Direction;
use crate::{Board, Coord};
use crate::board_functions::{coord_in_direction, is_outside, same_coord};

#[derive(Clone, Copy)]
struct TestCell {
    visited: bool,
    obstacle: bool,
    tail_length: usize,
}

pub fn reachable_cells(board: &Board, test_point_or_null: Option<&Coord>) -> usize {
    if test_point_or_null.is_none() {
        return 0;
    }
    let test_point = test_point_or_null.unwrap();

    let mut grid: Vec<Vec<TestCell>> = vec![
        vec![
            TestCell {
                visited: false,
                obstacle: false,
                tail_length: 0,
            };
            board.height as usize
        ];
        board.width as usize
    ];

    for snake in &board.snakes {
        for (index, snake_part) in snake.body.iter().enumerate() {
            if snake.body.len() > 2 && index + 1 >= snake.body.len() {
                grid[snake_part.x as usize][snake_part.y as usize].tail_length = snake.body.len();
                grid[snake_part.x as usize][snake_part.y as usize].obstacle = true;
            }

            if same_coord(snake_part, test_point) {
                grid[snake_part.x as usize][snake_part.y as usize].obstacle = false;
            } else {
                if snake_part.x as usize >= grid.len() || snake_part.y as usize >= grid[0].len() {
                    println!("Snake part out of board, should not happen.");
                }
                grid[snake_part.x as usize][snake_part.y as usize].obstacle = true;
            }
        }
    }

    let num_visited = recursive(test_point, board, &mut grid);
    num_visited
}

fn recursive(point: &Coord, board: &Board, grid: &mut Vec<Vec<TestCell>>) -> usize {
    if is_outside(point, board) {
        return 0;
    }

    let grid_cell = &mut grid[point.x as usize][point.y as usize];
    if grid_cell.tail_length > 0 {
        return grid_cell.tail_length;
    }

    if grid_cell.visited || grid_cell.obstacle {
        return 0;
    }

    let mut num_visited = 1;
    grid_cell.visited = true;

    num_visited += recursive(&coord_in_direction(point, &Direction::Up), board, grid);
    num_visited += recursive(&coord_in_direction(point, &Direction::Down), board, grid);
    num_visited += recursive(&coord_in_direction(point, &Direction::Left), board, grid);
    num_visited += recursive(&coord_in_direction(point, &Direction::Right), board, grid);

    num_visited
}