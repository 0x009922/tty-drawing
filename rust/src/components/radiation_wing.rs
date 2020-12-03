use std::collections::HashMap;
use std::f32::consts::PI;

use crate::core::*;
use crate::rendering::*;

const Y_SCALE_FACTOR: f32 = 0.5;

const RADWING_RADIUS: (f32, f32) = (5.0, 25.0);
const RADWING_WIDTH_ANGLE: f32 = PI / 3.0 * 0.8;
const RADWING_ROTATION_SPEED: f32 = 0.5;

pub struct RadiationWing {
    current_angle: f32,
    buff: VirtualBuffer, // rad_min: f32,
                         // rad_max: f32,
                         // width_angle: f32
}

impl RadiationWing {
    pub fn new(angle: f32, res: &TerminalResolution) -> Self {
        let (rows, cols) = res.get_rows_cols();

        let cx = cols as f32 / 2.0;
        let cy = rows as f32 / 2.0;
        let mut buff = VirtualBuffer::new();
        buff.set_offset(cx as i32, cy as i32);

        // если крыло будет рисоваться сразу, до первого тика,
        // то стоит сделать update_buffer прямо тут

        Self {
            current_angle: angle,
            buff,
        }
    }

    // fn draw_point(&self, buff: &mut ArtBuffer, angle: f32, radius: f32) {
    //     let (x, y) = rad_coords_to_cartesian(angle, radius);
    //     let x = (x + self.cx).round();
    //     let y = (y * Y_SCALE_FACTOR + self.cy).round();
    //     if x >= 0.0 && y >= 0.0 {
    //         buff.write(x as usize, y as usize, '*');
    //     }
    // }

    fn update_buffer(&mut self) {
        // чистка буффера
        self.buff.clear();

        // данные для рисования
        let a1 = self.current_angle;
        let a2 = a1 + RADWING_WIDTH_ANGLE;
        let (r1, r2) = RADWING_RADIUS;

        // рисование арок
        draw_arc(&mut self.buff, r1, a1, a2);
        draw_arc(&mut self.buff, r2, a1, a2);

        // края, углы
        // draw_rad_bounds(&mut self.buff, a1, a2, r1, r2);

        // боковые линии
        draw_side(&mut self.buff, a1, r1, r2);
        draw_side(&mut self.buff, a2, r1, r2);

        // заливка
        {
            // беру центр
            // let p1 = rad_coords_to_cartesian(a1, r1);
            // let p2 = rad_coords_to_cartesian(a2, r2);
            let (x, y) = rad_wing_center(a1, a2, r1, r2);

            // println!("{:?}", ((a1, r1), (a2, r2), p1, p2, (x, y)));
            // panic!("debug");

            // заливаю
            self.buff.fill(x as i32, (y * Y_SCALE_FACTOR) as i32, '#');
        }
    }
}

/// Нахождение центра крыла радиационного
///
/// Берутся его границы со всех сторон и у них берётся центр
fn rad_wing_center(a1: f32, a2: f32, r1: f32, r2: f32) -> (f32, f32) {
    let mut pmin: (f32, f32) = rad_coords_to_cartesian(a1, r1);
    let mut pmax: (f32, f32) = rad_coords_to_cartesian(a1, r1);

    for a in [a1, a2].iter() {
        for r in [r1, r2].iter() {
            let (x, y) = rad_coords_to_cartesian(*a, *r);
            // выставляю углы
            pmin = (floats_min(x, pmin.0), floats_min(y, pmin.1));
            pmax = (floats_max(x, pmax.0), floats_max(y, pmax.1));
        }
    }

    coords_center(pmin, pmax)
}

fn coords_center((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> (f32, f32) {
    ((x1 + x2) / 2.0, (y1 + y2) / 2.0)
}

impl Tick for RadiationWing {
    fn tick(&mut self, ms: u32) {
        // поворот крыла
        self.current_angle += ms as f32 * 0.001 * RADWING_ROTATION_SPEED;
        self.current_angle %= PI * 2.0;

        // рисование новой текстуры
        self.update_buffer();
    }
}

impl Art for RadiationWing {
    fn draw(&self, artist: &mut TerminalArtist) {
        self.buff.write_to_buff(&mut artist.buffer);
    }
}

fn draw_arc(buff: &mut VirtualBuffer, radius: f32, a1: f32, a2: f32) {
    /*
    надо пройтись от начальной точки до конечной по кругу
    дельта при этом должна быть одинакова что для маленького круга, что для большого,
    то есть должна зависеть от радиуса линейно

    l = 2 * PI * Radius

    чтобы перемещаться на пиксель каждый раз, нужен такой угол:

    dl = 1
    da / dl = 2 * PI / l
    da / 1 = (2 * PI) / (2 * PI * Radius)
    da = 1 / Radius

    чтобы перемещаться на долю пикселя (dl)

    da / dl = 1 / Radius
    da = dl / Radius
    */

    let da = 0.5 / radius;

    // // проверка на проход в обратную сторону
    // if a2 < a1 {
    //     da *= -1.0;
    // }

    let min_angle = floats_min(a1, a2);
    let max_angle = floats_max(a1, a2);

    // текущий угол
    let mut a = min_angle;
    let mut coords = rad_coords_to_cartesian(a, radius);

    while a <= max_angle {
        // закрашивание
        let x = coords.0.round() as i32;
        let y = (coords.1 * Y_SCALE_FACTOR).round() as i32;
        let ch = angle_to_line_char(-(a + PI / 2.0));
        // if a < PI / 2.0 && a > (PI / 4.0) {
        //     println!("{};{} a = {} - '{}'", x, y, a, ch);
        //     panic!("debug");
        // }
        buff.set_px(x, y, Some(ch));

        // переход дальше
        let mut next_coords = coords;
        while next_coords == coords && a <= max_angle {
            a += da;
            next_coords = rad_coords_to_cartesian(a, radius);
        }
        coords = next_coords;
    }
}

fn draw_side(buff: &mut VirtualBuffer, a: f32, r1: f32, r2: f32) {
    let side_char = angle_to_line_char(-a);

    const DR: f32 = 1.0;
    let mut r = r1;
    let mut coords = rad_coords_to_cartesian(a, r);

    while r <= r2 {
        // закрашиваю
        let x = coords.0.round() as i32;
        let y = (coords.1 * Y_SCALE_FACTOR).round() as i32;
        buff.set_px(x, y, Some(side_char));

        // дальше
        let mut next_coords = coords;
        while next_coords == coords {
            r += DR;
            next_coords = rad_coords_to_cartesian(a, r);
        }
        coords = next_coords;
    }
}

fn floats_min(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}

fn floats_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn rad_coords_to_cartesian(angle: f32, radius: f32) -> (f32, f32) {
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    (x, y)
}

// fn float_coords_to_integer((x, y): (f32, f32)) -> (i32, i32) {
//     (x as i32, y as i32)
// }

fn angle_to_line_char(val: f32) -> char {
    // нормализую до положительного в пределах [0; PI]
    let mut normalized = val % PI;
    if normalized < 0.0 {
        normalized += PI;
    }

    // теперь количество целых четвертей PI
    let quarts = ((normalized / (PI / 4.0)).round() as i32) % 4;

    // в зависимости от того, в какой "четверти", отдаю нужный символ
    match quarts {
        0 => '=',
        1 => '/',
        2 => '|',
        // 3
        _ => '\\',
        // invalid => panic!("out of range: {}", invalid)
    }
}

// тесты для рисования чёрточки
// TODO тестирование того, что она округляет значения к ближайшему

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_0_4() {
        assert_eq!(angle_to_line_char(0.0), '=');
    }

    #[test]
    fn angle_1_4() {
        assert_eq!(angle_to_line_char(PI / 4.0), '/');
    }

    #[test]
    fn angle_min_1_4() {
        assert_eq!(angle_to_line_char(-PI / 4.0), '\\');
    }

    #[test]
    fn angle_2_4() {
        assert_eq!(angle_to_line_char(PI / 2.0), '|');
    }

    #[test]
    fn angle_3_4() {
        assert_eq!(angle_to_line_char((PI / 4.0) * 3.0), '\\');
    }

    #[test]
    fn angle_13_4() {
        assert_eq!(angle_to_line_char((PI / 4.0) * 13.0), '/');
    }

    #[test]
    fn rounding_0() {
        assert_eq!(angle_to_line_char(PI * 0.25 + 0.1), '/');
        assert_eq!(angle_to_line_char(PI * 0.25 - 0.1), '/');
    }

    #[test]
    fn rounding_1() {
        assert_eq!(angle_to_line_char(PI * 0.5 + 0.1), '|');
        assert_eq!(angle_to_line_char(PI * 0.5 - 0.1), '|');
    }
}

// *** abstract buffer

struct VirtualBuffer {
    map: HashMap<(i32, i32), Option<char>>,
    offset: (i32, i32),
    bounds: VirtualBufferBounds,
}

struct VirtualBufferBounds {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

const NEIGHBOR_VARIANTS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl VirtualBuffer {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            offset: (0, 0),
            bounds: VirtualBufferBounds {
                x1: -100,
                y1: -100,
                x2: 100,
                y2: 100,
            },
        }
    }

    fn clear(&mut self) {
        // TODO: можно не чистить, а всё устанавливать в None. Так избегаются реаллокейты?
        self.map.clear();
    }

    /// Установка смещения, которое будет использовано при заполнении
    /// реального буффера
    fn set_offset(&mut self, x: i32, y: i32) {
        self.offset = (x, y);
    }

    /// Зарисовка одной точки
    fn set_px(&mut self, x: i32, y: i32, val: Option<char>) {
        let current_val = self.map.entry((x, y)).or_insert(None);
        *current_val = val;
    }

    /// Взятия данных пикселя
    fn get_px(&self, x: i32, y: i32) -> &Option<char> {
        match self.map.get(&(x, y)) {
            Some(v) => v,
            None => &None,
        }
    }

    // fn debug_print(&self) {
    //     println!("Debugging VirtualBuffer");
    //     let keys: Vec<&(i32,  i32)> = self.map.keys().collect();
    //     if keys.len() == 0 {
    //         println!("It is empty");
    //         return
    //     }

    //     // углы
    //     let mut leftup = *keys[0];
    //     let mut rightdown = leftup;
    //     for (x, y) in keys {
    //         leftup = (cmp::min(*x, leftup.0), cmp::min(*y, leftup.1));
    //         rightdown = (cmp::max(*x, rightdown.0), cmp::max(*y, rightdown.1));
    //     }

    //     println!("From: {:?}, To: {:?}", leftup, rightdown);
    //     println!("{}", "*".repeat((rightdown.0 - leftup.0 + 4) as usize));
    //     // println!("");

    //     for y in leftup.1..(rightdown.1 + 1) {
    //         print!("* ");
    //         for x in leftup.0..(rightdown.0 + 1) {
    //             let pixel: char = match self.map.get(&(x, y)) {
    //                 Some(Some(v)) => *v,
    //                 _ => ' '
    //             };
    //             print!("{}", pixel);
    //         }
    //         print!(" *");
    //         println!("");
    //     }

    //     // println!("");
    //     println!("{}", "*".repeat((rightdown.0 - leftup.0 + 4) as usize));
    // }

    /// Заливка всех None-пикселей, начиная с переданной координаты
    /// TODO оптимизировать
    fn fill(&mut self, x: i32, y: i32, val: char) {
        // self.debug_print();
        // println!("{:?} - {}", (x, y), val);
        // panic!("debug");

        // закраска сразу
        self.set_px(x, y, Some(val));

        // пробегаюсь по вариантам соседей
        for (dx, dy) in NEIGHBOR_VARIANTS.iter() {
            // беру координаты соседа
            let (x, y) = (x + dx, y + dy);

            if self.is_in_the_bounds(x, y) {
                let px_val = self.get_px(x, y);
                match px_val {
                    // если координаты в пределах буффера и если на их месте стоит None, то закрашиваю
                    None => self.fill(x, y, val),
                    Some(_) => (),
                }
            }
        }
    }

    /// отрисовка заполненных данных в реальном буффере
    fn write_to_buff(&self, buff: &mut ArtBuffer) {
        for ((x, y), val) in self.map.iter() {
            let x = *x + self.offset.0;
            let y = *y + self.offset.1;

            if x >= 0 && y >= 0 {
                if let Some(v) = val {
                    buff.write(x as usize, y as usize, *v);
                }
            }
        }
    }

    /// Проверка, находятся ли координаты в пределах размеров буффера
    fn is_in_the_bounds(&self, x: i32, y: i32) -> bool {
        x >= self.bounds.x1 && x <= self.bounds.x2 && y >= self.bounds.y1 && y <= self.bounds.y2
    }
}
