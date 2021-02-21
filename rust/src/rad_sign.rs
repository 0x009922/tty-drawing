use crate::buffer::Buffer2D;
use crate::rendering::{Art, TerminalArtist, TerminalResolution};
use crate::tick::Tick;
use resize::{formats::Gray, Resizer};
use std::f64::consts::PI;

use crate::vg::{
    draw_arc_with_bold_size, draw_line_with_bold_side, fill_canvas, vectors::Vector2, ArcBoldMode,
    Canvas,
};

/// С какого предела начинается малый радиус (относительно)
const RAD_START_REL: f64 = 0.2;

/// какая угловая длина у одного крыла
const PART_ANGLE: f64 = 2.0 * PI / 3.0 * 0.5;

/// Скорость вращения. Угол в радианах в секунду
const ROTATION_SPEED: f64 = 1.0;

/// Радиационный знак!
pub struct RadSign {
    resizer: Resizer<Gray<u8, u8>>,
    draw_buff: Buffer2D<u8>,
    resize_buff: Buffer2D<u8>,
    state: RadSignState,
    res: TerminalResolution,
}

pub struct RadSignState {
    angle: f64,
    center: Vector2,
    radius: f64,
}

impl RadSignState {
    pub fn new(angle: f64, radius: f64, center: Vector2) -> Self {
        Self {
            angle,
            radius,
            center,
        }
    }

    pub fn draw_on_canvas(&self, canv: &mut impl Canvas) {
        // рисую все три куска
        let rad_start = self.radius * RAD_START_REL;
        const DELTA_ANGLE: f64 = 2.0 * PI / 3.0;

        let mut a = self.angle;
        for _ in 0..3 {
            draw_rad_part(
                canv,
                &self.center,
                rad_start,
                self.radius,
                a,
                a + PART_ANGLE,
            );
            a += DELTA_ANGLE;
        }
    }
}

impl RadSign {
    /// инициализация
    pub fn new(start_angle: f64, res: TerminalResolution) -> Self {
        let draw_buff: Buffer2D<u8> = Buffer2D::new(500, 500, 0);
        let resize_buff: Buffer2D<u8> = Buffer2D::new(res.columns / 2, res.rows, 0);

        println!("size {:?}", res);

        let resizer = resize::Resizer::new(
            draw_buff.width,
            draw_buff.height,
            resize_buff.width,
            resize_buff.height,
            resize::Pixel::Gray8,
            resize::Type::Catrom,
        )
        .unwrap();

        Self {
            resizer,
            draw_buff,
            resize_buff,
            res,

            state: RadSignState {
                angle: start_angle,
                center: Vector2::new(250.0, 250.0),
                radius: 200.0,
            },
        }
    }
}

impl Tick for RadSign {
    fn tick(&mut self, ms: u64) {
        // двигаю угол
        self.state.angle += ms as f64 * 0.001 * ROTATION_SPEED;

        // рисую в буффера
        self.draw_buff.clear(None);
        self.state.draw_on_canvas(&mut self.draw_buff);
        self.resizer
            .resize(&self.draw_buff.buff, &mut self.resize_buff.buff)
            .unwrap();
    }
}

impl Art for RadSign {
    fn draw(&self, artist: &mut TerminalArtist) {
        // изливаю на артиста то, что у меня подготовлено в resize_buffer
        for (value, (x, y)) in self.resize_buff.get_iter() {
            // пока без детализации
            if value > 0 {
                let chr: char = if value < 100 {
                    '.'
                } else if value < 200 {
                    '+'
                } else if value < 255 {
                    '='
                } else {
                    '#'
                };
                artist.buffer.write(x + self.res.columns / 4, y, chr);
            }
        }
    }
}

fn draw_rad_part(
    canv: &mut impl Canvas,
    center: &Vector2,
    rad_start: f64,
    rad_end: f64,
    ang_start: f64,
    ang_end: f64,
) {
    // нарисовать две арки
    // нарисовать две прямые
    // залить

    // края крыла
    let vert_ss = Vector2::from_polar_coords(ang_start, rad_start) + center;
    let vert_se = Vector2::from_polar_coords(ang_start, rad_end) + center;
    let vert_ee = Vector2::from_polar_coords(ang_end, rad_end) + center;
    let vert_es = Vector2::from_polar_coords(ang_end, rad_start) + center;

    // центр крыла

    let mut wing_center = vert_ss.clone();
    wing_center
        .move_to_middle_with(&vert_se)
        .move_to_middle_with(&vert_ee)
        .move_to_middle_with(&vert_es);

    // арки
    draw_arc_with_bold_size(
        canv,
        center,
        rad_start,
        ang_start,
        ang_end - ang_start,
        ArcBoldMode::Outer,
    );
    draw_arc_with_bold_size(
        canv,
        center,
        rad_end,
        ang_start,
        ang_end - ang_start,
        ArcBoldMode::Inner,
    );

    // линии
    draw_line_with_bold_side(canv, &vert_ss, &vert_se, &wing_center);
    draw_line_with_bold_side(canv, &vert_es, &vert_ee, &wing_center);

    // залить
    fill_canvas(canv, (wing_center.x as i32, wing_center.y as i32), 255);
}
