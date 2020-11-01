import randomWords from 'random-words';

export default function* replsGenerator(): Generator<string, never, never> {
  while (true) {
    yield randomWords({ exactly: ~~(1 + Math.random() * 3), join: ' ' });
  }
}
