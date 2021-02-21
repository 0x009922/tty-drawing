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
