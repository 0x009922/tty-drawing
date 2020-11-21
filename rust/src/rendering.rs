// mod composition;
use termion::cursor;
use std::io::{self, Write};
use std::cmp;
// use rendering::c
pub use composition::*;


pub mod composition {
    // pub trait CompositionProvider {
    //     fn get_composition(&self) -> &Vec<CompositionItem>;
    // }

    // pub enum CompositionItem {
    //     Image(Image),
    //     // AnotherProvider(CompositionProvider)
    //     // Window(Window),
    //     // Offset(Offset)
    // }

    pub struct Image {
        pub x: i32,
        pub y: i32,
        pub lines: Vec<Vec<ImageAtom>>
    }

    pub struct ImageAtom(pub Option<char>);

    // pub struct CompositionIterator(dyn Iterator<Item=CompositionItem>);

    // pub struct Composition<T: Iterator> {
    //     pub items: Vec<T>
    // }
}

pub struct TerminalResolution {
    rows: usize,
    columns: usize
}

impl TerminalResolution {
    pub fn new() -> TerminalResolution {
        let (columns, rows) = termion::terminal_size().expect("Failed to get terminal size");
        TerminalResolution {
            rows: rows as usize,
            columns: columns as usize
        }
    }
}

pub struct TerminalRenderer {
    was_first_render: bool,
    resolution: TerminalResolution,
    buffer: PaintBuffer
}

// pub struct Offset {
//     x: i32,
//     y: i32,
//     composition: Composition
// }

// pub struct Window {
//     frame: Frame,
//     composition: Composition
// }

// #[derive(Debug, PartialEq)]
// pub struct Image {
//     pub x: i32,
//     pub y: i32,
//     pub lines: Vec<Vec<ImageAtom>>
// }

// #[derive(Debug, PartialEq)]
// pub struct ImageAtom(pub Option<char>);

// impl ImageAtom {
//     pub fn
// }

// pub enum CompositionItem{
//     Image(Image),
//     Window(Window),
//     Offset(Offset)
// }

// pub trait Composable {
//     fn get_composition_item(&self) -> &CompositionItem;
// }

struct Frame {
    x: i32,
    y: i32,
    w: usize,
    h: usize
}

impl Frame {
    pub fn from_resolution(res: &TerminalResolution) -> Frame {
        Frame {
            x: 0,
            y: 0,
            w: res.columns,
            h: res.rows
        }
    }

    fn merge_with_child(&self, child: &Frame) -> Option<Frame> {
        let mut axes_intersections: Vec<Section> = Vec::with_capacity(2);
        let preset = [
            (self.x, self.w, child.x, child.w),
            (self.y, self.h, child.y, child.h),
        ];
        for i in 0..2 {
            let (a, b, c, d) = preset[i];
            let result = frames_to_abs_and_intersect(a, b, c, d);
            match result {
                Some(section) => axes_intersections.push(section),
                None => return None,
            }
        };
        let (x, w) = abs_coords_to_origin_len(&axes_intersections[0]);
        let (y, h) = abs_coords_to_origin_len(&axes_intersections[1]);
        Some(Frame { x, w, y, h })
    }
}

fn frames_to_abs_and_intersect(parent_v: i32, parent_len: usize, child_v: i32, child_len: usize) -> Option<Section> {
    let a1 = parent_v;
    let a2 = a1 + (parent_len as i32);
    let b1 = a1 + child_v;
    let b2 = b1 + (child_len as i32);
    Section::intersection(&Section(a1, a2), &Section(b1, b2))
}

fn abs_coords_to_origin_len(Section(a1, a2): &Section) -> (i32, usize) {
    // Могу спокойно преобразовывать к usize в силу того,
    // что a2 должно быть больше a1 (отрезок)
    (*a1, (a2 - a1) as usize)
}



struct PaintBuffer {
    buff: Vec<Vec<char>>
}

impl PaintBuffer {
    fn new(res: &TerminalResolution) -> PaintBuffer {
        // Возможно тут одна строка на всех
        let row = vec![' '; res.columns];
        let rows = vec![row; res.rows];
        PaintBuffer { buff: rows }
    }

    fn clear(&mut self) {
        for row_num in 0..self.buff.len() {
            let row_len = self.buff[row_num].len();
            for col_num in 0..row_len {
                self.buff[row_num][col_num] = ' ';
            }
        }
    }

    fn write(&mut self, x: usize, y: usize, ch: char) {
        self.buff[y][x] = ch;
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


// fn build_composition(comp: &Composition, res: &TerminalResolution) -> String {
//     let line = "#".repeat(res.columns as usize);
//     let v = vec![line; res.rows as usize];
//     v.join("\n")
// }

impl TerminalRenderer {
    pub fn new() -> TerminalRenderer {
        let resolution = TerminalResolution::new();
        let buffer = PaintBuffer::new(&resolution);
        TerminalRenderer {
            was_first_render: false,
            resolution,
            buffer
        }
    }

    pub fn render(&mut self, image: &Image) {
        if !self.was_first_render {
            self.was_first_render = true;
            println!("{}", "-".repeat(self.resolution.columns as usize))
        } else {
            // Курсор в начало
            print!("{}", cursor::Goto(1, 1))
        }
        // Печать собственно собранного изображения
        print!("{}", self.build_str(image));
        io::stdout().flush().unwrap();
    }

    fn build_str(&mut self, image: &Image) -> String {
        self.buffer.clear();
        let init_frame = Frame::from_resolution(&self.resolution);
        TerminalRenderer::build_recursive(&mut self.buffer, image, &init_frame);
        self.buffer.build()
    }

    fn build_recursive(buff: &mut PaintBuffer, image: &Image, frame: &Frame) {
        fill_image(buff, image, frame);
        // for i in provider.get_composition() {
        //     match i {
        //         CompositionItem::Image(img) => {
        //             fill_image(buff, &img, frame);
        //         },
        //         CompositionItem::Window(window) => {
        //             if let Some(child_frame) = frame.merge_with_child(&window.frame) {
        //                 TerminalRenderer::build_recursive(buff, &window.composition, &child_frame)
        //             }
        //             // match frame.merge_with_child(&window.frame) {
        //             //     Some(child_frame) => {
        //             //         TerminalRenderer::build_recursive(buff, &window.composition, &child_frame)
        //             //     }
        //             // }
        //             // if let Some<Frame> = frame.merge_with_child(child: &Frame) {

        //             // }
        //         },
        //         CompositionItem::Offset(offset) => {
        //             let child_frame = Frame {
        //                 x: frame.x + offset.x,
        //                 y: frame.y + offset.y,
        //                 w: frame.w,
        //                 h: frame.h
        //             };
        //             TerminalRenderer::build_recursive(buff, &offset.composition, &child_frame)
        //         }
        //     }
        // }
    }
}


// TODO - переписать функционально, рекурсивно циклы
fn fill_image(buff: &mut PaintBuffer, img: &Image, frame: &Frame) {
    let mut i: usize = 0;
    let mut y = img.y;
    if y < 0 {
        i += (-y) as usize;
        y = 0;
    }
    let lines_count = img.lines.len();
    while i < lines_count && y < frame.h as i32 {
        let line = &img.lines[i];
        let line_len = line.len();
        let mut j: usize = 0;
        let mut x = img.x;
        if x < 0 {
            j += (-x) as usize;
            x = 0;
        }
        while j < line_len && x < frame.w as i32 {
            let atom = &line[j];
            if let ImageAtom(Some(ch)) = atom {
                let abs_x: usize = (x + frame.x) as usize;
                let abs_y: usize = (y + frame.y) as usize;
                buff.write(abs_x, abs_y, *ch);
            }
            j += 1;
            x += 1;
        }
        i += 1;
        y += 1;
    }
}

struct Section(i32, i32);

impl Section {
    fn intersection(a: &Section, b: &Section) -> Option<Section> {
        let (Section(a1, a2), Section(b1, b2)) = (a, b);
        if a2 < b1 || b2 < a1 {
            None
        } else {
            Some(Section(cmp::max(*a1, *b1), cmp::max(*a2, *b2)))
        }
    }
}
