use crate::maze::{Direction, Maze};

impl Maze {
    pub fn print(&self) {
        for y in 0..self.get_height() + 1 {
            for x in 0..self.get_width() + 1 {
                let ux: i64 = x.into();
                let uy: i64 = y.into();
                let print_left: bool = self.has_wall_safe(ux - 1, uy, Direction::Up);
                let print_up = self.has_wall_safe(ux, uy - 1, Direction::Left);
                let print_right = self.has_wall_safe(ux, uy, Direction::Up);
                let print_down = self.has_wall_safe(ux, uy, Direction::Left);
                if print_left && print_up && print_right && print_down {
                    print!("┼─");
                } else if print_left && print_up && print_right {
                    print!("┴─");
                } else if print_left && print_up && print_down {
                    print!("┤ ");
                } else if print_left && print_right && print_down {
                    print!("┬─");
                } else if print_up && print_right && print_down {
                    print!("├─");
                } else if print_left && print_up {
                    print!("┘ ");
                } else if print_left && print_right {
                    print!("──");
                } else if print_left && print_down {
                    print!("┐ ");
                } else if print_up && print_right {
                    print!("└─");
                } else if print_up && print_down {
                    print!("│ ");
                } else if print_right && print_down {
                    print!("┌─");
                } else if print_left {
                    print!("╴ ");
                } else if print_up {
                    print!("╵ ");
                } else if print_right {
                    print!("╶─");
                } else if print_down {
                    print!("╷ ");
                } else {
                    print!("  ");
                }
            }
            println!("");
        }
    }

    pub fn has_wall_safe(&self, x: i64, y: i64, direction: Direction) -> bool {
        let width = self.get_width().into();
        let height = self.get_height().into();
        if x < -1 || y < -1 || x > width || y > height {
            return false;
        }
        if y == -1 {
            return match x {
                -1 => false,
                x if x == width => false,
                _ => match direction {
                    Direction::Down => {
                        self.has_wall(x.try_into().expect("Out of bounds"), 0, Direction::Up)
                    }
                    _ => false,
                },
            };
        }
        if x == -1 {
            return match y {
                -1 => false,
                y if y == height => false,
                _ => match direction {
                    Direction::Right => {
                        self.has_wall(0, y.try_into().expect("Out of bounds"), Direction::Left)
                    }
                    _ => false,
                },
            };
        }
        if y == height {
            return match x {
                -1 => false,
                x if x == width => false,
                _ => match direction {
                    Direction::Up => self.has_wall(
                        x.try_into().expect("Out of bounds"),
                        (height - 1).try_into().expect("Out of bounds"),
                        Direction::Down,
                    ),
                    _ => false,
                },
            };
        }
        if x == width {
            return match y {
                -1 => false,
                y if y == height => false,
                _ => match direction {
                    Direction::Left => self.has_wall(
                        (width - 1).try_into().expect("Out of bounds"),
                        y.try_into().expect("Out of bounds"),
                        Direction::Right,
                    ),
                    _ => false,
                },
            };
        }
        return self.has_wall(
            x.try_into().expect("Out of bounds"),
            y.try_into().expect("Out of bounds"),
            direction,
        );
    }
}
