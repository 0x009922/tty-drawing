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
    buff: Vec<Vec<ArtPixel>>,
    rows: usize,
    cols: usize,
    // backface: char
}

pub enum ArtPixel {
    Simple(char),
    Composed(String),
}

impl ArtBuffer {
    pub fn new(res: &TerminalResolution) -> Self {
        let mut buff = Vec::with_capacity(res.rows);
        for _ in 0..res.rows {
            let mut row_vec = Vec::with_capacity(res.columns);
            for _ in 0..res.columns {
                row_vec.push(ArtPixel::Simple(' '))
            }
            buff.push(row_vec);
        }

        Self {
            buff,
            rows: res.rows,
            cols: res.columns,
            // backface: ' '
        }
    }

    // pub fn set_backface(&mut self, val: char) {
    //     self.backface = val;
    // }

    pub fn clear(&mut self) {
        for row_num in 0..self.buff.len() {
            let row_len = self.buff[row_num].len();
            for col_num in 0..row_len {
                self.buff[row_num][col_num] = ArtPixel::Simple(' ');
            }
        }
    }

    // TODO: сделать дженериком, который бы принимал любой тип x;y и переводил внутри в usize
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        if x < self.cols && y < self.rows {
            self.buff[y][x] = ArtPixel::Simple(ch);
        }
    }

    pub fn write_composed(&mut self, x: usize, y: usize, val: String) {
        if x < self.cols && y < self.rows {
            self.buff[y][x] = ArtPixel::Composed(val);
        }
    }

    fn build(&self) -> String {
        let mut rows: Vec<String> = Vec::with_capacity(self.buff.len());

        for row in &self.buff {
            let row_len = row.iter().fold(0, |acc, x| {
                let len = match x {
                    ArtPixel::Simple(_) => 1,
                    ArtPixel::Composed(x) => x.len(),
                };
                len + acc
            });

            let mut row_str = String::with_capacity(row_len);

            for pixel in row.iter() {
                match pixel {
                    ArtPixel::Simple(val) => row_str.push(*val),
                    ArtPixel::Composed(val) => {
                        row_str.push_str(&val);
                    }
                }
            }

            rows.push(row_str);
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
