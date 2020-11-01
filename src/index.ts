import { reactive } from '@vue/reactivity';
import chalk from 'chalk';
import usePrinter from './printer';

import { Composition, Image, ImageLine, Vector, Window, Frame } from './types';
import replsGenerator from './repls-generator';
import { useText, createFrame, useCenteredWindow } from './img';

const composition = reactive<Composition>(new Set());
const { resolution } = usePrinter({ composition });
const repls = replsGenerator();

const mainWindow = useCenteredWindow(resolution);
composition.add(mainWindow);
composition.add(createFrame(mainWindow));
// mainWindow.composition.add({
//   x: 0,
//   y: 0,
//   lines: Array.from(new Array(50), () => new Array(50).fill('#')),
// });

function addText() {
  const replica = repls.next().value;
  const position: Vector = {
    x: ~~(Math.random() * (mainWindow.w - replica.length)),
    y: ~~(Math.random() * mainWindow.h),
  };
  const text = useText(position, replica, () => mainWindow.composition.delete(text));
  mainWindow.composition.add(text);
  // setTimeout(() => {
  //   mainWindow.composition.delete(text);
  // }, 1450);
}

addText();
setInterval(addText, 250);

// Теперь надо только передать композицию в принтер
