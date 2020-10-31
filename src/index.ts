import { computed, effect, reactive, readonly, Ref, ref, UnwrapRef } from '@vue/reactivity';
import { usePrinter, ImageLine, Image } from './printer';
import replsGenerator from './repls-generator';
import { useTextImage } from './lines';

interface BoardString {
  str: Ref<string>;
  row: number;
  col: number;
}

// function useString(val: UnwrapRef<BoardString>): BoardString {
//   const visible = ref(0);
//   const str = computed(() => val.str.slice(0, visible.value).padEnd(val.str.length, ' '));

//   const interval = setInterval(() => {
//     visible.value += 1;
//     if (visible.value === val.str.length) {
//       clearInterval(interval);
//     }
//   }, 100);

//   return {
//     ...val,
//     str,
//   };
// }

// function useBoard() {
//   let key = 0;
//   const strings = reactive<Map<number, UnwrapRef<BoardString>>>(new Map());
//   const size = reactive({
//     rows: 0,
//     cols: 0,
//   });

//   const stringsList = computed(() => [...strings.values()]);

//   const imageLines = computed<ImageLine[]>(() => {
//     // Создаю пустые массивы
//     const rows = Array.from(new Array(size.rows), () => Array.from(new Array(size.cols), () => ' '));

//     // Заполняю строками
//     stringsList.value.forEach((bs) => {
//       const len = bs.str.length;
//       for (let i = 0; i < len; i++) {
//         const column = bs.col + i;
//         rows[bs.row][column] = bs.str[i];
//       }
//     });

//     // Соединяю
//     return rows;
//   });

//   function addString(s: BoardString): number {
//     strings.set(++key, s);
//     return key;
//   }

//   function removeString(stringKey: number): void {
//     strings.delete(stringKey);
//   }

//   return {
//     size,
//     addString,
//     removeString,
//     imageLines,
//   };
// }

function useWordsOnScreen() {
  const repls = replsGenerator();
  const image = ref<Image | null>(null);
  const { viewport } = usePrinter({ image });

  // const PRESET = [
  //   'Nya',
  //   'N-n-n-n-ya-a-a-a',
  //   'Mur',
  //   'Murrr',
  //   'Murrr-r-r-r-r',
  //   'Meow',
  //   'Mur-r-r meow!',
  //   // 'Who are you?',
  //   // 'Where am I?!',
  //   // 'Who am I?..',
  //   // 'Oh',
  //   // 'O-o-o-o-oh...',
  //   // 'Chaos',
  //   // 'cHaOs',
  //   // 'ha-ha-ha-haaaa...',
  //   // 'Pathetic',
  //   // 'Horror',
  //   // 'Uff',
  //   // 'Pain',
  //   // 'Suffering',
  // ];

  // const viewport: Viewport = {
  //   rows: 10,
  //   cols: 30,
  // };

  // const board = useBoard();
  // const image = computed<Image>(() => ({
  //   cols: viewport.cols,
  //   rows: viewport.rows,
  //   col: 15,
  //   row: 5,
  //   lines: board.imageLines.value,
  // }));
  // usePrinter({ image });

  // board.size.rows = viewport.rows;
  // board.size.cols = viewport.cols;
  // setImage()

  // effect(() => {
  //   print(board.screen.value);
  // });

  function pushString() {
    // Выбираю рандомную строчку
    const repl = repls.next().value;
    // const phrase = PRESET[~~(Math.random() * PRESET.length)];
    // Выбираю позицию

    const row = ~~(Math.random() * viewport.x);
    const col = ~~(Math.random() * (viewport.y - repl.length));
    // Создаю умную строку
    const boardString = useString({
      str: repl,
      col,
      row,
    });
    // Добавляю
    const strKey = board.addString(boardString);
    // Удаляю через некоторое время
    setTimeout(() => {
      board.removeString(strKey);
    }, 5e3);
  }

  // pushString();

  setInterval(pushString, 1500);
}

useWordsOnScreen();
