export interface Vector {
  x: number;
  y: number;
}

export interface ZIndexed {
  z?: number;
}

export interface Frame extends Vector {
  w: number;
  h: number;
}

export interface Window extends Frame, ZIndexed {
  composition: Composition;
}

export interface Image extends Vector, ZIndexed {
  lines: ImageLine[];
}

export type Composition = Set<Window | Image>;

export type ImageLine = (string | null)[];

export interface Resolution {
  rows: number;
  columns: number;
}
