#![feature(assoc_char_funcs)]

mod ascii;
mod bitmap;
mod components;
mod core;
mod rendering;
mod scene;

fn main() {
    for i in 0..64 {
        println!("{}: {}", i, ascii::num_to_char(i));
    }

    // println!("{}", 14 as bool)
    // let num: u8 = 64;

    // println!("Char for {}: {:?}", num, ascii::AsciiMatrix::from_num(num));
    // bitmap::create();
}
