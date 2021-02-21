// mod ascii;
mod bitmap;
mod buffer;
mod components;
// mod core;
mod rendering;
// mod scene;
mod rad_sign;
mod tick;
mod vg;

use components::fog::FogLine;
use rad_sign::RadSign;
use rendering::{Art, TerminalArtist, TerminalResolution};
use vg::vectors::Vector2;

// use resize::Type::Lanczos3;
// use resize::Pixel::Gray8;

// use rand::prelude::ThreadRng;
// use vg::{vectors::Vector2, Canvas};

fn main() {
    let mut main_scene = MainScene::new();
    tick::run_tick_loop(100, &mut main_scene);

    // // создаю буфферок
    // let mut cb = buffer::Buffer2D::<u8>::new(500, 500, 0);
    // // создаю знак
    // let sign = rad_sign::RadSignState::new(0.0, 100.0, Vector2::new(250.0, 250.0));
    // // рисую
    // sign.draw_on_canvas(&mut cb);
    // // сохраняю
    // bitmap::save_buff(&cb, "test.bmp");

    // // ресампл
    // let mut cb_small = buffer::Buffer2D::<u8>::new(167, 39, 0);
    // let mut resizer = resize::Resizer::new(
    //     cb.width,
    //     cb.height,
    //     cb_small.width,
    //     cb_small.height,
    //     resize::Pixel::Gray8,
    //     resize::Type::Catrom,
    // )
    // .unwrap();

    // resizer.resize(&cb.buff, &mut cb_small.buff).unwrap();

    // // let v: Vec<_> = cb_small.get_iter().collect();
    // // println!("{:?}", v);
    // for i in cb_small.get_iter() {
    //     if (i.0 != 0) {
    //         println!("{:?}", i)
    //     }
    // }

    // // сохраняю новое
    // bitmap::save_buff(&cb_small, "test_small.bmp");

    // // а теперь порисую на экране

    // let mut artist = rendering::TerminalArtist::new(&resolution);
    // for x in 0..cb_small.width {
    //     for y in 0..cb_small.height {
    //         let px = cb_small.get_px(x as i32, y as i32);
    //         let px_char: char = match px {
    //             Some(px_value) => {
    //                 let norm = (px_value as f64 / 256.0) * 10.0;
    //                 let norm_str = (norm as u32).to_string().as_bytes()[0] as char;
    //                 norm_str
    //             }
    //             None => ' ',
    //         };
    //         artist.buffer.write(x, y, px_char);
    //     }
    // }
    // artist.render();
}

struct MainScene {
    artist: TerminalArtist,
    fog: Vec<FogLine>,
    rad: RadSign,
}

impl MainScene {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let res = TerminalResolution::from_actual_terminal_size();
        let artist = TerminalArtist::new(res);

        // туман
        let fog = FogLine::generate_fog_lines(&mut rng, res.rows, res.columns, 30);

        // знак
        let rad = RadSign::new(0.5, res);

        Self { artist, fog, rad }
    }
}

impl tick::Tick for MainScene {
    fn tick(&mut self, ms: u64) {
        // println!("tick");
        // чистка буфера для начала
        self.artist.buffer.clear();

        // тик и прорисовка линий
        for line in self.fog.iter_mut() {
            line.tick(ms);
            line.draw(&mut self.artist);
        }

        // тик и прорисовка знака
        // println!("and now rad");
        self.rad.tick(ms);
        self.rad.draw(&mut self.artist);
        // println!("rad done")

        // // тик и прорисовка крыльев
        // for wing in wings.iter_mut() {
        //     wing.tick(delta_ms);
        //     wing.draw(&mut self.artist);
        // }

        // рендеринг буффера в терминале
        self.artist.render();
        // panic!()

        // self.num += 1;
    }
}
