use std::io::{self, Write};
use termion::cursor;

pub struct TerminalResolution {
    rows: usize,
    columns: usize,
}

impl TerminalResolution {
    pub fn new() -> TerminalResolution {
        let (columns, rows) = termion::terminal_size().expect("Failed to get terminal size");
        TerminalResolution {
            rows: rows as usize,
            columns: columns as usize,
        }
    }

    pub fn get_rows_cols(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

pub struct ArtBuffer {
    buff: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl ArtBuffer {
    pub fn new(res: &TerminalResolution) -> Self {
        // Возможно тут одна строка на всех
        let row = vec![' '; res.columns];
        let rows = vec![row; res.rows];
        Self {
            buff: rows,
            rows: res.rows,
            cols: res.columns,
        }
    }

    pub fn clear(&mut self) {
        for row_num in 0..self.buff.len() {
            let row_len = self.buff[row_num].len();
            for col_num in 0..row_len {
                self.buff[row_num][col_num] = ' ';
            }
        }
    }

    // TODO: сделать дженериком, который бы принимал любой тип x;y и переводил внутри в usize
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        if x < self.cols && y < self.rows {
            self.buff[y][x] = ch;
        }
    }

    fn build(&self) -> String {
        let mut rows: Vec<String> = Vec::with_capacity(self.buff.len());
        let vec_rows = &self.buff;
        for row in vec_rows {
            rows.push(row.iter().collect());
        }
        rows.join("\n")
    }
}

pub struct TerminalArtist {
    was_first_render: bool,
    pub resolution: TerminalResolution,
    pub buffer: ArtBuffer,
}

impl TerminalArtist {
    pub fn new() -> Self {
        let resolution = TerminalResolution::new();
        let buffer = ArtBuffer::new(&resolution);
        Self {
            was_first_render: false,
            resolution,
            buffer,
        }
    }

    pub fn render(&mut self) {
        if !self.was_first_render {
            self.was_first_render = true;
            println!("{}", "-".repeat(self.resolution.columns as usize))
        } else {
            // Курсор в начало
            print!("{}", cursor::Goto(1, 1))
        }
        // Печать собственно собранного изображения
        print!("{}", self.buffer.build());
        io::stdout().flush().unwrap();
    }
}
