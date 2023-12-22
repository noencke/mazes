use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, io::Write};

use crate::maze::{Direction, Maze};

pub fn save_maze(maze: &Maze, path: &Path, target_image_size: u32) -> Result<(), std::io::Error> {
    let unscaled_image_width = maze.get_width() * 2 + 1;
    let unscaled_image_height = maze.get_height() * 2 + 1;
    let scale = (target_image_size / unscaled_image_width.max(unscaled_image_height)).max(1);
    let file = File::create(path)?;
    let mut encoder = png::Encoder::new(
        BufWriter::new(file),
        unscaled_image_width * scale,
        unscaled_image_height * scale,
    );
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);
    let mut writer = encoder.write_header()?;
    let mut stream_writer = writer.stream_writer()?;
    let mut bit_writer = BitWriter::new(&mut stream_writer);

    let mut write_row = |f: &mut dyn FnMut(
        u32,
        &mut dyn FnMut(bool) -> Result<(), std::io::Error>,
    ) -> Result<(), std::io::Error>,
                         last: bool|
     -> Result<(), std::io::Error> {
        for _ in 0..scale {
            for x in 0..maze.get_width() {
                f(x, &mut |wall: bool| {
                    for _ in 0..scale {
                        bit_writer.write_bool(!wall)?;
                    }
                    Ok(())
                })?;
            }
            for _ in 0..scale {
                bit_writer.write_bool(!last)?;
            }
            // If the last pixel of a row is not byte-aligned, we need to write the byte containing it to
            // the png stream writer before moving on to the next row. The remaining bits in this last byte are ignored.
            bit_writer.flush()?;
        }
        Ok(())
    };

    for y in 0..maze.get_height() {
        write_row(
            &mut |x, write_cell| {
                write_cell(true)?;
                write_cell(maze.has_wall(x, y, Direction::Up))
            },
            true,
        )?;

        write_row(
            &mut |x, write_cell| {
                write_cell(maze.has_wall(x, y, Direction::Left))?;
                write_cell(false)
            },
            maze.has_wall(maze.get_width() - 1, y, Direction::Right),
        )?;
    }

    write_row(
        &mut |x, write_cell| {
            write_cell(true)?;
            write_cell(maze.has_wall(x, maze.get_height() - 1, Direction::Down))
        },
        true,
    )?;

    stream_writer.finish()?;
    Ok(())
}

struct BitWriter<'a, W: Write> {
    writer: &'a mut W,
    buffer: [u8; 1],
    bits_written: u8,
}

impl<'a, W: Write> BitWriter<'a, W> {
    fn new(writer: &'a mut W) -> BitWriter<W> {
        BitWriter {
            writer,
            buffer: [0],
            bits_written: 0,
        }
    }

    fn write_bool(&mut self, value: bool) -> Result<(), std::io::Error> {
        if value {
            self.buffer[0] |= 0b00000001 << (7 - self.bits_written);
        }
        self.bits_written += 1;
        match self.bits_written {
            8 => self.flush(),
            _ => Ok(()),
        }
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        if self.bits_written > 0 {
            self.writer.write_all(&self.buffer)?;
            self.buffer[0] = 0;
            self.bits_written = 0;
        }
        Ok(())
    }
}
