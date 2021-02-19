use std::f64::consts::PI;

use crate::vg::{
    draw_arc_with_bold_size, draw_line_with_bold_side, fill_canvas,
    vectors::{PolarVec, Vector2},
    ArcBoldMode, Canvas,
};

/// Радиационный знак!
pub struct RadSign {
    angle: f64,
    center: Vector2,
    radius: f64,
}

/// С какого предела начинается малый радиус (относительно)
const RAD_START_REL: f64 = 0.2;

/// какая угловая длина у одного крыла
const PART_ANGLE: f64 = 2.0 * PI / 3.0 * 0.5;

impl RadSign {
    pub fn new(start_angle: f64, center: Vector2, radius: f64) -> Self {
        Self {
            angle: start_angle,
            center,
            radius,
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

    // let wing_center: Vector2 =

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
    // WIP
}

// trait Yahoo {
//     fn yah(&self);
// }

// fn use_yahoo<T: Yahoo>(y: &T) {
//     y.yah();

//     use_yahoo(y);
// }
