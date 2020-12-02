mod rendering;

use rendering::*;
use std::time::Duration;
use std::thread;
use rand::{Rng, prelude::ThreadRng};

fn main() {
  let mut rng = rand::thread_rng();
  
  // let resolution = TerminalResolution::new();
  let mut artist = TerminalArtist::new();
  let mut fog_lines = generate_fog_lines(&mut rng, &artist.resolution);

  const MS: u64 = 100;
  let ms_duration = Duration::from_millis(MS);

  loop {
    artist.buffer.clear();
    for line in fog_lines.iter_mut() {
      line.tick(MS as u32);
      line.draw(&mut artist);
    }
    artist.render();

    thread::sleep(ms_duration);
  }
}

fn generate_fog_lines(rng: &mut ThreadRng, res: &TerminalResolution) -> Vec<FogLine> {
  const COUNT: u32 = 50; 
  let (rows, cols) = res.get_rows_cols();
  (0..COUNT).map(|_| FogLine::new_random(rows, cols, rng)).collect()
}

// *** core

trait Tick {
  fn tick(&mut self, ms: u32);
}

trait Art {
  fn draw(&self, artist: &mut TerminalArtist);
}

// *** Линия тумана

// скорость перемещения тумана в секунду
const FOG_SPEED: (f32, f32) = (2.0, 5.0);

// длина стрелочки тумана
const FOG_LEN: (f32, f32) = (2.0, 5.0);

struct FogLine {
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
  max_x: u8
}

impl FogLine {
  fn new_random(rows: usize, cols: usize, rng: &mut ThreadRng) -> Self {
    // x - где-то в пределах [0; cols]
    let x: f32 = rng.gen_range(0.0, cols as f32);
    // y - где-то в [0; rows]
    let y: u8 = rng.gen_range(0, rows as u8);
    // reverse - 50/50
    let reverse = rng.gen::<f32>() > 0.5;
    let speed = rng.gen_range(FOG_SPEED.0, FOG_SPEED.1);
    let length = rng.gen_range(FOG_LEN.0, FOG_LEN.1);

    Self {
      x,
      y,
      reverse,
      max_x: cols as u8,
      speed,
      length
    }
  }
}

impl Tick for FogLine {
  fn tick(&mut self, ms: u32) {
    let mut delta = (ms as f32) * 0.001 * self.speed;
    if self.reverse {
      delta *= -1.0;
    }
    self.x += delta;
    self.x %= self.max_x as f32;
  }
}

impl Art for FogLine {
  fn draw(&self, artist: &mut TerminalArtist) {
    let y = self.y as usize;
    let x_start = self.x.round() as usize;
    let x_end = (self.x + self.length).round() as usize;

    for i in x_start..(x_end + 1) {
      let x = i % artist.buffer.cols();
      artist.buffer.write(x, y, '-')
    }
  }
}
