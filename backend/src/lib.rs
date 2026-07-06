use wasm_bindgen::prelude::*;

/// Solve a Sudoku puzzle.
///
/// Accepts a flat array of 81 numbers (0 = empty cell, 1-9 = given digit).
/// Returns the solved board as a flat array of 81 numbers, or an empty array
/// if the puzzle has no solution.
#[wasm_bindgen]
pub fn solve(board: &[u8]) -> Vec<u8> {
    if board.len() != 81 {
        return Vec::new();
    }
    let mut grid = [0u8; 81];
    grid.copy_from_slice(board);
    if solve_board(&mut grid) {
        grid.to_vec()
    } else {
        Vec::new()
    }
}

/// Check whether a value can be placed at position (row, col) in the grid.
fn is_valid(grid: &[u8; 81], row: usize, col: usize, val: u8) -> bool {
    // Check row
    for c in 0..9 {
        if grid[row * 9 + c] == val {
            return false;
        }
    }
    // Check column
    for r in 0..9 {
        if grid[r * 9 + col] == val {
            return false;
        }
    }
    // Check 3×3 box
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if grid[r * 9 + c] == val {
                return false;
            }
        }
    }
    true
}

/// Backtracking solver — fills `grid` in-place and returns `true` on success.
fn solve_board(grid: &mut [u8; 81]) -> bool {
    for i in 0..81 {
        if grid[i] == 0 {
            let row = i / 9;
            let col = i % 9;
            for val in 1..=9u8 {
                if is_valid(grid, row, col, val) {
                    grid[i] = val;
                    if solve_board(grid) {
                        return true;
                    }
                    grid[i] = 0;
                }
            }
            return false; // No valid digit found — backtrack
        }
    }
    true // All cells filled
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_puzzle() -> [u8; 81] {
        [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8,
            0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6,
            0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ]
    }

    #[test]
    fn test_solve_returns_81_values() {
        let puzzle = sample_puzzle();
        let result = solve(&puzzle);
        assert_eq!(result.len(), 81);
    }

    #[test]
    fn test_solve_no_zeros() {
        let puzzle = sample_puzzle();
        let result = solve(&puzzle);
        assert!(!result.contains(&0));
    }

    #[test]
    fn test_solve_preserves_givens() {
        let puzzle = sample_puzzle();
        let result = solve(&puzzle);
        for (i, &given) in puzzle.iter().enumerate() {
            if given != 0 {
                assert_eq!(result[i], given);
            }
        }
    }

    #[test]
    fn test_unsolvable_returns_empty() {
        // Two 5s in the same row — unsolvable
        let mut puzzle = [0u8; 81];
        puzzle[0] = 5;
        puzzle[1] = 5;
        let result = solve(&puzzle);
        assert!(result.is_empty());
    }

    #[test]
    fn test_invalid_length_returns_empty() {
        let result = solve(&[1, 2, 3]);
        assert!(result.is_empty());
    }
}
