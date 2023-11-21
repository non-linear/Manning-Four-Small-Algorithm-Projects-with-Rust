use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == INUM_COLS * INUM_ROWS {
        if REQUIRE_CLOSED_TOUR {
            for [offset_x, offset_y] in offsets {
                let new_row = cur_row + offset_y;
                let new_col = cur_col + offset_x;
                if new_row == 0 && new_col == 0 {
                    return true;
                }
            }
            return false;
        }
        return true;
    } else {
        // try all moves
        for [offset_x, offset_y] in offsets {
            let new_row = cur_row + offset_y;
            let new_col = cur_col + offset_x;

            // test legality of move
            if new_row >= 0 && new_row < INUM_ROWS && new_col >= 0 && new_col < INUM_COLS {
                // test usefulness of move
                if board[new_row as usize][new_col as usize] == UNVISITED {
                    board[new_row as usize][new_col as usize] = num_visited + 1;
                    if find_tour(board, offsets, new_row, new_col, num_visited + 1) {
                        // Solution for path has been found. Back-propagate "the good news".
                        return true;
                    } else {
                        // Solution for path couldn't be found, back up.
                        board[new_row as usize][new_col as usize] = UNVISITED;
                    }
                }
            }
        }
        // if no move leads to
        return false;
    }
}

fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    println!("");
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            print!("{position:0>2} ", position = board[row][col]);
        }
        println!("")
    }
}

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}
