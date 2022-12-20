use std::collections::VecDeque;

type Board = VecDeque<Vec<bool>>;

fn solidify_board(mut board: Board, shape: &Vec<Vec<bool>>, cur: (usize, usize)) -> Board {
    for y in 0..shape.len() {
        for x in 0..shape[y].len() {
            board[cur.1 + y][cur.0 + x] |= shape[y][x];
        }
    }

    board
}

fn tetris() -> usize {
    let mut board: Board = VecDeque::new();
    // all shapes
    let shape_iter = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true,  true, true],
            vec![false, true, false]
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true,  true,  true]
        ],
        vec![
            vec![true],
            vec![true],
            vec![true],
            vec![true]
        ],
        vec![
            vec![true, true],
            vec![true, true]
        ]
    ];

    let mut shape_iter = shape_iter.iter().cycle();

    let mut instructions = include_str!("../data/17.input").chars().cycle();

    // repeat 2022 times
    for _ in 0..2022 {
        let shape = shape_iter.next().expect("impossible: cycle iterator");

        // add headway for the rock to fall
        for _ in 0..shape.len() + 3 {
            board.push_front(vec![false, false, false, false, false, false, false]);
        }

        // top left point on the cursor
        let mut cursor = (2, 0);

        let mut stopped = false;

        while !stopped {
            // jet (left/right) movement, most of this is checking for walls
            match instructions.next() {
                Some('>') => {
                    if cursor.0 + shape[0].len() < 7 {
                        let mut immovable = false;
                        for y in 0..shape.len() {
                            for x in 0..shape[y].len() {
                                if shape[y][x] {
                                    if board[cursor.1 + y][cursor.0 + x + 1] {
                                        immovable = true;
                                    }
                                }
                            }
                        }

                        if !immovable { cursor.0 += 1 }
                    }
                },
                Some('<') => {
                    if cursor.0 > 0 {
                        let mut immovable = false;
                        for y in 0..shape.len() {
                            for x in 0..shape[y].len() {
                                if shape[y][x] {
                                    if board[cursor.1 + y][cursor.0 + x - 1] {
                                        immovable = true;
                                    }
                                }
                            }
                        }

                        if !immovable { cursor.0 -= 1 }
                    }
                },
                _ => continue // stupid "end of file" char I think
            };

            // check for bottom of board
            for y in 0..shape.len() {
                for x in 0..shape[y].len() {
                    if shape[y][x] && !stopped {
                        // if item below the shape value is a board value
                        // or the bottom of the board is reached
                        if (cursor.1 + y + 1) == board.len() ||
                            board[cursor.1 + y + 1][cursor.0 + x]
                        {
                            // solidify the board and move onto the next rock
                            board = solidify_board(board, shape, cursor);
                            stopped = true;
                        }
                    }
                }
            }

            // move rock down
            cursor.1 += 1;
        }

        // remove all empty layers
        while board[0] == vec![false, false, false, false, false, false, false] {
            board.pop_front();
        }

    }

    // for y in 0..board.len() {
    //     for x in 0..board[y].len() {
    //         if board[y][x] { print!("#") } else { print!(" ") }
    //     }
    //     println!();
    // }

    return board.len();
}

fn main() {
    println!("part one: {}", tetris());
}
