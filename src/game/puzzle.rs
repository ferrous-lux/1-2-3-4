use std::sync::LazyLock;

use super::state::{Grid, Mode};

/// All 16 valid complete 2×2 grids.
static ALL_VALID_GRIDS: LazyLock<Vec<Grid>> = LazyLock::new(|| {
    vec![
        Grid::from([[1, 3], [2, 4]]),
        Grid::from([[1, 3], [4, 2]]),
        Grid::from([[3, 1], [2, 4]]),
        Grid::from([[3, 1], [4, 2]]),
        Grid::from([[1, 2], [3, 4]]),
        Grid::from([[1, 4], [3, 2]]),
        Grid::from([[3, 2], [1, 4]]),
        Grid::from([[3, 4], [1, 2]]),
        Grid::from([[2, 1], [4, 3]]),
        Grid::from([[2, 3], [4, 1]]),
        Grid::from([[4, 1], [2, 3]]),
        Grid::from([[4, 3], [2, 1]]),
        Grid::from([[2, 4], [1, 3]]),
        Grid::from([[2, 4], [3, 1]]),
        Grid::from([[4, 2], [1, 3]]),
        Grid::from([[4, 2], [3, 1]]),
    ]
});

const POSITIONS: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

fn mode_offset(mode: Mode) -> u64 {
    match mode {
        Mode::Easy => 0,
        Mode::Medium => 7,
        Mode::Hard => 13,
        Mode::Extreme => 0,
    }
}

pub fn is_valid_solution(grid: &Grid) -> bool {
    ALL_VALID_GRIDS.contains(grid)
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub grid: Grid,
    pub prefilled: [[bool; 2]; 2],
    pub mode: Mode,
}

/// Generate a puzzle for the given mode, seeded by user click count.
pub fn generate(mode: Mode, clicks: u64) -> Puzzle {
    match mode {
        Mode::Extreme => Puzzle {
            grid: Grid::new(),
            prefilled: [[false; 2]; 2],
            mode,
        },
        _ => {
            let idx = (clicks.wrapping_add(mode_offset(mode)) as usize) % 16;
            let solution = &ALL_VALID_GRIDS[idx];
            let mut grid = *solution;
            let mut prefilled = [[true; 2]; 2];

            let pattern = idx / 4; // 0..3
            let blank_count = 4 - mode.prefilled_count();
            for b in 0..blank_count {
                let (r, c) = POSITIONS[(pattern + b) % 4];
                grid.set(r, c, None);
                prefilled[r][c] = false;
            }

            Puzzle {
                grid,
                prefilled,
                mode,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sixteen_valid_grids() {
        assert_eq!(ALL_VALID_GRIDS.len(), 16);
    }

    #[test]
    fn test_all_grids_are_distinct() {
        let mut sorted: Vec<Grid> = ALL_VALID_GRIDS.iter().copied().collect();
        sorted.sort_by_key(|g| {
            let c = g.cells();
            [c[0][0], c[0][1], c[1][0], c[1][1]]
        });
        sorted.dedup();
        assert_eq!(sorted.len(), 16);
    }

    #[test]
    fn test_is_valid_solution_accepts_valid() {
        for grid in ALL_VALID_GRIDS.iter() {
            assert!(is_valid_solution(grid));
        }
    }

    #[test]
    fn test_is_valid_solution_rejects_invalid() {
        let invalid = Grid::from([[1, 2], [4, 3]]);
        assert!(!is_valid_solution(&invalid));
    }

    #[test]
    fn test_is_valid_solution_rejects_incomplete() {
        let mut grid = Grid::new();
        grid.set(0, 0, Some(1));
        assert!(!is_valid_solution(&grid));
    }

    #[test]
    fn test_easy_puzzle_has_three_prefilled() {
        let puzzle = generate(Mode::Easy, 0);
        let n = puzzle.prefilled.iter().flatten().filter(|&&b| b).count();
        assert_eq!(n, 3);
    }

    #[test]
    fn test_medium_puzzle_has_two_prefilled() {
        let puzzle = generate(Mode::Medium, 0);
        let n = puzzle.prefilled.iter().flatten().filter(|&&b| b).count();
        assert_eq!(n, 2);
    }

    #[test]
    fn test_hard_puzzle_has_one_prefilled() {
        let puzzle = generate(Mode::Hard, 0);
        let n = puzzle.prefilled.iter().flatten().filter(|&&b| b).count();
        assert_eq!(n, 1);
    }

    #[test]
    fn test_extreme_puzzle_has_no_prefilled() {
        let puzzle = generate(Mode::Extreme, 0);
        let n = puzzle.prefilled.iter().flatten().filter(|&&b| b).count();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_extreme_puzzle_is_empty() {
        let puzzle = generate(Mode::Extreme, 0);
        assert!(puzzle.grid.cells().iter().flatten().all(|c| c.is_none()));
    }

    #[test]
    fn test_prefilled_cells_match_grid() {
        let puzzle = generate(Mode::Medium, 0);
        for r in 0..2 {
            for c in 0..2 {
                if puzzle.prefilled[r][c] {
                    assert!(puzzle.grid.get(r, c).is_some());
                }
            }
        }
    }

    #[test]
    fn test_different_clicks_give_different_grids() {
        let a = generate(Mode::Easy, 0).grid;
        let b = generate(Mode::Easy, 1).grid;
        assert_ne!(a, b);
    }
}
