import { computed, effect, reactive, readonly, Ref } from '@vue/reactivity';

const BACKFACE = ' ';

export interface Vector {
  x: number;
  y: number;
}

// export interface Viewport {
//   rows: number;
//   cols: number;
// }

// export interface PositionedViewport extends Viewport {
//   col: number;
//   row: number;
// }

export interface BoundingRect {
  position: Vector;
  size: Vector;
}

export interface Image extends BoundingRect {
  lines: ImageLine[];
}

export type ImageLine = (string | null)[];

function createFrameImage(imgRect: BoundingRect): Image {
  const {
    size: { x: width, y: height },
  } = imgRect;

  const rect: BoundingRect = {
    position: {
      x: imgRect.position.x - 1,
      y: imgRect.position.y - 1,
    },
    size: {
      x: width + 2,
      y: height + 2,
    },
  };

  const topbottom = new Array(width).fill('─');
  const empty = new Array(width).fill(null);
  const lines: ImageLine[] = [];
  lines.push(['╭', ...topbottom, '╮']);
  for (let i = 0; i < height; i++) {
    lines.push(['│', ...empty, '│']);
  }
  lines.push(['╰', ...topbottom, '╯']);

  return { ...rect, lines };
}

function makeComposition(viewport: Vector, images: Image[]): string {
  // Заполняю экран пустотой
  const screen: string[][] = Array.from(new Array(viewport.y), () => new Array(viewport.x).fill(BACKFACE));

  // Накладываю изображения одно за другим
  images.forEach((img) => {
    const imageLinesCount = Math.min(img.lines.length, img.size.y);
    for (let i = 0, y = img.position.y; i < imageLinesCount && y < viewport.y; i++, y++) {
      const line = img.lines[i];
      const lineCharsCount = Math.min(line.length, img.size.x);
      for (let j = 0, x = img.position.x; j < lineCharsCount && x < viewport.x; j++, x++) {
        const char = line[j];
        if (char) {
          // console.log('SET', y, x, char);
          screen[y][x] = char;
        }
      }
    }
  });

  // Собираю в одну строку
  const joined = screen.map((x) => x.join('')).join('');

  return joined;
}

export function usePrinter(props: { image: Ref<Image | null> }) {
  const out = process.stdout;
  let wasFirstPrint = false;

  const terminalViewport: Vector = reactive({
    x: 0,
    y: 0,
  });

  const imageFrame = computed<Image | null>(() => createFrameImage(props.image.value));
  const composition = computed<string>(() => makeComposition(terminalViewport, [imageFrame.value, props.image.value]));

  /**
   * Установка terminalSize в соответствие с текущими
   * настоящими размерами терминала
   */
  function syncTerminalViewport() {
    terminalViewport.y = out.rows;
    terminalViewport.x = out.columns;
  }

  /**
   * Печать в видимой области
   */
  function print(data: string): void {
    if (!wasFirstPrint) {
      wasFirstPrint = true;
      out.write(`${'-'.repeat(terminalViewport.x)}\n`);
    } else {
      out.cursorTo(0, 0);
    }
    out.write(data);
  }

  syncTerminalViewport();

  // Этого не будет на Windows
  out.on('resize', () => syncTerminalViewport);

  // Настраиваю отрисовку
  effect(() => {
    print(composition.value);
  });

  return {
    viewport: readonly(terminalViewport),
  };
}
