use crate::buffer::Buffer2D;
use crate::rendering::{Art, TerminalArtist, TerminalResolution};
use crate::tick::Tick;
use resize::{formats::Gray, Resizer};
use std::f64::consts::PI;

use crate::vg::{
    draw_arc_with_bold_size, draw_line_with_bold_side, fill_canvas, vectors::Vector2, BoldMode,
    Canvas,
};

/// С какого предела начинается малый радиус (относительно)
const RAD_START_REL: f64 = 0.2;

/// какая угловая длина у одного крыла
const PART_ANGLE: f64 = 2.0 * PI / 3.0 * 0.5;

/// Скорость вращения. Угол в радианах в секунду
const ROTATION_SPEED: f64 = 1.0;

// const SYMBOLS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
// const SYMBOLS: [char; 4] = ['░', '▒', '▓', '█'];
const SYMBOLS: [char; 9] = ['-', '•', '*', '!', '?', '$', '#', '%', '█'];

// // Параметры вырезки. Какой относительный угол и длину они занимают
// // Длина относительна расстояния между RAD_START и RAD_END
// const NOTCH_ANGLE_REL: f64 = 0.5;
// const NOTCH_LEN_REL: f64 = 0.5;

const NOTCH_RAD_START_REL: f64 = 0.15;
const NOTCH_RAD_END_REL: f64 = 0.5;
const NOTCH_OFFSET_REL: f64 = 0.3;

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

        // println!("size {:?}", res);

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
            if let Some(chr) = char_by_value(value) {
                artist.buffer.write(x + self.res.columns / 4, y, chr);
                //                    ^ это потому, что картинка ужата в два раза по оси OX
            }

            // // пока без детализации
            // if value > 0 {
            //     let chr: char = if value < 100 {
            //         '.'
            //     } else if value < 200 {
            //         '+'
            //     } else if value < 255 {
            //         '='
            //     } else {
            //         '#'
            //     };
            //     artist.buffer.write(x + self.res.columns / 4, y, chr);
            // }
        }
    }
}

fn char_by_value(val: u8) -> Option<char> {
    if val > 0 {
        // делаю число относительным к 8, а не к 255
        let val_rel = (val as usize) * SYMBOLS.len() / 256;
        Some(SYMBOLS[val_rel as usize])
    } else {
        None
    }
}

// fn char_by_vertical_tuple((v1, v2, v3): (u8, u8, u8)) -> Option<char> {
//     if val > 0 {
//         // делаю число относительным к 8, а не к 255
//         let val_rel = (val as usize) * SYMBOLS.len() / 256;
//         Some(SYMBOLS[val_rel as usize])
//     } else {
//         None
//     }
// }

fn draw_rad_part(
    canv: &mut impl Canvas,
    center: &Vector2,
    rad_start: f64,
    rad_end: f64,
    ang_start: f64,
    ang_end: f64,
) {
    // нарисовать две арки
    // нарисовать две прямыеInner
    // залить

    // края крыла
    let vert_ss = Vector2::from_polar_coords(ang_start, rad_start) + center;
    let vert_se = Vector2::from_polar_coords(ang_start, rad_end) + center;
    let vert_ee = Vector2::from_polar_coords(ang_end, rad_end) + center;
    let vert_es = Vector2::from_polar_coords(ang_end, rad_start) + center;

    // общие длины, угол и радиус
    let all_len = rad_end - rad_start;
    let all_ang_len = ang_end - ang_start;

    // вырезка
    /* У вырезки тот же угол, но другие радиусы. Также центр вырезки сдвинут
       от центра основного крыла на некое расстояние в направлении биссектрисы
       угла
    */

    // смещение вырезки
    let notch_origin =
        Vector2::from_polar_coords((ang_start + ang_end) / 2.0, rad_end * NOTCH_OFFSET_REL)
            + center;
    // радиусы вырезки
    let notch_rad_start = rad_end * NOTCH_RAD_START_REL;
    let notch_rad_end = rad_end * NOTCH_RAD_END_REL;

    // let notch_len = all_len * NOTCH_LEN_REL;
    // let notch_rad_start = rad_start + all_len / 2.0 - notch_len / 2.0;
    // let notch_rad_end = notch_rad_start + notch_len;
    // let notch_ang_len = all_ang_len * NOTCH_ANGLE_REL;
    // let notch_ang_start = ang_start + all_ang_len / 2.0 - notch_ang_len / 2.0;
    // let notch_ang_end = notch_ang_start + notch_ang_len;

    // края вырезки
    let notch_ss = Vector2::from_polar_coords(ang_start, notch_rad_start) + &notch_origin;
    let notch_se = Vector2::from_polar_coords(ang_start, notch_rad_end) + &notch_origin;
    let notch_ee = Vector2::from_polar_coords(ang_end, notch_rad_end) + &notch_origin;
    let notch_es = Vector2::from_polar_coords(ang_end, notch_rad_start) + &notch_origin;

    // центр крыла
    let mut wing_center = vert_ss.clone();
    wing_center
        .move_to_middle_with(&vert_se)
        .move_to_middle_with(&vert_ee)
        .move_to_middle_with(&vert_es);

    // центр вырезки
    let mut notch_center = notch_ss.clone();
    notch_center
        .move_to_middle_with(&notch_se)
        .move_to_middle_with(&notch_ee)
        .move_to_middle_with(&notch_es);

    // арки рисую основные
    draw_arc_with_bold_size(
        canv,
        center,
        rad_start,
        ang_start,
        all_ang_len,
        BoldMode::Outer,
    );
    draw_arc_with_bold_size(
        canv,
        center,
        rad_end,
        ang_start,
        all_ang_len,
        BoldMode::Inner,
    );

    // арки рисую не основные
    draw_arc_with_bold_size(
        canv,
        &notch_origin,
        notch_rad_start,
        ang_start,
        all_ang_len,
        BoldMode::Inner,
    );
    draw_arc_with_bold_size(
        canv,
        &notch_origin,
        notch_rad_end,
        ang_start,
        all_ang_len,
        BoldMode::Outer,
    );

    // линии основные
    draw_line_with_bold_side(canv, &vert_ss, &vert_se, &wing_center, BoldMode::Inner);
    draw_line_with_bold_side(canv, &vert_es, &vert_ee, &wing_center, BoldMode::Inner);

    // линии вырезки
    draw_line_with_bold_side(canv, &notch_ss, &notch_se, &notch_center, BoldMode::Outer);
    draw_line_with_bold_side(canv, &notch_es, &notch_ee, &notch_center, BoldMode::Outer);

    // залить
    // ищу центр для заливки
    let mut fill_center = vert_ss.clone();
    fill_center
        .move_to_middle_with(&vert_es)
        .move_to_middle_with(&notch_ss)
        .move_to_middle_with(&notch_es);
    fill_canvas(canv, (fill_center.x as i32, fill_center.y as i32), 255);
}
