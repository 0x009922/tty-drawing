import { Image, Vector } from './printer';

export function useTextImage(pos: Vector, text: string): Image {
  return {
    position: pos,
    size: {
      x: text.length,
      y: 1,
    },
    lines: [text.split('')],
  };
}
