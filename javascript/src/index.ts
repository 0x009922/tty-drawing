import { computed, reactive } from '@vue/reactivity';
import { usePrinter, Composition, Window } from './printer';
import { createFrame, useCenteredWindow, useSnowflakes, useInfiniteReplicas } from './img';

const rootComposition = reactive<Composition>(new Set());
const { resolution } = usePrinter({ composition: rootComposition });
const mainWindow = useCenteredWindow(resolution);
useInfiniteReplicas(mainWindow);
const flakesWindow: Window = reactive({
  x: computed(() => mainWindow.x),
  w: computed(() => mainWindow.w),
  y: computed(() => mainWindow.y),
  h: computed(() => mainWindow.h),
  z: 10,
  composition: new Set(),
});
useSnowflakes(flakesWindow).forEach((x) => flakesWindow.composition.add(x));
rootComposition.add(mainWindow);
rootComposition.add(flakesWindow);
rootComposition.add(createFrame(mainWindow));
