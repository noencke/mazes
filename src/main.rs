mod generate;
mod maze;
mod print;
mod save;
use maze::Maze;
use save::save_maze;
use std::{env, path::Path};

const TARGET_IMAGE_SIZE: u32 = 1000;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: {} <width> <height> [path]", args[0]);
        return;
    }

    match (args[1].parse(), args[2].parse()) {
        (Ok(width), Ok(height)) if width > 0 && height > 0 => {
            let maze = Maze::generate_perfect(width, height);
            if args.len() > 3 {
                let path = Path::new(&args[3]);
                save_maze(&maze, &path, TARGET_IMAGE_SIZE).unwrap_or_else(|e| {
                    println!("Failed to write to file {}: {}", args[3], e);
                });
            } else {
                maze.print();
            }
        }
        _ => {
            println!("Usage: {} <width> <height> [path]", args[0]);
            println!("Width and height must be integers greater than 0");
        }
    }
}
