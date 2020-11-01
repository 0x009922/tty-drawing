import { ref, computed, reactive, Ref } from '@vue/reactivity';
import chalk from 'chalk';
import { delay } from './tools';
import { Frame, Image, ImageLine, Resolution, Vector, Window } from './types';

export function useText(pos: Vector, text: string, done: () => void): Image {
  const states = reactive<number[]>(new Array(text.length).fill(0));

  async function appear() {
    while (true) {
      let i = -1;
      while (i < 0 || states[i] >= 2) {
        i = ~~(Math.random() * states.length);
      }
      states[i] += 1;
      if (states.every((x) => x >= 2)) {
        break;
      }
      await delay(50);
    }
    await delay(1000);
    disappear();
  }

  async function disappear() {
    for (let i = text.length; i >= 0; i--) {
      const next = i - 1;

      if (i < text.length) {
        states[i] = 4;
      }
      if (next >= 0) {
        states[next] = 3;
      }
      await delay(50);
    }
    done();
  }

  appear();

  const textSplitted = text.split('');
  const textAppeared = computed<string[]>(() => {
    return textSplitted.map((char, i) => {
      switch (states[i]) {
        case 0:
          return null;
        case 1:
          return char === ' ' ? null : chalk.bold.green('.');
        case 2:
          return chalk.magenta(char);
        case 3:
          return chalk.bold.red('_');
        default:
          return null;
      }
    });
  });

  const lines = computed<ImageLine[]>(() => [textAppeared.value]);

  return reactive({
    ...pos,
    lines,
  });
}

/**
 * Создаёт изображение-рамку вокруг содержимого
 * @param target Может быть reactive
 */
export function createFrame(target: Frame, mode: 'table' | 'debug' = 'table'): Image {
  const x = computed(() => target.x - 1);
  const y = computed(() => target.y - 1);
  const lines = computed<ImageLine[]>(() => (mode === 'table' ? tableFrame : debugFrame)(target.w, target.h));

  return reactive({ x, y, lines });
}

function tableFrame(contentWidth: number, contentHeight: number): ImageLine[] {
  const topbottom = new Array(contentWidth).fill('─');
  const empty = new Array(contentWidth).fill(null);
  const value: ImageLine[] = [];
  value.push(['╭', ...topbottom, '╮']);
  for (let i = 0; i < contentHeight; i++) {
    value.push(['│', ...empty, '│']);
  }
  value.push(['╰', ...topbottom, '╯']);
  return value;
}

function debugFrame(contentWidth: number, contentHeight: number): ImageLine[] {
  const indexToNum = (i: number) => `${i % 10}`;
  const topbottom = Array.from(new Array(contentWidth), (v, i) => indexToNum(i));
  const empty = new Array(contentWidth).fill(null);
  const lines: ImageLine[] = [];
  const pushTopBottom = () => lines.push([' ', ...topbottom, ' ']);
  pushTopBottom();
  for (let i = 0; i < contentHeight; i++) {
    const num = indexToNum(i);
    lines.push([num, ...empty, num]);
  }
  pushTopBottom();
  return lines;
}

export function useCenteredWindow(r: Resolution): Window {
  const { round } = Math;

  const w = computed(() => round(r.columns * 0.6));
  const h = computed(() => round(r.rows * 0.5));
  const y = computed(() => round(r.rows / 2 - h.value / 2 - 1));
  const x = computed(() => round(r.columns / 2 - w.value / 2 - 1));

  return reactive({
    x,
    y,
    w,
    h,
    composition: new Set(),
  });
}

export function useSnowflakes(f: Frame): Image[] {
  const SNOWFLAKE_LINES: ImageLine[] = [[chalk.bold.cyan('*')]];
  const SPEED = 0.3;
  const COUNT = 20;

  const height = computed(() => f.h);
  const width = computed(() => f.w);

  async function move(y: Ref<number>) {
    const dt = 0.1;
    const dtMillis = dt * 1000;
    while (true) {
      y.value += dt * SPEED;
      y.value %= 1;
      await delay(dtMillis);
    }
  }

  const flakes: Image[] = Array.from(new Array(COUNT), () => {
    const relX = ref(Math.random());
    const relY = ref(-Math.random());
    move(relY);
    const x = computed(() => ~~(relX.value * width.value));
    const y = computed(() => ~~(relY.value * height.value));
    return reactive({ x, y, lines: SNOWFLAKE_LINES });
  });

  return flakes;
}
