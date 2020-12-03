use crate::core;
use crate::rendering::TerminalArtist;
use rand::{prelude::ThreadRng, Rng};
use termion::color;

// скорость перемещения тумана в секунду
const FOG_SPEED: (f32, f32) = (2.0, 10.0);

// длина стрелочки тумана
const FOG_LEN: (f32, f32) = (2.0, 5.0);

pub struct FogLine {
    // позиция горизонтально - плавающая
    x: f32,
    // вертикально - фикс
    y: u8,
    // скорость перемещения в секунду
    speed: f32,
    // длина линии
    length: f32,
    // движется в обратную сторону?
    reverse: bool,
    // крайняя граница справа
    max_x: u8,
}

impl FogLine {
    pub fn new_random(rows: usize, cols: usize, rng: &mut ThreadRng) -> Self {
        // x - где-то в пределах [0; cols]
        let x: f32 = rng.gen_range(0.0, cols as f32);
        // y - где-то в [0; rows]
        let y: u8 = rng.gen_range(0, rows as u8);
        // reverse - 50/50
        // let reverse = rng.gen::<f32>() > 0.5;
        let speed = rng.gen_range(FOG_SPEED.0, FOG_SPEED.1);
        let length = rng.gen_range(FOG_LEN.0, FOG_LEN.1);

        Self {
            x,
            y,
            reverse: false,
            max_x: cols as u8,
            speed,
            length,
        }
    }
}

impl core::Tick for FogLine {
    fn tick(&mut self, ms: u32) {
        // движение линии со своей скоростью в своём направлении

        let delta = (ms as f32) * 0.001 * self.speed;
        if self.reverse {
            self.x -= delta;
            while self.x < 0.0 {
                self.x += self.max_x as f32;
            }
        } else {
            self.x += delta;
            while self.x > self.max_x as f32 {
                self.x -= self.max_x as f32;
            }
        }
    }
}

impl core::Art for FogLine {
    fn draw(&self, artist: &mut TerminalArtist) {
        // рисование линии в зависимости от её положения

        let y = self.y as usize;
        let x_start = self.x.round() as usize;
        let x_end = (self.x + self.length).round() as usize;

        for i in x_start..(x_end + 1) {
            let x = i % self.max_x as usize;
            let s = format!(
                "{}{}{}",
                color::Fg(color::LightBlack),
                '-',
                color::Fg(color::Reset)
            );
            artist.buffer.write_composed(x, y, s)
        }
    }
}
