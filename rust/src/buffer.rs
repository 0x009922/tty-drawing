use crate::vg::Canvas;

pub struct Buffer2D<T: Sized + Copy> {
    pub width: usize,
    pub height: usize,
    pub buff: Vec<T>,
    init_value: T,
}

impl<T: Sized + Copy> Buffer2D<T> {
    pub fn new(width: usize, height: usize, init: T) -> Self {
        let buff: Vec<T> = vec![init; width * height];

        Self {
            width,
            height,
            buff,
            init_value: init,
        }
    }

    /// координаты в индекс вектора
    fn coords_to_offset(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            return self.ucoords_to_offset(x, y);
            // if x < self.width && y < self.height {
            //     return Some(y * self.width + x);
            // }
        }
        None
    }

    fn ucoords_to_offset(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    /// очистка буффера. если передаётся None, будет очищен тем же, чем при инициализации
    pub fn clear(&mut self, val: Option<T>) {
        let val: T = match val {
            Some(x) => x,
            None => self.init_value,
        };
        self.buff.iter_mut().map(|x| *x = val);
    }

    pub fn set_by_coords(&mut self, (x, y): (i32, i32), v: T) {
        if let Some(offset) = self.coords_to_offset(x, y) {
            self.buff[offset] = v;
        }
    }

    pub fn set_by_ucoords(&mut self, (x, y): (usize, usize), v: T) {
        if let Some(offset) = self.ucoords_to_offset(x, y) {
            self.buff[offset] = v;
        }
    }

    pub fn get_by_coords(&self, (x, y): (i32, i32)) -> Option<T> {
        if let Some(offset) = self.coords_to_offset(x, y) {
            Some(self.buff[offset])
        } else {
            None
        }
    }

    // pub fn set_by_
}

impl Canvas for Buffer2D<u8> {
    fn put_px(&mut self, x: i32, y: i32, v: u8) {
        self.set_by_coords((x, y), v);
        // if let Some(offset) = self.coords_to_offset(x, y) {
        //     self.buff[offset] = v;
        // }
    }

    fn get_px(&self, x: i32, y: i32) -> Option<u8> {
        self.get_by_coords((x, y))
        // if let Some(offset) = self.coords_to_offset(x, y) {
        //     Some(self.buff[offset])
        // } else {
        //     None
        // }
    }
}

// pub struct CanvasBuffer {
//     pub width: usize,
//     pub height: usize,
//     pub buff: Vec<u8>,
// }

// impl CanvasBuffer {
//     pub fn new(width: usize, height: usize) -> Self {
//         let buff: Vec<u8> = vec![0; width * height];

//         Self {
//             width,
//             height,
//             buff,
//         }
//     }

//     /// координаты в индекс вектора
//     fn coords_to_offset(&self, x: i32, y: i32) -> Option<usize> {
//         if x >= 0 && y >= 0 {
//             let x = x as usize;
//             let y = y as usize;
//             if x < self.width && y < self.height {
//                 return Some(y * self.width + x);
//             }
//         }
//         None
//     }

//     // /// отдача буффера. Может понадобиться для изменения размера
//     // pub fn get_mut<'a>(&'a mut self) -> &'a mut Vec<u8> {
//     //     &mut self.buff
//     // }
// }

// impl Canvas for CanvasBuffer {
//     fn put_px(&mut self, x: i32, y: i32, v: u8) {
//         if let Some(offset) = self.coords_to_offset(x, y) {
//             self.buff[offset] = v;
//         }
//     }

//     fn get_px(&self, x: i32, y: i32) -> Option<u8> {
//         if let Some(offset) = self.coords_to_offset(x, y) {
//             Some(self.buff[offset])
//         } else {
//             None
//         }
//     }
// }
