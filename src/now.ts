import { createEffect, createSignal, onCleanup, type Accessor } from 'solid-js';
import type { Mode } from './settings';

const DEMO_INTERVAL_MS = 2500;
const MINUTES_PER_DAY = 1440;

/** Current time, ticking on the full second. In demo mode it jumps randomly through the day. */
export function createNow(mode: Accessor<Mode>): Accessor<Date> {
  const [now, setNow] = createSignal(new Date());

  createEffect(() => {
    if (mode() === 'demo') {
      const id = setInterval(() => {
        const step = 1 + Math.floor(Math.random() * MINUTES_PER_DAY);
        setNow((d) => new Date(d.getTime() + step * 60_000));
      }, DEMO_INTERVAL_MS);
      onCleanup(() => clearInterval(id));
      return;
    }

    let timer: ReturnType<typeof setTimeout>;
    const tick = () => {
      setNow(new Date());
      timer = setTimeout(tick, 1000 - (Date.now() % 1000));
    };
    tick();
    // The macOS screensaver keeps the page hidden, where timers may be throttled; it calls this
    // right before taking each snapshot.
    window.qlockTick = () => setNow(new Date());
    onCleanup(() => {
      clearTimeout(timer);
      delete window.qlockTick;
    });
  });

  return now;
}

declare global {
  interface Window {
    qlockTick?: () => void;
  }
}
