#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Maze {
    width: u32,
    height: u32,
    vertical_walls: Vec<bool>, // Whether there is a wall to the left of each cell (row major order)
    horizontal_walls: Vec<bool>, // Whether there is a wall above each cell (row major order)
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        assert!(
            width > 0 && height > 0,
            "Width and height must be greater than 0"
        );

        let vertical_walls = vec![true; ((width + 1) * height) as usize];
        let horizontal_walls = vec![true; (width * (height + 1)) as usize];
        Maze {
            width,
            height,
            vertical_walls,
            horizontal_walls,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn has_wall(&self, x: u32, y: u32, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.vertical_walls[(y * (self.width + 1) + x) as usize],
            Direction::Up => self.horizontal_walls[(y * self.width + x) as usize],
            Direction::Right => self.vertical_walls[(y * (self.width + 1) + (x + 1)) as usize],
            Direction::Down => self.horizontal_walls[((y + 1) * self.width + x) as usize],
        }
    }

    pub fn set_wall(&mut self, x: u32, y: u32, direction: Direction, wall: bool) {
        match direction {
            Direction::Left => self.vertical_walls[(y * (self.width + 1) + x) as usize] = wall,
            Direction::Up => self.horizontal_walls[(y * self.width + x) as usize] = wall,
            Direction::Right => {
                self.vertical_walls[(y * (self.width + 1) + (x + 1)) as usize] = wall
            }
            Direction::Down => self.horizontal_walls[((y + 1) * self.width + x) as usize] = wall,
        }
    }
}
