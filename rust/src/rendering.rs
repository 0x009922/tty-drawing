use crate::buffer::Buffer2D;
use std::io::{self, Write};
use termion::cursor;

/// Разрешение терминала - сколько строчек и колонок в нём
pub struct TerminalResolution {
    pub rows: usize,
    pub columns: usize,
}

impl TerminalResolution {
    pub fn from_actual_terminal_size() -> TerminalResolution {
        let (columns, rows) = termion::terminal_size().expect("Failed to get terminal size");
        TerminalResolution {
            rows: rows as usize,
            columns: columns as usize,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }

    pub fn get_rows_cols(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

/// Основной буффер, который используется рисовальщиком в терминале
pub struct ArtBuffer {
    buff: Buffer2D<char>,
    // rows: usize,
    // cols: usize,
    // // backface: char
}

// pub enum ArtPixel {
//     Simple(char),
//     Composed(String),
// }

impl ArtBuffer {
    pub fn new(res: &TerminalResolution) -> Self {
        let buff = Buffer2D::new(res.columns, res.rows, ' ');

        Self { buff }
    }

    // pub fn set_backface(&mut self, val: char) {
    //     self.backface = val;
    // }

    pub fn clear(&mut self) {
        self.buff.clear(None);
        // for row_num in 0..self.buff.len() {
        //     let row_len = self.buff[row_num].len();
        //     for col_num in 0..row_len {
        //         self.buff[row_num][col_num] = ArtPixel::Simple(' ');
        //     }
        // }
    }

    // TODO: сделать дженериком, который бы принимал любой тип x;y и переводил внутри в usize
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        self.buff.set_by_ucoords((x, y), ch);
        // if x < self.cols && y < self.rows {
        //     self.buff[y][x] = ArtPixel::Simple(ch);
        // }
    }

    // pub fn write_composed(&mut self, x: usize, y: usize, val: String) {
    //     if x < self.cols && y < self.rows {
    //         self.buff[y][x] = ArtPixel::Composed(val);
    //     }
    // }

    fn build(&self) -> String {
        // создаю вектор с заранее известным кол-вом строчек
        // потом их соединю символом переноса строки
        let mut rows: Vec<String> = Vec::with_capacity(self.buff.height);

        // бегаю построчно
        for row_num in 0..self.buff.height {
            let start_index = row_num * self.buff.width;

            // беру конкретные символы, относящиеся к строке
            let row_slice = &self.buff.buff[start_index..start_index + self.buff.width];

            // соединяю символы в строку
            let collected: String = row_slice.iter().collect();

            // кладу в список
            rows.push(collected);
        }

        // for row in &self.buff {
        //     let row_len = row.iter().fold(0, |acc, x| {
        //         let len = match x {
        //             ArtPixel::Simple(_) => 1,
        //             ArtPixel::Composed(x) => x.len(),
        //         };
        //         len + acc
        //     });

        //     let mut row_str = String::with_capacity(row_len);

        //     for pixel in row.iter() {
        //         match pixel {
        //             ArtPixel::Simple(val) => row_str.push(*val),
        //             ArtPixel::Composed(val) => {
        //                 row_str.push_str(&val);
        //             }
        //         }
        //     }

        //     rows.push(row_str);
        // }

        rows.join("\n")
    }
}

/// Рисовальщик в терминале
pub struct TerminalArtist<'a> {
    was_first_render: bool,
    pub resolution: &'a TerminalResolution,
    pub buffer: ArtBuffer,
}

impl<'a> TerminalArtist<'a> {
    /// инициализация
    pub fn new(resolution: &'a TerminalResolution) -> Self {
        // let resolution = TerminalResolution::new();
        let buffer = ArtBuffer::new(&resolution);
        Self {
            was_first_render: false,
            resolution,
            buffer,
        }
    }

    /// очистка экрана и рендеринг буффера
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
