// pub mod buffer;
pub mod line;
pub mod vectors;

use line::Line;
use std::f64::consts::PI;
use vectors::{PolarVec, Vector2};

// Тут что-то вроде векторной графики (Vector Graphics)

/// Типаж элементарного черно-белого холста с глубиной пикселя
/// в 256
pub trait Canvas {
    /// Вставка пикселя в канвас. Значение - от 0 до 255
    fn put_px(&mut self, x: i32, y: i32, v: u8);

    /// Получение пикселя из канвы. Если находится за пределами, то None
    fn get_px(&self, x: i32, y: i32) -> Option<u8>;
}

pub enum BoldMode {
    Inner,
    Outer,
}

/// Рисование сектора окружности, внутренняя сторона которой
/// не сглажена
pub fn draw_arc_with_bold_size<T: Canvas>(
    drawer: &mut T,
    center: &Vector2,
    radius: f64,
    angle_start: f64,
    angle_step: f64,
    mode: BoldMode,
) {
    // буду рисовать круг линиями

    // // подготовлюсь к выбору жирности линии
    // let filter_by_mode: fn(f64, f64) -> bool = match mode {
    //     ArcBoldMode::Inner => |dist, rad| dist < rad,
    //     ArcBoldMode::Outer => |dist, rad| dist > rad
    // };

    // условно - сколько отрезков в единице длины окружности
    const LINE_DENSE: f64 = 1.0;

    // нормлизую исходный угол - чтобы не больше всей окружности
    let angle_step_norm = angle_step.min(2.0 * PI);

    // ищу длину арки
    let arc_len = (2.0 * PI * angle_step_norm * radius).abs();

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
        // строю прямую, которая параллельна и проходит через точку, к которой надо быть жирным
        let parallel = Line::from_parallel_line_and_point(&from_to_line, center);

        // рисую
        for ((x, y), value) in line_drawing::XiaolinWu::<f64, i32>::new(from.tuple(), to.tuple()) {
            // ищу расстояние от точки до прямой
            let dist = parallel.point_dist(&Vector2::new(x as f64, y as f64));

            let should_be_bold = match mode {
                BoldMode::Inner => dist < radius,
                BoldMode::Outer => dist > radius,
            };

            let value_fixed = if should_be_bold {
                255
            } else {
                (value * 255.0) as u8
            };

            // теперь выясняю, как рисую. Жирной, или как сказал Ву?
            // Жирной, если mode Outer и расстояние больше радиуса
            // или если mode Inner и расстояние меньше радиуса
            // match mode {
            //     ArcBoldMode::Inner i
            // }

            // let value_fixed = if (mode == ArcBoldMode::Inner && dist < radius) || (mode == ArcBoldMode::Outer && dist > radius) {
            //     255
            // } else {
            //     (value * 255.0) as u8
            // };

            // // если точка на расстоянии меньшем, чем радиус, то рисую прям чёрной
            // // let dist = points_distance(&center, &(x as f64, y as f64));
            // // println!("dist rad {} {}", dist, radius);
            // let value_outer: u8 = if dist < radius {
            //     255
            // } else {
            //     (value * 255.0) as u8
            // };
            // println!("putting {} {} {} {}", x, y, x as u32, y as u32);

            drawer.put_px(x, y, value_fixed);
        }

        // к следующей
        a += delta_angle;
    }

    // ???
    // PROFIT!
}

/// Рисование сглаженной линии, у которой одна сторона не сглажена.
/// Нужно, чтобы заливка работала хорошо
pub fn draw_line_with_bold_side<T: Canvas>(
    dr: &mut T,
    from: &Vector2,
    to: &Vector2,
    side_point: &Vector2,
    mode: BoldMode,
) {
    let main_line = Line::from_two_points(from, to);
    let parallel = Line::from_parallel_line_and_point(&main_line, side_point);
    let dist_to_side_point = main_line.point_dist(side_point);

    for ((x, y), value) in line_drawing::XiaolinWu::<f64, i32>::new(from.tuple(), to.tuple()) {
        // ищу расстояние от точки до прямой
        let dist = parallel.point_dist(&Vector2::new(x as f64, y as f64));

        let should_be_bold = match mode {
            BoldMode::Inner => dist < dist_to_side_point,
            BoldMode::Outer => dist > dist_to_side_point,
        };

        // если точка на расстоянии меньшем, чем радиус, то рисую прям чёрной
        // let dist = points_distance(&center, &(x as f64, y as f64));
        // println!("dist rad {} {}", dist, dist_to_side_point);
        let value_outer: u8 = if should_be_bold {
            255
        } else {
            (value * 255.0) as u8
        };

        dr.put_px(x, y, value_outer);
    }
}

/// заливка канваса, начиная с точки
pub fn fill_canvas<T>(canv: &mut T, (x, y): (i32, i32), new_color: u8)
where
    T: Canvas,
{
    if let Some(old_color) = canv.get_px(x, y) {
        // TODO не создавать постоянно новый вектор. Сделать alloc только один раз!
        let mut stack: Vec<(i32, i32)> = vec![(x, y)];
        let mut span_left = false;
        let mut span_right = false;

        while stack.len() > 0 {
            // Удаляю из стека верхушку
            let point = stack.pop().unwrap();

            // println!("Point... {:?} ", point);

            // Находим границу сверху
            let (_, mut y1) = point;
            loop {
                // println!("Loop inner 1 {:?} {}", point, y1);
                match canv.get_px(point.0, y1 - 1) {
                    Some(px_col) if px_col == old_color => {
                        y1 -= 1;
                    }
                    _ => break,
                }
            }

            span_left = false;
            span_right = false;
            // Топаем по строке сверху вниз
            loop {
                // println!("Loop inner {:?} {}", point, y1);

                match canv.get_px(point.0, y1) {
                    Some(px) if px == old_color => {
                        // закрашиваю
                        // println!("Put px");
                        canv.put_px(point.0, y1, new_color);

                        // беру цвет точки слева
                        let left_color = canv.get_px(point.0 - 1, y1);
                        if !span_left && left_color == Some(old_color) {
                            // нашёл новую точку - слева
                            stack.push((point.0 - 1, y1));
                            span_left = true;
                        } else if span_left && left_color != Some(old_color) {
                            span_left = false;
                        }

                        // беру цвет точки справа
                        let right_color = canv.get_px(point.0 + 1, y1);
                        if !span_right && right_color == Some(old_color) {
                            // нашёл новую точку - слева
                            stack.push((point.0 + 1, y1));
                            span_right = true;
                        } else if span_right && right_color != Some(old_color) {
                            span_right = false;
                        }

                        // окей, иду дальше вниз
                        y1 += 1;
                    }
                    _ => break,
                }
            }
        }
    }
}
