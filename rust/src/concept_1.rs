/**
 * В этом концепте создаётся структура с вложенными элементами. Все они
 * реализуют типаж Tickable - тикающие. В цикле раз за разом мутируется вся структура
 * через tick, и после этого отрисовывается через заимствование.
 */
fn main() {
  let mut root = Root::new();

  for _ in 0..10 {
    root.tick();
    draw(&root);
  }
}

struct Root {
  items: [Sub; 3] 
}

struct Sub {
  counter: u8
}

trait Tickable {
  fn tick(&mut self);
}

impl Root {
  fn new() -> Self {
    Self {
      items: [
        Sub::new(0),
        Sub::new(1),
        Sub::new(2)
      ]
    }
  }
}

impl Tickable for Root {
  fn tick(&mut self) {
    for item in self.items.iter_mut() {
      item.tick();
    }
  }
}

impl Sub {
  fn new(init: u8) -> Self {
    Self {
      counter: init
    }
  }
  
  fn get_char(&self) -> char {
    match self.counter {
      0 => '0',
      1 => '1',
      2 => '2',
      3 => '3',
      _ => panic!("Unknown counter value: {}", self.counter)
    }
  }
}

impl Tickable for Sub {
  fn tick(&mut self) {
    self.counter += 1;
    self.counter %= 4;
  }
}

fn draw(root: &Root) {
  for i in root.items.iter() {
    print!("{} ", i.get_char());
  }
  println!("");
}
