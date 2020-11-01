import { Frame } from './types';

export const delay = (d: number) => new Promise((r) => setTimeout(r, d));

export function inspectFrame(f: Frame): void {
  console.log(`Frame ${f.x} -> ${f.w} | ${f.y} -> ${f.h}`);
}
