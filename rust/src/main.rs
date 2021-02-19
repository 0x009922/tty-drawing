// mod ascii;
mod bitmap;
// mod components;
// mod core;
// mod rendering;
// mod scene;
mod rad_sign;
mod vg;

use vg::vectors::Vector2;

fn main() {
    // создаю буфферок
    let mut cb = vg::buffer::CanvasBuffer::new(500, 500);

    // создаю знак
    let sign = rad_sign::RadSign::new(0.0, Vector2::new(250.0, 250.0), 100.0);

    // рисую
    sign.draw_on_canvas(&mut cb);

    // сохраняю
    bitmap::save_buff(&cb, "test.bmp");
}
