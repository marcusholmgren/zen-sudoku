use wasm_bindgen::prelude::*;

/// Solve a Sudoku puzzle.
///
/// Accepts a flat array of 81 numbers (0 = empty cell, 1-9 = given digit).
/// Returns the solved board as a flat array of 81 numbers, or an empty array
/// if the puzzle is invalid or has no solution.
#[wasm_bindgen]
pub fn solve(board: &[u8]) -> Vec<u8> {
    if board.len() != 81 {
        return Vec::new();
    }

    let mut grid = [0u8; 81];
    let mut row_masks = [0u16; 9];
    let mut col_masks = [0u16; 9];
    let mut box_masks = [0u16; 9];

    // 1. Validate inputs and populate bitmasks, detecting initial conflicts
    for i in 0..81 {
        let val = board[i];
        if val > 9 {
            return Vec::new(); // Invalid cell value
        }
        grid[i] = val;

        if val > 0 {
            let row = i / 9;
            let col = i % 9;
            let box_idx = (row / 3) * 3 + (col / 3);
            let mask = 1 << (val - 1);

            // Check if this given digit conflicts with other givens
            if (row_masks[row] & mask) != 0
                || (col_masks[col] & mask) != 0
                || (box_masks[box_idx] & mask) != 0
            {
                return Vec::new(); // Initial board is invalid (contains conflicts)
            }

            row_masks[row] |= mask;
            col_masks[col] |= mask;
            box_masks[box_idx] |= mask;
        }
    }

    // 2. Solve the board using backtracking with MRV heuristic
    if solve_board(&mut grid, &mut row_masks, &mut col_masks, &mut box_masks) {
        grid.to_vec()
    } else {
        Vec::new()
    }
}

/// Helper function to find the empty cell with the fewest candidates (MRV heuristic).
/// Returns the index of the cell and its allowed candidates mask, or None if all cells are filled.
fn find_best_cell(
    grid: &[u8; 81],
    row_masks: &[u16; 9],
    col_masks: &[u16; 9],
    box_masks: &[u16; 9],
) -> Option<(usize, u16)> {
    let mut best_idx = None;
    let mut min_candidates = 10;
    let mut best_candidates_mask = 0;

    for i in 0..81 {
        if grid[i] == 0 {
            let row = i / 9;
            let col = i % 9;
            let box_idx = (row / 3) * 3 + (col / 3);

            let used = row_masks[row] | col_masks[col] | box_masks[box_idx];
            let allowed = !used & 0x1FF; // Only bits 0..8 represent values 1..9
            let count = allowed.count_ones();

            if count < min_candidates {
                min_candidates = count;
                best_idx = Some(i);
                best_candidates_mask = allowed;
                if count == 0 {
                    // Fast path: Cell has 0 candidates, this path is dead
                    return Some((i, 0));
                }
            }
        }
    }

    best_idx.map(|idx| (idx, best_candidates_mask))
}

/// Backtracking solver using MRV and bitmasks.
fn solve_board(
    grid: &mut [u8; 81],
    row_masks: &mut [u16; 9],
    col_masks: &mut [u16; 9],
    box_masks: &mut [u16; 9],
) -> bool {
    if let Some((idx, candidates_mask)) = find_best_cell(grid, row_masks, col_masks, box_masks) {
        if candidates_mask == 0 {
            return false; // No candidates available for this cell, backtrack
        }

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + (col / 3);

        // Try each candidate digit
        for val in 1..=9u8 {
            let val_mask = 1 << (val - 1);
            if (candidates_mask & val_mask) != 0 {
                // Place digit
                grid[idx] = val;
                row_masks[row] |= val_mask;
                col_masks[col] |= val_mask;
                box_masks[box_idx] |= val_mask;

                // Recurse
                if solve_board(grid, row_masks, col_masks, box_masks) {
                    return true;
                }

                // Backtrack
                grid[idx] = 0;
                row_masks[row] &= !val_mask;
                col_masks[col] &= !val_mask;
                box_masks[box_idx] &= !val_mask;
            }
        }
        false
    } else {
        true // All cells filled successfully
    }
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

    #[test]
    fn test_invalid_value_returns_empty() {
        let mut puzzle = sample_puzzle();
        puzzle[2] = 10; // Invalid value
        let result = solve(&puzzle);
        assert!(result.is_empty());
    }
}
