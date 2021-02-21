// mod ascii;
mod bitmap;
mod buffer;
// mod components;
mod core;
mod rendering;
// mod scene;
mod rad_sign;
mod vg;

// use resize::Type::Lanczos3;
// use resize::Pixel::Gray8;

use vg::{vectors::Vector2, Canvas};

fn main() {
    // создаю буфферок
    let mut cb = buffer::Buffer2D::<u8>::new(500, 500, 0);

    // создаю знак
    let sign = rad_sign::RadSign::new(0.0, Vector2::new(250.0, 250.0), 200.0);

    // рисую
    sign.draw_on_canvas(&mut cb);

    // беру разрешение терминала
    let resolution = rendering::TerminalResolution::new(40, 100);

    // // сохраняю
    // bitmap::save_buff(&cb, "test.bmp");

    // ресампл
    let mut cb_small = buffer::Buffer2D::<u8>::new(resolution.columns, resolution.rows, 0);
    let mut resizer = resize::Resizer::new(
        cb.width,
        cb.height,
        cb_small.width,
        cb_small.height,
        resize::Pixel::Gray8,
        resize::Type::Catrom,
    )
    .unwrap();

    resizer.resize(&cb.buff, &mut cb_small.buff).unwrap();

    // // сохраняю новое
    // bitmap::save_buff(&cb_small, "test_small.bmp");

    // а теперь порисую на экране

    let mut artist = rendering::TerminalArtist::new(&resolution);
    for x in 0..cb_small.width {
        for y in 0..cb_small.height {
            let px = cb_small.get_px(x as i32, y as i32);
            let px_char: char = match px {
                Some(px_value) => {
                    let norm = (px_value as f64 / 256.0) * 10.0;
                    let norm_str = (norm as u32).to_string().as_bytes()[0] as char;
                    norm_str
                }
                None => ' ',
            };
            artist.buffer.write(x, y, px_char);
        }
    }
    artist.render();
}
