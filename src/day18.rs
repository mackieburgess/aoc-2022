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
    // x
    while position.x >= board.len() {
        board.push(vec![]);
    }

    // y
    for sub_board in board.iter_mut() {
        while position.y >= sub_board.len() {
            sub_board.push(vec![]);
        }

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
    if let Some(sub_board) = board.get(x) {
        if let Some(sub_sub_board) = sub_board.get(y) {
            if let Some(value) = sub_sub_board.get(z) {
                return *value;
            }
        }
    }

    false
}

fn blob_surface_area(board: &Board, position: &Pos) -> usize {
    let mut acc = 0;

    // lazy
    if position.x == 0 || !blob_at(board, position.x-1, position.y, position.z) { acc += 1 }
    if !blob_at(board, position.x+1, position.y, position.z) { acc += 1 }

    if position.y == 0 || !blob_at(board, position.x, position.y-1, position.z) { acc += 1 }
    if !blob_at(board, position.x, position.y+1, position.z) { acc += 1 }

    if position.z == 0 || !blob_at(board, position.x, position.y, position.z-1) { acc += 1 }
    if !blob_at(board, position.x, position.y, position.z+1) { acc += 1 }

    return acc;
}

fn connected_blobs() -> usize {
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

    let mut surface_area = 0;

    for position in positions.iter() {
        surface_area += blob_surface_area(&board, position)
    }

    return surface_area;

}

fn main() {
    println!("part one: {}", connected_blobs());
}
