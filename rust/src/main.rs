mod rendering;
use std::thread;
use std::time::Duration;
use rendering::composition::{Image, ImageAtom};

fn main() {
  // упростим задачу - сделаю одну структуру, в которой есть одно обновляющееся изображение
  // буду постоянно тикать и давать ей возможность обновляться, а потом заимствовать её композицию
  // и отдавать на рендер

  let mut renderer = rendering::TerminalRenderer::new();

  let mut img = Img::new(5);
  let interval = Duration::from_millis(100);

  loop {
    img.tick();
    renderer.render(&img.image);
    thread::sleep(interval);
  }
}

struct Img {
  counter: usize,
  image: Image,
  size: usize
}

impl Img {
  fn new(size: usize) -> Img {
    let img_data: Vec<ImageAtom> = (0..size).into_iter().map(|_| ImageAtom(None)).collect();

    Img {
      counter: 0,
      size,
      image: Image {
        x: 5,
        y: 5,
        lines: vec![img_data]
      }
    }
  }

  fn tick(&mut self) {
    self.counter += 1;
    self.counter %= self.size;
    self.update_image();
  }

  fn update_image(&mut self) {
    for i in 0..self.size {
      let val: Option<char> = if self.counter == i { Some('_') } else { None };
      self.image.lines[0][i] = ImageAtom(val);
    }
  }
}
