import { computed, effect, reactive, readonly, Ref } from "@vue/reactivity";

const BACKFACE = ' ';

interface BoundingRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type ImageLine = (string | null)[]

export interface Image extends BoundingRect {
  lines: ImageLine[]
}

export interface Viewport {
  rows: number;
  cols: number;
}


function createFrameImage(contentRect: BoundingRect): Image {
  const rect: BoundingRect = {
    x: contentRect.x - 1,
    y: contentRect.y - 1,
    width: contentRect.width + 2,
    height: contentRect.height + 2,
  }

  const topbottom = new Array(contentRect.width).fill('─');
  const empty = new Array(contentRect.width).fill(null);
  const lines: ImageLine[] = [];
  lines.push(['╭', ...topbottom, '╮']);
  for (let i = 0; i < contentRect.height; i++) {
    lines.push(['│', ...empty, '│'])
  }
  lines.push(['╰', ...topbottom, '╯']);

  return { ...rect, lines };
}

function makeComposition(viewport: Viewport, images: Image[]): string {
  // Заполняю экран пустотой
  const screen: string[][] = Array.from(new Array(viewport.rows), () => (
    new Array(viewport.cols).fill(BACKFACE)
  ))

  // Накладываю изображения одно за другим
  images.forEach((img) => {
    const imageLinesCount = Math.min(img.lines.length, img.height);
    for (let i = 0, y = img.y; i < imageLinesCount && y < viewport.rows; i++, y++) {
      const line = img.lines[i];
      const lineCharsCount = Math.min(line.length, img.width);
      for (let j = 0, x = img.x; j < lineCharsCount && x < viewport.cols; j++, x++) {
        const char = line[j];
        if (char) {
          // console.log('SET', y, x, char);
          screen[y][x] = char;
        }
      }
    }
  })

  // Собираю в одну строку
  const joined = screen.map((x) => x.join('')).join('');

  return joined;
}


export function usePrinter(props: {
  image: Ref<Image>
}) {
  const out = process.stdout;
  let wasFirstPrint = false;
  
  const terminalViewport: Viewport = reactive({
    rows: 0,
    cols: 0
  });

  const imageFrame = computed<Image>(() => createFrameImage(props.image.value));
  const composition = computed<string>(() => makeComposition(terminalViewport, [imageFrame.value, props.image.value]))

  /**
   * Установка terminalSize в соответствие с текущими
   * настоящими размерами терминала
   */
  function syncTerminalViewport() {
    terminalViewport.rows = out.rows;
    terminalViewport.cols = out.columns;
  }

  /**
   * Печать в видимой области
   */
  function print(data: string): void {
    if (!wasFirstPrint) {
      wasFirstPrint = true;
      out.write(`${'-'.repeat(terminalViewport.cols)}\n`);
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
  })


  return {
    viewport: readonly(terminalViewport)

  };
}