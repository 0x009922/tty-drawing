use super::Canvas;

pub struct CanvasBuffer {
    pub width: usize,
    pub height: usize,
    pub buff: Vec<u8>,
}

impl CanvasBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buff: Vec<u8> = vec![0; width * height];

        Self {
            width,
            height,
            buff,
        }
    }

    /// координаты в индекс вектора
    fn coords_to_offset(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if x < self.width && y < self.height {
                return Some(y * self.width + x);
            }
        }
        None
    }

    // /// отдача буффера. Может понадобиться для изменения размера
    // pub fn get_mut<'a>(&'a mut self) -> &'a mut Vec<u8> {
    //     &mut self.buff
    // }
}

impl Canvas for CanvasBuffer {
    fn put_px(&mut self, x: i32, y: i32, v: u8) {
        if let Some(offset) = self.coords_to_offset(x, y) {
            self.buff[offset] = v;
        }
    }

    fn get_px(&self, x: i32, y: i32) -> Option<u8> {
        if let Some(offset) = self.coords_to_offset(x, y) {
            Some(self.buff[offset])
        } else {
            None
        }
    }
}
