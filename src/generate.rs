use crate::maze::{Direction, Maze};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{random, RngCore};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashSet;
use std::vec;

impl Maze {
    /// Generates a "perfect maze" using backtracking.
    pub fn generate_perfect(width: u32, height: u32) -> Maze {
        let mut maze = Maze::new(width, height);
        let rng = &mut rand::thread_rng();

        // Randomly choose the entrance to the maze - a cell on the border - and remove the wall there.
        let (start_x, start_y) =
            choose_random_border_cell(maze.get_width(), maze.get_height(), rng);

        maze.carve_border_cell(start_x, start_y, rng);

        // Keep track of the border cell that is the "farthest walk" from the entrance cell.
        // It will become the exit after the maze is fully generated.
        struct Exit {
            x: u32,
            y: u32,
            distance: u64,
        }
        let mut exit = Exit {
            x: start_x,
            y: start_y,
            distance: 0,
        };

        // Create a stack of pending cells through which to carve the path. Each will be visited once.
        struct PendingCell {
            x: u32,
            y: u32,
            directions: Vec<Direction>,
        }
        impl PendingCell {
            fn new(x: u32, y: u32, rng: &mut ThreadRng) -> PendingCell {
                let mut directions = vec![
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                ];
                directions.shuffle(rng);
                PendingCell { x, y, directions }
            }
        }
        let mut path = vec![PendingCell::new(start_x, start_y, rng)];
        let visited = &mut HashSet::from([(start_x, start_y)]);

        // Carve the path through the maze.
        while let Some(cell) = path.last_mut() {
            let (x, y) = (cell.x, cell.y);
            match cell.directions.pop() {
                Some(direction) => {
                    let next_xy = maze.get_adjacent_cell(x, y, direction);
                    if let Some((next_x, next_y)) = next_xy {
                        if !visited.contains(&next_xy.unwrap()) {
                            visited.insert((next_x, next_y));
                            path.push(PendingCell::new(next_x, next_y, rng));
                            maze.set_wall(x, y, direction, false);
                            // If this is a border cell and it's farther from the entrance than the current exit, make it the new exit.
                            let distance = path.len() as u64;
                            if maze.is_border_cell(next_x, next_y) {
                                if distance > exit.distance {
                                    exit = Exit {
                                        x: next_x,
                                        y: next_y,
                                        distance,
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    // When there are no new directions we can carve from this cell (we've boxed ourselves in somewhere), go back to the previous cell.
                    path.pop();
                }
            }
        }

        // Carve the exit.
        maze.carve_border_cell(exit.x, exit.y, rng);
        maze
    }

    fn get_adjacent_cell(&self, x: u32, y: u32, direction: Direction) -> Option<(u32, u32)> {
        match direction {
            Direction::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Direction::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x < self.get_width() - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y < self.get_height() - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        }
    }

    fn is_border_cell(&self, x: u32, y: u32) -> bool {
        x == 0 || y == 0 || x == self.get_width() - 1 || y == self.get_height() - 1
    }

    fn carve_border_cell(&mut self, x: u32, y: u32, rng: &mut ThreadRng) {
        match vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
        .into_iter()
        .filter_map(|d| {
            if self.get_adjacent_cell(x, y, d).is_none() && self.has_wall(x, y, d) {
                Some(d)
            } else {
                None
            }
        })
        .choose(rng)
        {
            Some(direction) => self.set_wall(x, y, direction, false),
            None => panic!("No border cell to carve"),
        }
    }
}

fn choose_random_border_cell(width: u32, height: u32, rng: &mut dyn RngCore) -> (u32, u32) {
    assert!(width > 0 && height > 0);
    match (width, height) {
        (1, 1) => (0, 0),
        (w, 1) => (rng.gen_range(0..w), 0),
        (1, h) => (0, rng.gen_range(0..h)),
        _ => {
            if random::<bool>() {
                if random::<bool>() {
                    (rng.gen_range(0..width - 1), 0)
                } else {
                    (rng.gen_range(1..width), height - 1)
                }
            } else {
                if random::<bool>() {
                    (0, rng.gen_range(1..height))
                } else {
                    (width - 1, rng.gen_range(0..height - 1))
                }
            }
        }
    }
}
