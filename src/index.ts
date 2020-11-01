import { reactive } from '@vue/reactivity';
import usePrinter from './printer';
import { Composition, Vector } from './types';
import replsGenerator from './repls-generator';
import { useText, createFrame, useCenteredWindow, useSnowflakes } from './img';

const composition = reactive<Composition>(new Set());
const { resolution } = usePrinter({ composition });
const mainWindow = useCenteredWindow(resolution);
const flakes = useSnowflakes(mainWindow);
const repls = replsGenerator();

composition.add(mainWindow);
composition.add(createFrame(mainWindow));
flakes.forEach((x) => mainWindow.composition.add(x));

function addText() {
  const replica = repls.next().value;
  const position: Vector = {
    x: ~~(Math.random() * (mainWindow.w - replica.length)),
    y: ~~(Math.random() * mainWindow.h),
  };
  const text = useText(position, replica, () => mainWindow.composition.delete(text));
  mainWindow.composition.add(text);
}

addText();
setInterval(addText, 250);
