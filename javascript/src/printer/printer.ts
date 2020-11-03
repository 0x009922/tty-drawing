import { computed, effect, reactive, readonly } from '@vue/reactivity';
import { Composition, Resolution } from './types';
import { buildComposition } from './composition-builder';

export interface PrinterProps {
  composition: Composition;
  backface?: string;
}

export interface Printer {
  resolution: Resolution;
}

function useTerminalResolution(): Resolution {
  const val = reactive<Resolution>({
    rows: 0,
    columns: 0,
  });

  function sync() {
    val.rows = process.stdout.rows;
    val.columns = process.stdout.columns;
  }

  sync();
  process.stdout.on('resize', sync);

  return readonly(val);
}

export default function usePrinter(props: PrinterProps): Printer {
  const { composition } = props;
  const resolution = useTerminalResolution();

  let wasFirstPrint = false;
  function print(compositionData: string): void {
    if (!wasFirstPrint) {
      wasFirstPrint = true;
      process.stdout.write(`${'-'.repeat(resolution.columns)}\n`);
    } else {
      process.stdout.cursorTo(0, 0);
    }
    process.stdout.write(compositionData);
  }

  const buildedComposition = computed<string>(() => (
    buildComposition({
      composition,
      resolution,
      backface: props.backface,
    })
  ))

  effect(() => print(buildedComposition.value));

  return { resolution };
}
