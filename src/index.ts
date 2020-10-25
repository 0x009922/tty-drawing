import {
  computed, effect, reactive, readonly, Ref, ref, shallowReactive, UnwrapRef
} from '@vue/reactivity';

interface BoardString {
  str: Ref<string>;
  row: number;
  col: number;
}

function useString(val: UnwrapRef<BoardString>): BoardString {
  const visible = ref(0);
  const str = computed(() => val.str.slice(0, visible.value).padEnd(val.str.length, ' '));

  const interval = setInterval(() => {
    visible.value += 1;
    if (visible.value === val.str.length) {
      clearInterval(interval);
    }
  }, 100);

  return {
    ...val,
    str
  };
}

function useBoard() {
  let key = 0;
  const strings = reactive<Map<number, UnwrapRef<BoardString>>>(new Map());
  const size = reactive({
    rows: 0,
    cols: 0
  });

  const stringsList = computed(() => [...strings.values()]);

  const screen = computed<string[]>(() => {
    // Создаю пустые массивы
    const rows = Array.from(new Array(size.rows), () => (
      Array.from(new Array(size.cols), () => ' ')
    ));

    // Заполняю строками
    stringsList.value.forEach((bs) => {
      const len = bs.str.length;
      for (let i = 0; i < len; i++) {
        const column = bs.col + i;
        rows[bs.row][column] = bs.str[i];
      }
    });

    // Соединяю
    return rows.map((row) => row.join(''));
  });

  function addString(s: BoardString): number {
    strings.set(++key, s);
    return key;
  }

  function removeString(stringKey: number): void {
    strings.delete(stringKey);
  }

  return {
    size,
    addString,
    removeString,
    screen
  };
}

function usePrinter() {
  const out = process.stdout;
  const rows = ref<number>(out.rows);
  const columns = ref<number>(out.columns - 1);

  // Этого не будет на Windows
  out.on('resize', () => {
    rows.value = out.rows;
    columns.value = out.columns - 1;
  });

  function print(data: string[]): void {
    out.cursorTo(0, 0)
    out.write(`${data.join('\n')}`);
    // data.forEach((r) => {
    //   out.write(`${r}\n`);
    // });
  }

  return {
    rows: readonly(rows),
    columns: readonly(columns),
    print
  };
}

function useWordsOnScreen() {
  const PRESET = [
    'Who are you?..',
    'Where am I?!...',
    'Oh',
    'Chaos',
    'Pathetic',
    'Horror'
  ];

  const { rows, columns: cols, print } = usePrinter();
  const board = useBoard();

  board.size.rows = rows.value;
  board.size.cols = cols.value;

  effect(() => {
    print(board.screen.value);
  });

  function pushString() {
    // Выбираю рандомную строчку
    const phrase = PRESET[~~(Math.random() * PRESET.length)];
    // Выбираю позицию
    const row = ~~(Math.random() * rows.value);
    const col = ~~(Math.random() * (cols.value - phrase.length));
    // Создаю умную строку
    const boardString = useString({
      str: phrase,
      col,
      row
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
