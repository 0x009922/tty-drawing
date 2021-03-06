use crate::vg::Canvas;

pub struct Buffer2D<T: Sized + Copy> {
    pub width: usize,
    pub height: usize,
    pub buff: Vec<T>,
    init_value: T,
}

pub struct Buffer2DIterator<'a, T: Sized + Copy> {
    buff: &'a Buffer2D<T>,
    current_offset: usize,
}

impl<'a, T: Sized + Copy> Iterator for Buffer2DIterator<'a, T> {
    type Item = (T, (usize, usize));

    fn next(&mut self) -> Option<(T, (usize, usize))> {
        match self.buff.offset_to_coords(self.current_offset) {
            Some(coords) => {
                let value = self.buff.buff[self.current_offset];
                self.current_offset += 1;
                Some((value, coords))
            }
            None => None,
        }
    }
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

    fn offset_to_coords(&self, offset: usize) -> Option<(usize, usize)> {
        if offset < self.buff.len() {
            let x: usize = offset % self.width;
            let y: usize = offset / self.width;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn get_iter<'a>(&'a self) -> Buffer2DIterator<'a, T> {
        Buffer2DIterator {
            buff: &self,
            current_offset: 0,
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
        for x in self.buff.iter_mut() {
            *x = val
        }
        // self.buff.iter_mut().map(|x| *x = val);
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

// impl<T: Sized + Copy> Iterator for Buffer2D<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {

//     }
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_offset_to_coords_fine_in_square() {
        let buff = Buffer2D::new(5, 5, 0);
        assert_eq!(buff.offset_to_coords(7), Some((2, 1)));
    }

    #[test]
    fn buffer_offset_to_coords_fine_in_not_square() {
        let buff = Buffer2D::new(5, 4, 0);
        assert_eq!(buff.offset_to_coords(15), Some((0, 3)));
    }

    #[test]
    fn buffer_offset_to_coords_edge() {
        let buff = Buffer2D::new(3, 2, 0);
        assert_eq!(buff.offset_to_coords(2), Some((2, 0)));
    }

    #[test]
    fn buffer_iterator_works_correctly() {
        // создаю небольшой буффер и наполняю
        let mut buff = Buffer2D::new(3, 2, 0);
        buff.set_by_coords((1, 0), 4);
        buff.set_by_coords((1, 1), 3);
        buff.set_by_coords((0, 1), 9);

        // проверяю, что отдаёт итератор
        let mut iterator = buff.get_iter();
        assert_eq!(iterator.next(), Some((0, (0, 0))));
        assert_eq!(iterator.next(), Some((4, (1, 0))));
        assert_eq!(iterator.next(), Some((0, (2, 0))));
        assert_eq!(iterator.next(), Some((9, (0, 1))));
        assert_eq!(iterator.next(), Some((3, (1, 1))));
        assert_eq!(iterator.next(), Some((0, (2, 1))));
        assert_eq!(iterator.next(), None);
    }
}
