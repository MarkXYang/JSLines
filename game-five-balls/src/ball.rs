#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BallColor {
    Red, Blue, Yellow, Green, Brown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ball {
    pub id: u32,
    pub color: BallColor,
}

impl Ball { /* ... new ... */ }
impl BallColor { /* ... random_color (needs rand crate) ... */ }
grid.rs Proposal:

use crate::ball::Ball;
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Option<Ball>>>,
    ball_count: u32,
}

impl Grid {
    pub fn new(size: usize) -> Self { /* ... */ }
    pub fn place_ball(&mut self, ...) -> Result<(), &'static str> { /* ... */ }
    pub fn get_cell(&self, ...) -> Option<&Ball> { /* ... */ }
    pub fn is_empty(&self, ...) -> bool { /* ... */ }
}
