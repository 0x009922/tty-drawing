export interface Vector {
  x: number;
  y: number;
}

export interface Frame extends Vector {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface Window extends Frame {
  composition: Composition;
}

export interface Image extends Vector {
  lines: ImageLine[];
}

export type Composition = Set<Window | Image>;

export type ImageLine = (string | null)[];

export interface Resolution {
  rows: number;
  columns: number;
}
