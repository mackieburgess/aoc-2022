use std::collections::VecDeque;
use std::cmp::{min, max};

// extravagant type definition, bool denotes whether something is on that point
type Board = VecDeque<VecDeque<bool>>;

fn fill_board(mut board: Board, x: usize, y: usize) -> Board {
    // fill required grid space with falses
    while board.len() <= y {
        board.push_back(VecDeque::new());
    }

    for line in board.iter_mut() {
        while line.len() <= x {
            line.push_back(false);
        }
    }

    return board;
}

fn draw_x(mut board: Board, y: usize, x1: usize, x2: usize) -> Board {
    let x_max = max(x1, x2);
    let x_min = min(x1, x2);

    board = fill_board(board, x_max, y);

    for x_val in x_min..=x_max {
        board[y][x_val] = true;
    }

    return board;
}

fn draw_y(mut board: Board, x: usize, y1: usize, y2: usize) -> Board {
    let y_max = max(y1, y2);
    let y_min = min(y1, y2);

    board = fill_board(board, x, y_max);

    for y_val in y_min..=y_max {
        board[y_val][x] = true;
    }

    return board;
}

fn draw_board() -> Board {
    // input parsing is always the most intensive component of AOC
    let commands: Vec<Vec<(usize, usize)>> = include_str!("../data/14.input")
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .filter_map(|pos| pos.split_once(','))
                .filter_map(|(x,y)| {
                    match (x.parse::<usize>().ok(), y.parse::<usize>().ok()) {
                        (Some(x), Some(y)) => Some((x, y)),
                        _ => None
                    }
                }).collect()
        }).collect();

    let mut board: Board = VecDeque::new();

    for line in commands.iter() {
        for window in line.windows(2) {
            match window {
                [(x1, y1), (x2, y2)] if y1 == y2 => board = draw_x(board, *y1, *x1, *x2),
                [(x1, y1), (x2, y2)] if x1 == x2 => board = draw_y(board, *x1, *y1, *y2),
                _ => ()
            }
        }
    }

    board.push_back(VecDeque::new());

    for _ in 0..board[0].len() {
        if let Some(back) = board.back_mut() {
            back.push_back(false);
        }
    }

    return board;
}

fn first_abyss_unit() -> usize {
    let mut cave_scan = draw_board();
    let (mut cur_x, mut cur_y) = (500,0);
    let mut sand_rested = 0;

    // first, place the initial block of sand
    cave_scan[cur_y][cur_x] = true;

    loop {
        if cur_y == cave_scan.len() - 1 {
            break
        }

        if !cave_scan[cur_y+1][cur_x] {
            // fall down
            cave_scan[cur_y][cur_x] = false;

            cur_y += 1;

            cave_scan[cur_y][cur_x] = true;
        } else if !cave_scan[cur_y+1][cur_x-1] {
            // fall down-left
            cave_scan[cur_y][cur_x] = false;

            cur_y += 1;
            cur_x -= 1;

            cave_scan[cur_y][cur_x] = true;
        } else if !cave_scan[cur_y+1][cur_x+1] {
            // fall down-left
            cave_scan[cur_y][cur_x] = false;

            cur_y += 1;
            cur_x += 1;

            cave_scan[cur_y][cur_x] = true;
        } else {
            // more sand
            cur_x = 500;
            cur_y = 0;

            cave_scan[cur_y][cur_x] = true;
            sand_rested += 1;
        }
    }

    return sand_rested;
}

fn main() {
    println!("part one: {}", first_abyss_unit());
}
