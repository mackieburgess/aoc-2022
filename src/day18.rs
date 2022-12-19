use std::collections::{VecDeque,HashSet};

type Board = Vec<Vec<Vec<bool>>>;

struct Pos {
    x: usize,
    y: usize,
    z: usize
}

impl Pos {
    fn from(x: usize, y: usize, z: usize) -> Self {
        Pos {x, y, z}
    }
}

fn apply_to_board(mut board: Board, position: &Pos) -> Board {
    // applies falses to make sure the board is big enough,
    // then sets the given position to false

    // x
    while position.x >= board.len() {
        board.push(vec![]);
    }

    // y
    for sub_board in board.iter_mut() {
        while position.y >= sub_board.len() {
            sub_board.push(vec![]);
        }

        // z
        for sub_sub_board in sub_board.iter_mut() {
            while position.z >= sub_sub_board.len() {
                sub_sub_board.push(false);
            }
        }
    }

    board[position.x][position.y][position.z] = true;

    return board;
}

fn blob_at(board: &Board, x: usize, y: usize, z: usize) -> bool {
    // returns true if the value exists and is lava
    if let Some(sub_board) = board.get(x) {
        if let Some(sub_sub_board) = sub_board.get(y) {
            if let Some(value) = sub_sub_board.get(z) {
                return *value;
            }
        }
    }

    false
}

fn blob_surface_area(board: &Board, pos: &Pos) -> usize {
    let mut acc = 0;

    // this is pretty unsightly, but it does the job
    // just checks if each neighbour to pos is lava or not
    if pos.x == 0 || !blob_at(board, pos.x-1, pos.y, pos.z) { acc += 1 }
    if !blob_at(board, pos.x+1, pos.y, pos.z) { acc += 1 }

    if pos.y == 0 || !blob_at(board, pos.x, pos.y-1, pos.z) { acc += 1 }
    if !blob_at(board, pos.x, pos.y+1, pos.z) { acc += 1 }

    if pos.z == 0 || !blob_at(board, pos.x, pos.y, pos.z-1) { acc += 1 }
    if !blob_at(board, pos.x, pos.y, pos.z+1) { acc += 1 }

    return acc;
}

fn is_stuck(board: &Board, x: usize, y: usize, z: usize) -> bool {
    let mut paths: VecDeque<(usize, usize, usize)> = VecDeque::from([(x, y, z)]);
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

    loop {
        let path = paths.pop_front();

        if let Some((x, y, z)) = path {
            // check for edge reached
            if x.min(y.min(z)) == 0 ||
                x == board.len() - 1 ||
                y == board[x].len() - 1 ||
                z == board[x][y].len() - 1
            {
                return false;
            }

            // I spent ages creating an amazing optimisation that speeds up the code drastically,
            // only to realise that it was a bug that was slowing me down ._.
            visited.insert((x, y, z));

            // yeah this sucks
            // just add a value if it isn't set to be checked, has been checked, or is a wall
            if !board[x-1][y][z] && !visited.contains(&(x-1, y, z))&& !paths.contains(&(x-1, y, z)) { paths.push_back((x-1, y, z)) }
            if !board[x+1][y][z] && !visited.contains(&(x+1, y, z))&& !paths.contains(&(x+1, y, z)) { paths.push_back((x+1, y, z)) }
            if !board[x][y-1][z] && !visited.contains(&(x, y-1, z))&& !paths.contains(&(x, y-1, z)) { paths.push_back((x, y-1, z)) }
            if !board[x][y+1][z] && !visited.contains(&(x, y+1, z))&& !paths.contains(&(x, y+1, z)) { paths.push_back((x, y+1, z)) }
            if !board[x][y][z-1] && !visited.contains(&(x, y, z-1))&& !paths.contains(&(x, y, z-1)) { paths.push_back((x, y, z-1)) }
            if !board[x][y][z+1] && !visited.contains(&(x, y, z+1))&& !paths.contains(&(x, y, z+1)) { paths.push_back((x, y, z+1)) }
        } else {
            // if paths is exhausted, we know that this value is inside a blob
            return true;
        }
    }
}

fn fill_interior(mut board: Board) -> Board {

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            for z in 0..board[x][y].len() {
                if !board[x][y][z] {
                    if is_stuck(&board, x, y, z) {
                        board[x][y][z] = true;
                    }
                }
            }
        }
    }

    return board;
}

fn connected_blobs(fancy: bool) -> usize 
{
    // 3D array to represent x, y, z
    let mut board: Board = vec![];

    let positions = include_str!("../data/18.input")
        .lines()
        .filter_map(|line| {
            let line = line
                .split(',')
                .filter_map(|val| val.parse::<usize>().ok())
                // .map(|val| val - 1)
                .collect::<Vec<usize>>();

            if line.len() == 3 {
                Some(Pos::from(line[0], line[1], line[2]))
            } else {
                None
            }

        }).collect::<Vec<Pos>>();

    // build board
    for position in positions.iter() {
        board = apply_to_board(board, position);
    }

    // apply interior filling for part two
    if fancy { board = fill_interior(board); }

    let mut surface_area = 0;

    // count up surface area
    for position in positions.iter() {
        surface_area += blob_surface_area(&board, position)
    }

    return surface_area;

}

fn main() {
    println!("part one: {}", connected_blobs(false));
    println!("part two: {}", connected_blobs(true));
}
