use crate::buffer::Buffer2D;
use crate::vg::Canvas;
use rustbitmap::{BitMap, Rgba};

// pub struct BitmapCanvas {
//     bmp: BitMap,
// }

// impl Canvas for BitmapCanvas {
//     fn put_px(&mut self, x: u32, y: u32, v: u8) {
//         println!("put px {} {} {}", x, y, v);
//         self.bmp.set_pixel(x, y, value_to_color(v)).unwrap();
//     }

//     fn get_px(&self, x: u32, y: u32) -> u8 {
//         0
//     }
// }

// trait Drawer {
//     fn put_px(&mut self, x: u32, y: u32, v: u8);

//     fn fill(&mut self, x: u32, y: u32, v: u8);
// }

// struct BitmapDrawer {
//     bmp: BitMap,
// }

// impl Drawer for BitmapDrawer {
//     fn put_px(&mut self, x: u32, y: u32, v: u8) {
//         println!("put px {} {} {}", x, y, v);
//         self.bmp.set_pixel(x, y, value_to_color(v)).unwrap();
//     }

//     fn fill(&mut self, x: u32, y: u32, v: u8) {
//         self.bmp.fill_region(x, y, value_to_color(v)).unwrap();
//     }
// }

fn value_to_color(v: u8) -> Rgba {
    Rgba::rgba(255 - v, 255 - v, 255 - v, 255)
}

pub fn save_buff(buff: &Buffer2D<u8>, filename: &str) {
    let mut bmp = BitMap::new(buff.width as u32, buff.height as u32);
    for x in 0..buff.width as i32 {
        for y in 0..buff.height as i32 {
            let px = buff.get_px(x, y).unwrap();
            bmp.set_pixel(x as u32, y as u32, value_to_color(px))
                .unwrap();
        }
    }

    bmp.save_as(filename).unwrap();
}

/*
нужно что
А то, чтобы был сделан асбтрактный интерфейс для рисования на нём
по-пиксельно. Чтобы можно было вставить в пиксель цвет, и узнать, какой в нём цвет
*/

// type Point = (f64, f64);

// fn points_distance(a: &Point, b: &Point) -> f64 {
//     ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
// }

// fn polar_to_cartesian(r: f64, a: f64) -> Point {
//     (r * a.cos(), r * a.sin())
// }

// fn add_to_point(p1: &Point, p2: &Point) -> Point {
//     (p1.0 + p2.0, p1.1 + p2.1)
// }

// fn draw_line<T: Drawer>(drawer: &mut T, from: Point, to: Point) {
//     for ((x, y), value) in line_drawing::XiaolinWu::<f64, i32>::new(from, to) {
//         drawer.put_px(x as u32, y as u32, (value * 255.0) as u8);
//     }
// }
