/* eslint-disable max-params */
import { Composition, Resolution, Frame, Image, Window } from './types';

const DEFAULT_BACKFACE = ' ';

type Buffer = string[][];

function isImage(v: Window | Image): v is Image {
  return !!(v as Image).lines;
}

function framesIntersection(parent: Frame, child: Frame): Frame | null {
  // Нужно вернуть пересечение фреймов, квадратов
  // Для этого я вычисляю абсолютные точки для обоих фреймов,
  // нахожу пересечения отрезков по OX и OY, и, если оба не нулевые,
  // складываю из них фрейм

  const XY: [number, number][] = [];
  const PRESET: (keyof Frame)[][] = [
    ['x', 'w'],
    ['y', 'h'],
  ];

  for (const [keyCoord, keyLen] of PRESET) {
    const p1 = parent[keyCoord];
    const p2 = p1 + parent[keyLen];
    const c1 = p1 + child[keyCoord];
    const c2 = c1 + child[keyLen];

    const inter = sectionsIntersection([p1, p2], [c1, c2]);
    if (!inter) {
      return null;
    }
    XY.push(inter);
  }

  const [[x, w], [y, h]] = XY.map(absCoordsToOriginLen);
  return { x, w, y, h };
}

type OriginLength = [number, number];
type AbsCoords = [number, number];

function absCoordsToOriginLen([pos1, pos2]: AbsCoords): OriginLength {
  return [pos1, pos2 - pos1];
}

function sectionsIntersection([a1, a2]: AbsCoords, [b1, b2]: AbsCoords): AbsCoords | null {
  if (a2 < b1 || b2 < a1) return null;
  return [Math.max(a1, b1), Math.min(a2, b2)];
}

function initBuffer(r: Resolution, backface: string = DEFAULT_BACKFACE): Buffer {
  return Array.from(new Array(r.rows), () => new Array(r.columns).fill(backface));
}

function buildBuffer(b: Buffer): string {
  return b.map((row) => row.join('')).join('\n');
}

function writeToBuffer(b: Buffer, x: number, y: number, value: string): void {
  b[y][x] = value;
}

function fillImage(img: Image, buffer: Buffer, frame: Frame): void {
  let i = 0;
  let y = img.y;
  if (y < 0) {
    i -= y;
    y = 0;
  }
  for (; i < img.lines.length && y < frame.h; i++, y++) {
    const line = img.lines[i];
    let j = 0;
    let x = img.x;
    if (x < 0) {
      j -= x;
      x = 0;
    }
    for (; j < line.length && x < frame.w; j++, x++) {
      const val = line[j];
      if (val) {
        // Считаю абсолютные координаты
        const absX = x + frame.x;
        const absY = y + frame.y;
        writeToBuffer(buffer, absX, absY, val);
      }
    }
  }
}

function buildRecursive(comp: Composition, buffer: Buffer, frame: Frame): void {
  for (const item of comp) {
    if (isImage(item)) {
      // Просто заполняю картинку внутри фрейма. Чётко в рамках
      fillImage(item, buffer, frame);
    } else {
      // Опа, у нас тут окно
      // Значит, создаю новый фрейм и в нём
      // печатаю композицию
      const newFrame = framesIntersection(frame, item);
      if (newFrame) {
        buildRecursive(item.composition, buffer, newFrame);
      }
    }
  }
}

export interface BuildCompositionOpts {
  composition: Composition;
  resolution: Resolution;
  backface?: string;
}

export function buildComposition(opts: BuildCompositionOpts): string {
  const buffer = initBuffer(opts.resolution, opts.backface);
  const frame: Frame = {
    x: 0,
    y: 0,
    w: opts.resolution.columns,
    h: opts.resolution.rows,
  };
  buildRecursive(opts.composition, buffer, frame);
  return buildBuffer(buffer);
}
