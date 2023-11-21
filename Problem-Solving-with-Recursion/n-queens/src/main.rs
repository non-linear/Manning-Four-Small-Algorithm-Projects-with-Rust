use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 6;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];
    // let mut attack_counts = [[0; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_1(&mut board, 0, 0);
    // let success = place_queens_2(&mut board, 0, 0, 0);
    // let success = place_queens_3(&mut board, &mut attack_counts, 0, 0);
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
fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    for r0 in 0..INUM_ROWS {
        for c0 in 0..INUM_COLS {
            if !series_is_legal(board, r0, c0, 1, 0)
                || !series_is_legal(board, r0, c0, -1, 0)
                || !series_is_legal(board, r0, c0, 0, 1)
                || !series_is_legal(board, r0, c0, 0, -1)
                || !series_is_legal(board, r0, c0, 1, 1)
                || !series_is_legal(board, r0, c0, 1, -1)
                || !series_is_legal(board, r0, c0, -1, 1)
                || !series_is_legal(board, r0, c0, -1, -1)
            {
                return false;
            }
        }
    }
    return true;
}

// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
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
    return board_is_legal(board);
}

// Try placing a queen at position [r][c].
// Return true if we find a legal board.
#[allow(dead_code)]
fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {
    if r >= INUM_ROWS {
        return board_is_a_solution(board);
    } else {
        let cur_r: usize = r as usize;
        let cur_c: usize = c as usize;
        let next_c = (c + 1) % INUM_COLS;
        let next_r = r + (c + 1) / INUM_COLS;

        // try NOT placing a queen
        if place_queens_1(board, next_r, next_c) {
            return true;
        }
        // try placing a queen
        board[cur_r][cur_c] = 'Q';
        if place_queens_1(board, next_r, next_c) {
            return true;
        }
        // neither not placing nor placing a queen finds solution: backtrack
        board[cur_r][cur_c] = '.';
        return false;
    }
}

// Try placing a queen at position [r][c].
// Return true if we find a legal board.
#[allow(dead_code)]
fn place_queens_2(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    num_placed: i32,
) -> bool {
    if r >= INUM_ROWS || num_placed == INUM_ROWS {
        return board_is_a_solution(board);
    } else {
        let cur_r: usize = r as usize;
        let cur_c: usize = c as usize;
        let next_c = (c + 1) % INUM_COLS;
        let next_r = r + (c + 1) / INUM_COLS;

        // try NOT placing a queen
        if place_queens_2(board, next_r, next_c, num_placed) {
            return true;
        }
        // try placing a queen
        board[cur_r][cur_c] = 'Q';
        if place_queens_2(board, next_r, next_c, num_placed + 1) {
            return true;
        }
        // neither not placing nor placing a queen finds solution: backtrack
        board[cur_r][cur_c] = '.';
        return false;
    }
}

// Try placing a queen at position [r][c].
// Return true if we find a legal board.
#[allow(dead_code)]
fn place_queens_3(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    attack_counts: &mut [[i32; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
) -> bool {
    if r >= INUM_ROWS {
        return board_is_a_solution(board);
    } else {
        let cur_r: usize = r as usize;
        let cur_c: usize = c as usize;
        let next_c = (c + 1) % INUM_COLS;
        let next_r = r + (c + 1) / INUM_COLS;

        // try NOT placing a queen
        if place_queens_3(board, attack_counts, next_r, next_c) {
            return true;
        }
        // try placing a queen
        if attack_counts[cur_r][cur_c] == 0 {
            board[cur_r][cur_c] = 'Q';
            adjust_attack_counts(board, attack_counts);
            if place_queens_3(board, attack_counts, next_r, next_c) {
                return true;
            }
        }
        // neither not placing nor placing a queen finds solution: backtrack
        board[cur_r][cur_c] = '.';
        adjust_attack_counts(board, attack_counts);
        return false;
    }
}

fn adjust_attack_counts(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    attack_counts: &mut [[i32; NUM_COLS]; NUM_ROWS],
) {
    // reset attack accounts to zero
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            attack_counts[row][col] = 0;
        }
    }

    // scan board for queens
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            if board[row][col] == 'Q' {
                // increase attack counts in current position
                attack_counts[row][col] += 1;
                // increase attack counts to the left
                for i in 0..col {
                    attack_counts[row][i] += 1;
                }
                // increase attack counts to the right
                for i in col + 1..NUM_COLS {
                    attack_counts[row][i] += 1;
                }
                // increase attack counts to the top
                for i in 0..row {
                    attack_counts[i][col] += 1;
                }
                // increase attack counts to the bottom
                for i in row + 1..NUM_ROWS {
                    attack_counts[i][col] += 1;
                }
                // increase attack counts to top right
                let mut new_col = col as i32 + 1;
                let mut new_row = row as i32 + 1;
                while position_onboard(new_row, new_col) {
                    attack_counts[new_row as usize][new_col as usize] += 1;
                    new_col += 1;
                    new_row += 1;
                }
                // increase attack counts to bottom right
                let mut new_col = col as i32 + 1;
                let mut new_row = row as i32 - 1;
                while position_onboard(new_row, new_col) {
                    attack_counts[new_row as usize][new_col as usize] += 1;
                    new_col += 1;
                    new_row -= 1;
                }
                // increase attack counts to top left
                let mut new_col = col as i32 - 1;
                let mut new_row = row as i32 + 1;
                while position_onboard(new_row, new_col) {
                    attack_counts[new_row as usize][new_col as usize] += 1;
                    new_col -= 1;
                    new_row += 1;
                }
                // increase attack counts to bottom left
                let mut new_col = col as i32 - 1;
                let mut new_row = row as i32 - 1;
                while position_onboard(new_row, new_col) {
                    attack_counts[new_row as usize][new_col as usize] += 1;
                    new_col -= 1;
                    new_row -= 1;
                }
            }
        }
    }
}
