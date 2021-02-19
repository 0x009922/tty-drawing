use std::ops::Add;

/// представление вектора в декартовой системе координат
#[derive(Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_polar_coords(a: f64, r: f64) -> Self {
        Self::from_polar(&PolarVec { a, r })
    }

    pub fn from_polar(PolarVec { a, r }: &PolarVec) -> Self {
        Vector2::new(r * a.cos(), r * a.sin())
    }

    pub fn tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// двигает вектор на середину между ним и другим. среднее арифметическое
    pub fn move_to_middle_with(&mut self, other: &Vector2) -> &mut Self {
        self.x = (self.x + other.x) / 2.0;
        self.y = (self.y + other.y) / 2.0;
        self
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
