use core::f64::consts::PI;
use line::Line;
use rustbitmap::{BitMap, Rgba};
use vectors::{PolarVec, Vector2};

trait Drawer {
    fn put_px(&mut self, x: u32, y: u32, v: u8);

    fn fill(&mut self, x: u32, y: u32, v: u8);
}

struct BitmapDrawer {
    bmp: BitMap,
}

impl Drawer for BitmapDrawer {
    fn put_px(&mut self, x: u32, y: u32, v: u8) {
        println!("put px {} {} {}", x, y, v);
        self.bmp.set_pixel(x, y, value_to_color(v)).unwrap();
    }

    fn fill(&mut self, x: u32, y: u32, v: u8) {
        self.bmp.fill_region(x, y, value_to_color(v)).unwrap();
    }
}

fn value_to_color(v: u8) -> Rgba {
    Rgba::rgba(255 - v, 255 - v, 255 - v, 255)
}

pub fn create() {
    let mut bmp = BitMap::new(100, 100);
    let mut dr = BitmapDrawer { bmp };

    draw_arc(&mut dr, &Vector2::new(40.0, 40.0), 24.0, 0.4, 2.0 * PI);
    draw_line_with_bold_side(
        &mut dr,
        &Vector2::new(10.0, 10.0),
        &Vector2::new(50.0, 90.0),
        &Vector2::new(40.0, 40.0),
    );
    dr.fill(40, 40, 255);

    // for ((x, y), value) in line_drawing::XiaolinWu::<f32, i8>::new((5.0, 5.0), (10.0, 40.0)) {
    //     // println!("{:?}", (x, y, value))
    //     dr.put_px(x as u32, y as u32, (value * 255.0) as u8);
    // }

    // bmp.fill_region(1, 1, Rgba::rgba(255, 255, 0, 255)).unwrap();
    dr.bmp.save_as("test.bmp").unwrap();
}

/*
нужно что
А то, чтобы был сделан асбтрактный интерфейс для рисования на нём
по-пиксельно. Чтобы можно было вставить в пиксель цвет, и узнать, какой в нём цвет
*/

// type Point = (f64, f64);

fn draw_arc<T: Drawer>(
    drawer: &mut T,
    center: &Vector2,
    radius: f64,
    angle_start: f64,
    angle_step: f64,
) {
    // буду рисовать круг линиями

    // условно - сколько отрезков в единице длины окружности
    const LINE_DENSE: f64 = 0.5;

    // нормлизую исходный угол - чтобы не больше всей окружности
    let angle_step_norm = angle_step.min(2.0 * PI);

    // ищу длину арки
    let arc_len = (2.0 * PI * angle_step_norm).abs();

    // считаю, сколько у меня будет отрезков, примерно
    let lines_count = LINE_DENSE * arc_len;

    // считаю, на какой угол нужно будет двигаться каждый шаг
    let delta_angle = angle_step_norm / lines_count;

    // а теперь нужно, собственно, двигаться
    let mut a = angle_start;
    let end_a = a + angle_step_norm;
    while a <= end_a {
        // рассчитываю точки начала и конца сегмента арки

        let from = Vector2::from_polar(&PolarVec::new(radius, a)) + center;
        let to = Vector2::from_polar(&PolarVec::new(radius, a + delta_angle)) + center;
        // from += center;
        // let from = add_to_point(&polar_to_cartesian(radius, a), &center);
        // let to = add_to_point(&polar_to_cartesian(radius, a + delta_angle), &center);

        // выясняю данные о прямой
        let from_to_line = Line::from_two_points(&from, &to);
        // строю прямую, которая параллельна и проходит через центр
        let center_parallel = Line::from_parallel_line_and_point(&from_to_line, center);

        // рисую
        for ((x, y), value) in line_drawing::XiaolinWu::<f64, i32>::new(from.tuple(), to.tuple()) {
            // ищу расстояние от точки до прямой
            let dist = center_parallel.point_dist(&Vector2::new(x as f64, y as f64));

            // если точка на расстоянии меньшем, чем радиус, то рисую прям чёрной
            // let dist = points_distance(&center, &(x as f64, y as f64));
            println!("dist rad {} {}", dist, radius);
            let value_outer: u8 = if dist < radius {
                255
            } else {
                (value * 255.0) as u8
            };

            drawer.put_px(x as u32, y as u32, value_outer);
        }

        // к следующей
        a += delta_angle;
    }

    // ???
    // PROFIT!
}

fn draw_line_with_bold_side<T: Drawer>(
    dr: &mut T,
    from: &Vector2,
    to: &Vector2,
    side_point: &Vector2,
) {
    let main_line = Line::from_two_points(from, to);
    let parallel = Line::from_parallel_line_and_point(&main_line, side_point);
    let dist_to_side_point = main_line.point_dist(side_point);

    for ((x, y), value) in line_drawing::XiaolinWu::<f64, i32>::new(from.tuple(), to.tuple()) {
        // ищу расстояние от точки до прямой
        let dist = parallel.point_dist(&Vector2::new(x as f64, y as f64));

        // если точка на расстоянии меньшем, чем радиус, то рисую прям чёрной
        // let dist = points_distance(&center, &(x as f64, y as f64));
        // println!("dist rad {} {}", dist, dist_to_side_point);
        let value_outer: u8 = if dist < dist_to_side_point {
            255
        } else {
            (value * 255.0) as u8
        };

        dr.put_px(x as u32, y as u32, value_outer);
    }
}

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

mod vectors {
    use std::ops::{Add, AddAssign};

    /// представление вектора в декартовой системе координат
    pub struct Vector2 {
        pub x: f64,
        pub y: f64,
    }

    impl Vector2 {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn from_polar(PolarVec { a, r }: &PolarVec) -> Self {
            Vector2::new(r * a.cos(), r * a.sin())
        }

        pub fn tuple(&self) -> (f64, f64) {
            (self.x, self.y)
        }
    }

    // impl AddAssign for Vector2 {
    //     fn add_assign(&mut self, other: Self) {
    //         self.x += other.x;
    //         self.y += other.y;
    //     }
    // }

    // impl<'a> AddAssign<&'a Vector2> for Vector2 {
    //     fn add_assign(&mut self, other: &Self) {
    //         self.x += other.x;
    //         self.y += other.y
    //     }
    // }

    impl Add<&'_ Vector2> for Vector2 {
        type Output = Self;

        fn add(self, other: &Vector2) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    /// представление вектора в полярной системе координат
    pub struct PolarVec {
        pub a: f64,
        pub r: f64,
    }

    impl PolarVec {
        pub fn new(radius: f64, angle: f64) -> Self {
            Self {
                a: angle,
                r: radius,
            }
        }
    }
}

mod line {
    use super::vectors::Vector2;

    /// Представление прямой в виде её коэф-ов из общего уравнения прямой
    #[derive(PartialEq, Debug)]
    pub struct Line {
        pub a: f64,
        pub b: f64,
        pub c: f64,
    }

    impl Line {
        pub fn new(a: f64, b: f64, c: f64) -> Self {
            Self { a, b, c }
        }

        /// рассчёт коэф-ов прямой по двум точкам
        pub fn from_two_points(
            Vector2 { y: y1, x: x1 }: &Vector2,
            Vector2 { x: x2, y: y2 }: &Vector2,
        ) -> Self {
            let a = y2 - y1;
            let b = x1 - x2;
            let c = y1 * (x1 + x2) - x1 * (y1 + y2);
            Self { a, b, c }
        }

        /// рассчёт коэф-тов, используя точку, лежающую на прямой, и другой прямой,
        /// которая параллельна нужной
        pub fn from_parallel_line_and_point(l: &Line, p: &Vector2) -> Self {
            let Line { a, b, .. } = l;
            let c = -a * p.x - b * p.y;
            Self { a: *a, b: *b, c }
        }

        /// расстояние от прямой до некой точки, координаты которой заданы вектором
        pub fn point_dist(&self, point: &Vector2) -> f64 {
            (self.a * point.x + self.b * point.y + self.c).abs()
                / (self.a.powi(2) + self.b.powi(2)).sqrt()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn line_from_2_points() {
            let m1 = Vector2::new(-4.0, 3.0);
            let m2 = Vector2::new(2.0, 7.0);

            assert_eq!(Line::new(4.0, -6.0, 34.0), Line::from_two_points(&m1, &m2));
        }

        // #[test]
        // fn line_from_parallel_and_point() {
        //     unimplemented!();
        // }
    }
}
