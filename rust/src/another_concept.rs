/**
 * Цель - реализовать динамический элемент композиции.
 * 
 * Есть, допустим, два типа элементов композиции - точка и компонент.
 * Компонент - это нечто динамическое, реализующее типаж Component,
 * trait object.
 * 
 * И вот я из этого попробую составить дерево элементов со своим тикающим поведением,
 * и реализую фрактальное раскрытие этого всего дела
 */
fn main() {
  let mut root = Root::new();
}

enum Elem {
  Dot(Option<char>),
  Component(Box<dyn Component>)
}

// traits

trait Component {
  fn elements<'a>(&'a self) -> &'a Vec<Elem>;
}

trait Tick {
  fn tick(&mut self);
}

// root

struct Root {
  items: Vec<Elem>,
  // children: Vec<Blink
}

impl Root {
  fn new() -> Self {
    Self {
      items: vec![
        Elem::Dot(Some('S')),
        Elem::Component(Box::new(Blink::new('r'))),
        Elem::Dot(Some('E'))
      ]
    }
  }
}

impl Component for Root {
  fn elements<'a>(&'a self) -> &'a Vec<Elem> {
    &self.items
  }
}

// child

struct Blink {
  val: char,
  elems: Vec<Elem>
}

impl Blink {
  fn new(val: char) -> Self {
    Self {
      val,
      elems: vec![Elem::Dot(None)]
    }
  }
}

impl Component for Blink {
  fn elements<'a>(&'a self) -> &'a Vec<Elem> {
    &self.elems
  }
}

impl Tick for Blink {
  fn tick(&mut self) {
    self.elems[0] = match self.elems[0] {
      Elem::Dot(Some(_)) => Elem::Dot(None),
      Elem::Dot(None) => Elem::Dot(Some(self.val)),
      _ => panic!("Undefined self[0] value!")
    }
  }
}
