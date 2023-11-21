use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 30;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];
    // let mut attack_counts = [[0; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_4(&mut board, 0, 0, 0);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

fn dump_board(board: &mut [[char; NUM_COLS]; NUM_ROWS]) {
    println!("");
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            print!("{position} ", position = board[row][col]);
        }
        println!("")
    }
}

fn position_onboard(row: i32, col: i32) -> bool {
    if row >= 0 && col >= 0 && row < INUM_ROWS && col < INUM_COLS {
        return true;
    }
    return false;
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut row = r0;
    let mut col = c0;
    let mut queen_found = false;
    while position_onboard(row, col) {
        if board[row as usize][col as usize] == 'Q' {
            if queen_found {
                return false;
            } else {
                queen_found = true;
            }
        }
        row += dr;
        col += dc;
    }
    return true;
}

// Return true if the board is legal.
fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS], r0: i32, c0: i32) -> bool {
    return series_is_legal(board, r0, c0, 0, 1)
        && series_is_legal(board, r0, c0, 0, -1)
        && series_is_legal(board, r0, c0, 1, 1)
        && series_is_legal(board, r0, c0, 1, -1)
        && series_is_legal(board, r0, c0, -1, 1)
        && series_is_legal(board, r0, c0, -1, -1);
}

// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS], r0: i32, c0: i32) -> bool {
    for row in 0..NUM_ROWS {
        let mut queen_in_row = false;
        for col in 0..NUM_COLS {
            if board[row][col] == 'Q' {
                if queen_in_row {
                    return false;
                }
                queen_in_row = true;
            }
        }
        if !queen_in_row {
            return false;
        }
    }
    return board_is_legal(board, r0, c0);
}

// Try to place a queen in this column.
// Return true if we find a legal board.
fn place_queens_4(board: &mut [[char; NUM_COLS]; NUM_ROWS], c: i32, r0: i32, c0: i32) -> bool {
    if c == INUM_COLS {
        return board_is_a_solution(board, r0, c0);
    } else {
        if !board_is_legal(board, r0, c0) {
            return false;
        }
        for r in 0..INUM_ROWS {
            board[r as usize][c as usize] = 'Q';
            if place_queens_4(board, c + 1, r, c) {
                return true;
            }
            board[r as usize][c as usize] = '.';
        }
        return false;
    }
}
