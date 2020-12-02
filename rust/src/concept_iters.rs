/**
 * Цель этого концепта - работа с итераторами.
 * 
 * Нужно создать функцию, которая принимала бы итератор,
 * который бы возвращал enum из двух вариантов. Первый вариант - это,
 * допустим, какой-нибудь символ, обычный char. Второй вариант - это
 * некая функция (замыкание), которая возвращает итератор, идентичный
 * первоначальному по типу. То есть она отдавала бы итератор, который
 * можно было бы заново пустить в изначальную функцию, создавая
 * рекурсию и фрактальное раскрытие дерева элементов.
 * 
 * Для начала можно сделать это с использованием векторов как частного 
 * случая итератора, а затем уже сделать универсально с итератором,
 * чтобы на вход шли и вектора, и массивы, и что угодно.
 * 
 * Реализация позволит понимать, как реализовать гибкий фрактальный рендеринг
 * дерева компонентов.
 */
fn main() {
  let sub2: Vec<Item> = vec![
    Item::Final('3')
  ];

  let sub1: Vec<Item> = vec![
    Item::Final('e'),
    Item::Subtree(&sub2),
    Item::Final('n'),
  ];

  let items: Vec<Item> = vec![
    Item::Final('o'),
    Item::Final('p'),
    Item::Subtree(&sub1),
    Item::Final('m')
  ];

  let expanded = expand_items(&items);
  assert_eq!(expanded, vec!['o', 'p', 'e', '3', 'n', 'm']);
  println!("{:?}", expanded)
}

enum Item {
  Final(char),
  Subtree(Fn() -> &Vec<Item>)
}

trait SubtreeProvider {
  fn items(&self) -> &Vec<Item>;
}

fn expand_items(items: &Vec<Item>) -> Vec<char> {
  let mut result: Vec<char> = vec![];

  for item in items.iter() {
    match item {
      Item::Final(x) => result.push(*x),
      Item::Subtree(sub_items) => {
        let expanded = expand_items(sub_items);
        for sub_item in expanded.iter() {
          result.push(*sub_item)
        }
      }
    }
  }

  result
}
