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

struct Prng {
    state: u64,
}

impl Prng {
    fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 0x123456789abcdef } else { seed } }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x as u32
    }

    fn range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        let diff = (max - min) as u32;
        min + (self.next_u32() % diff) as usize
    }

    fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.range(0, i + 1);
            slice.swap(i, j);
        }
    }
}

/// Backtracking grid generator with randomized search order.
fn fill_grid_randomized(
    grid: &mut [u8; 81],
    row_masks: &mut [u16; 9],
    col_masks: &mut [u16; 9],
    box_masks: &mut [u16; 9],
    prng: &mut Prng,
) -> bool {
    if let Some((idx, candidates_mask)) = find_best_cell(grid, row_masks, col_masks, box_masks) {
        if candidates_mask == 0 {
            return false;
        }

        let row = idx / 9;
        let col = idx % 9;
        let box_idx = (row / 3) * 3 + (col / 3);

        let mut digits: Vec<u8> = (1..=9u8)
            .filter(|&val| (candidates_mask & (1 << (val - 1))) != 0)
            .collect();

        prng.shuffle(&mut digits);

        for val in digits {
            let val_mask = 1 << (val - 1);
            grid[idx] = val;
            row_masks[row] |= val_mask;
            col_masks[col] |= val_mask;
            box_masks[box_idx] |= val_mask;

            if fill_grid_randomized(grid, row_masks, col_masks, box_masks, prng) {
                return true;
            }

            grid[idx] = 0;
            row_masks[row] &= !val_mask;
            col_masks[col] &= !val_mask;
            box_masks[box_idx] &= !val_mask;
        }
        false
    } else {
        true
    }
}

fn get_seed() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x123456789)
    }
}

/// Generate a new Sudoku puzzle given a difficulty ("easy", "medium", "hard").
///
/// Returns an 81-length array with givens and 0 for empty cells.
#[wasm_bindgen]
pub fn generate(difficulty: &str) -> Vec<u8> {
    let givens_count = match difficulty.to_lowercase().as_str() {
        "easy" => 40,
        "medium" => 32,
        "hard" => 26,
        _ => 32,
    };

    let seed = get_seed();
    let mut prng = Prng::new(seed);

    let mut full_grid = [0u8; 81];
    let mut row_masks = [0u16; 9];
    let mut col_masks = [0u16; 9];
    let mut box_masks = [0u16; 9];

    fill_grid_randomized(&mut full_grid, &mut row_masks, &mut col_masks, &mut box_masks, &mut prng);

    let mut puzzle = full_grid;
    let mut indices: Vec<usize> = (0..81).collect();
    prng.shuffle(&mut indices);

    let cells_to_remove = 81 - givens_count;
    for &idx in indices.iter().take(cells_to_remove) {
        puzzle[idx] = 0;
    }

    puzzle.to_vec()
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
        puzzle[2] = 10;
        let result = solve(&puzzle);
        assert!(result.is_empty());
    }

    #[test]
    fn test_generate_easy_medium_hard() {
        for diff in &["easy", "medium", "hard"] {
            let puzzle = generate(diff);
            assert_eq!(puzzle.len(), 81);

            let given_count = puzzle.iter().filter(|&&v| v != 0).count();
            let expected_givens = match *diff {
                "easy" => 40,
                "medium" => 32,
                "hard" => 26,
                _ => 32,
            };
            assert_eq!(given_count, expected_givens);

            let solved = solve(&puzzle);
            assert_eq!(solved.len(), 81);
            assert!(!solved.contains(&0));
        }
    }
}
