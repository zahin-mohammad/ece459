// Library routines for reading and solving Sudoku puzzles

#![warn(clippy::all)]
pub mod verify;

use std::io::Read;
use std::num::{NonZeroU8};

// Type definition for a 9x9 array that will represent a Sudoku puzzle.
// Entries with None represent unfilled positions in the puzzle.
type Sudoku = [[Option<NonZeroU8>; 9]; 9];

// This function is called by main. It calls solve() to recursively find the solution.
// The puzzle is modified in-place.
pub fn solve_puzzle(puzzle: &mut Sudoku) {
    match get_next_empty_cell(puzzle) {
        None => {}
        Some(next_cell) => {
            solve(puzzle, next_cell.0, next_cell.1);
        }
    }
}

// Fills in the empty positions in the puzzle with the right values, using a
// recursive brute force approach. Modify the puzzle in place. Return true if
// solved successfully, false if unsuccessful. You may modify the function signature
// if you need/wish.
fn solve(puzzle: &mut Sudoku, row: usize, col: usize) -> bool {
    // Try all possible combinations until it passes....
    for i  in 1..10 {
        let guess = NonZeroU8::new(i);
        if check_square(puzzle, row, col, guess.unwrap()) {
            puzzle[row][col] = guess;
            let next_cell;
            // If current guess works and there are no other empty cells, it is solved
            match get_next_empty_cell(puzzle) {
                None => { return true; }
                Some(e) => { next_cell = e; }
            }
            // Back track
            if solve(puzzle, next_cell.0, next_cell.1) {
                return true
            }
            puzzle[row][col] = Option::None;
        }
    }
    return false;
}

fn get_next_empty_cell(puzzle: &Sudoku) -> Option<(usize, usize)> {
    for next_row in 0..puzzle.len() {
        for next_col in 0..puzzle.len() {
            if puzzle[next_row][next_col].is_none() {
                return Option::Some((next_row, next_col));
            }
        }
    }
    return Option::None
}
// Helper that checks if a specific square in the puzzle can take on
// a given value. Return true if that value is allowed in that square, false otherwise.
// You can choose not to use this if you prefer.
fn check_square(puzzle: &Sudoku, row: usize, col: usize, val: NonZeroU8) -> bool {
    // Check vertical
    for i in 0..puzzle.len(){
        if row == i{
            continue
        }
        if puzzle[i][col] == Some(val) {
            // println!("check_square failed vertically on {} {} {}", row, col, val);
            return false;
        }
    }
    // Check horizontally
    for i in 0..puzzle.len() {
        if col == i {
            continue
        }
        if puzzle[row][i] == Some(val){
            // println!("check_square failed horizontally on {} {} {}", row, col, val);
            return false;
        }
    }
    // Check subgrid
    let subgrid_row_start = row - row %3;
    let subgrid_col_start = col - col %3 ;
    for row_iter in 0..3 {
        for col_iter in 0..3 {
            if subgrid_row_start+row_iter == row && subgrid_col_start+col_iter == col {
                continue;
            }
            if puzzle[subgrid_row_start+row_iter][subgrid_col_start+col_iter] == Some(val) {
                // println!("check_square failed in subgrid on {} {} {}", row_iter, col_iter, val);
                return false;
            }
        }
    }
    true
}

// Helper for printing a sudoku puzzle to stdout for debugging.
pub fn print_puzzle(puzzle: &Sudoku) {
    for row in puzzle.iter() {
        for elem in row.iter() {
            print!("{}", elem.map(|e| (e.get() + b'0') as char).unwrap_or('.'));
        }
        print!("\n");
    }
    print!("\n");
}

// Read the input byte by byte until a complete Sudoku puzzle has been
// read or EOF is reached.  Assumes the input follows the correct format
// (i.e. matching the files in the input folder).
pub fn read_puzzle(reader: &mut impl Read) -> Option<Box<Sudoku>> {
    // Turn the input stream into an iterator of bytes
    let mut bytes = reader.bytes().map(|b| b.expect("input error")).peekable();
    // Go thru the input until we find a puzzle or EOF (None)
    loop {
        match bytes.peek() {
            Some(b'1'..=b'9') | Some(b'.') => break,
            None => return None,
            _ => {
                bytes.next();
            }
        }
    }
    let mut puzzle:Box<[[Option<NonZeroU8>;9]; 9]> = Box::new([[None; 9]; 9]);
    // Fill in the puzzle matrix. Ignore the non-puzzle input bytes.
    // Credit: https://piazza.com/class/kiz072yjgnk57a?cid=33
    for i in 0..9 {
        let mut j = 0;
        while j < 9 {
            let b = bytes.next().expect("unexpected EOF");

            let elem = match b {
                b'1'..=b'9' => NonZeroU8::new(b - b'0'),
                b'.' => None,
                _ => continue,
            };
            puzzle[i][j] = elem;
            j += 1;
        }
    }
    Some(puzzle)
}

// Do a simple check that the puzzle is valid.
// Returns true if it is valid, false if it is not.
// (The verifier server doesn't tell you what's wrong so this function can also help you track
// down an error if your puzzles are not being solved correctly.)
pub fn check_puzzle(puzzle: &Sudoku) -> bool {
    // Check that each row is valid
    // Check that each column is valid
    // Check that each subgrid is valid
    for (row_index, row) in puzzle.iter().enumerate() {
        for (col_index, val) in row.iter().enumerate() {
            if !check_square(puzzle,
                             row_index,
                             col_index,
                             val.expect("should not have a None value in puzzle")) {
                println!("failed on {} {} {}", row_index, col_index, val.unwrap());
                return false;
            }
        }
    }
    true
}
