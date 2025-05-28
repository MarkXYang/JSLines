use std::collections::VecDeque;
use crate::grid::Grid; // Or appropriate path

/**
 * Pathfinding is responsible for finding a path between two points on the grid.
 * It is used to move balls from one point to another.
 */
pub fn find_path(
    grid: &Grid,
    start_coords: (usize, usize),
    end_coords: (usize, usize)
) -> Option<Vec<(usize, usize)>> {
    let (start_row, start_col) = start_coords;
    // Basic checks: end_coords must be empty, start/end within bounds
    if !grid.is_within_bounds(start_row, start_col) || !grid.is_within_bounds(end_row, end_col) {
        return None;
    }
    // (Need is_within_bounds helper in Grid)
    if grid.cells[start_row][start_col].is_some() || grid.cells[end_row][end_col].is_some() {
        return None;
    }

    let mut queue: VecDeque<((usize, usize), Vec<(usize, usize)>)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid.size]; grid.size];

    queue.push_back((start_coords, vec![start_coords]));
    visited[start_row][start_col] = true;

    while let Some(((current_row, current_col), path)) = queue.pop_front() {
        if (current_row, current_col) == end_coords {
            return Some(path); // Path found
        }

        // Explore neighbors (up, down, left, right)
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_row = current_row as isize + dr;
            let new_col = current_col as isize + dc;

            if new_row < 0 || new_row >= grid.size as isize || new_col < 0 || new_col >= grid.size as isize {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if visited[new_row][new_col] {
                continue;
            }

            if grid.cells[new_row][new_col].is_some() {
                continue;
            }

            let new_path = path.clone();
            new_path.push((new_row, new_col));
            visited[new_row][new_col] = true;
            queue.push_back(((new_row, new_col), new_path));
        }
    }
    None // No path found
}