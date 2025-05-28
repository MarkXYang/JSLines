use rand::Rng;
use crate::grid::Grid;
use crate::solver::BallSequence;

/**
 * Ball is a struct that represents a ball on the grid.
 * It has an id and a color.
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BallColor {
    Red, Blue, Yellow, Green, Brown,
}

/**
 * Ball is a struct that represents a ball on the grid.
 * It has an id and a color.
 */ 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ball {
    pub id: u32,
    pub color: BallColor,
}

impl Ball { 
    pub fn new(id: u32, color: BallColor) -> Self {
        Self { id, color }
    }
}

impl BallColor { 
    pub fn random_color() -> Self {
        let colors = [BallColor::Red, BallColor::Blue, BallColor::Yellow, BallColor::Green, BallColor::Brown];
        colors[rand::thread_rng().gen_range(0..colors.len())]
    }
}

// The following definitions seem to be remnants of a bad merge or copy-paste.
// They are unrelated to BallColor and Ball struct definitions.
// Removing them as they are duplicated or misplaced from grid.rs or other files.
// use crate::ball::Ball;
// pub struct Grid {
//    pub size: usize,
// Removing the problematic block below as it's a malformed copy of Grid or other struct parts.
/*
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Option<Ball>>>,
    ball_count: u32,
}

impl Grid {
    pub fn new(size: usize) -> Self { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
    pub fn place_ball(&mut self, ...) -> Result<(), &'static str> { 
        Self { 
    }
    pub fn get_cell(&self, ...) -> Option<&Ball> { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
    pub fn is_empty(&self, ...) ->  bool { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
    pub fn is_within_bounds(&self, ...) -> bool { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
    pub fn is_valid_move(&self, ...) -> bool { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
    pub fn get_neighbors(&self, ...) -> Vec<Option<&Ball>> { 
        Self { 
            size, 
            cells: vec![vec![None; size]; size], 
            ball_count: 0 
        }
    }
}