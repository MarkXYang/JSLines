use crate::ball::{Ball, BallColor};
use crate::grid::Grid;

#[derive(Debug, Clone)]
struct BallSequence {
    color: BallColor,
    count: usize,
    positions: Vec<(usize, usize)>,
}

impl BallSequence {
    pub fn new(color: BallColor, count: usize, positions: Vec<(usize, usize)>) -> Self {
        Self { color, count, positions }
    }
    pub fn increment(&mut self) {
        self.count += 1;
    }
    pub fn is_five_or_more(&self) -> bool {
        self.count >= 5
    }
}
    

pub struct Solver;
// (BallSequence struct and find_horizontal/vertical_lines as before)

impl Solver {
    fn find_diagonals_top_left_to_bottom_right(grid: &Grid) -> Vec<Vec<(usize, usize)>> {
        let mut lines_to_clear = Vec::new();
        let n = grid.size;
        // Loop 1: Diagonals starting on top row (0, start_c)
        for start_c in 0..n {
            for j in 0..n {
                let r = j;
                let c = start_c + j;
                if grid.is_within_bounds(r, c) && grid.cells[r][c].is_some() {
                    let color = grid.cells[r][c].unwrap();
                    let mut sequence = BallSequence::new(color, 1, vec![(r, c)]);
                    while grid.is_within_bounds(r + 1, c + 1) && grid.cells[r + 1][c + 1].is_some() && grid.cells[r + 1][c + 1].unwrap() == color {
                        sequence.increment();
                        r += 1;
                        c += 1;
                    }
                    if sequence.is_five_or_more() {
                        lines_to_clear.push(sequence.positions);
                    }
                }
            }
        }
        // Loop 2: Diagonals starting on left col (start_r, 0), (start_r > 0)
        for start_r in 1..n {
            for j in 0..n {
                let r = start_r + j;
                let c = j;
                if grid.is_within_bounds(r, c) && grid.cells[r][c].is_some() {
                    let color = grid.cells[r][c].unwrap();
                    let mut sequence = BallSequence::new(color, 1, vec![(r, c)]);
                    while grid.is_within_bounds(r + 1, c + 1) && grid.cells[r + 1][c + 1].is_some() && grid.cells[r + 1][c + 1].unwrap() == color {
                        sequence.increment();
                        r += 1;
                        c += 1;
                    }
                    if sequence.is_five_or_more() {
                        lines_to_clear.push(sequence.positions);
                    }
                }
            }
        }
        lines_to_clear
    }

    fn find_diagonals_top_right_to_bottom_left(grid: &Grid) -> Vec<Vec<(usize, usize)>> {
        let mut lines_to_clear = Vec::new();
        let n = grid.size;
        // Loop 1: Diagonals starting on top row (0, start_c)
        for start_c in 0..n {
            for j in 0..n {
                let r = j;
                let c = start_c - j;
                if grid.is_within_bounds(r, c) && grid.cells[r][c].is_some() {
                    let color = grid.cells[r][c].unwrap();
                    let mut sequence = BallSequence::new(color, 1, vec![(r, c)]);
                    while grid.is_within_bounds(r + 1, c - 1) && grid.cells[r + 1][c - 1].is_some() && grid.cells[r + 1][c - 1].unwrap() == color {
                        sequence.increment();
                        r += 1;
                        c -= 1;
                    }
                    if sequence.is_five_or_more() {
                        lines_to_clear.push(sequence.positions);
                    }
                }
            }
        }
        lines_to_clear
    }

    pub fn scan_and_collect_lines_to_clear(grid: &Grid) -> Vec<Vec<(usize, usize)>> {
        let mut all_lines = Vec::new();
        all_lines.extend(Self::find_horizontal_lines(grid));
        all_lines.extend(Self::find_vertical_lines(grid));
        all_lines.extend(Self::find_diagonals_top_left_to_bottom_right(grid));
        all_lines.extend(Self::find_diagonals_top_right_to_bottom_left(grid));
        all_lines
    }
}