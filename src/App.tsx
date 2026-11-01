import { createEffect, createMemo, createSignal, onCleanup, onMount, Show } from 'solid-js';
import { createStore } from 'solid-js/store';
import { Clock } from './components/Clock';
import { Help } from './components/Help';
import { LanguagePicker } from './components/LanguagePicker';
import { ThemePicker } from './components/ThemePicker';
import { Spooky } from './components/Spooky';
import { frameText, sameFrame, type Frame } from './core/locale';
import { formatDate } from './core/date';
import { festival } from './core/festival';
import { secondsFrame } from './core/seconds';
import { createNow } from './now';
import { LOCALES, localeById } from './locales';
import { cycle, loadSettings, MODES, saveSettings, THEMES } from './settings';

const IDLE_MS = 2500;
const EMPTY: Frame = { words: [], dots: 0 };
// The macOS screensaver host shows the page while WebKit treats it as hidden, which freezes CSS
// animations mid-way (blurred, half-faded clock). `?still` renders the end state instead.
const STILL = new URLSearchParams(location.search).has('still');

export function App() {
  const [settings, setSettings] = createStore(loadSettings());
  const [ready, setReady] = createSignal(false);
  const [helpOpen, setHelpOpen] = createSignal(false);
  const [pickerOpen, setPickerOpen] = createSignal(false);
  const [langOpen, setLangOpen] = createSignal(false);
  const [idle, setIdle] = createSignal(false);
  const [toast, setToast] = createSignal('');
  const [showSeconds, setShowSeconds] = createSignal(false);

  const locale = createMemo(() => localeById(settings.locale));
  const now = createNow(() => settings.mode);

  const frame = createMemo<Frame>(
    () => {
      if (!ready()) return EMPTY;
      if (showSeconds()) {
        const seconds = secondsFrame(locale().grid, now().getSeconds());
        // Like the QLOCKTWO, the corner dots keep counting minutes.
        if (seconds) return { ...seconds, dots: locale().frame(now(), { intro: settings.intro }).dots };
      }
      if (settings.mode === 'all') return { words: locale().words, dots: 4 };
      return locale().frame(now(), { intro: settings.intro });
    },
    EMPTY,
    { equals: sameFrame },
  );
  const text = createMemo(() => frameText(locale().grid, frame()));
  const caption = createMemo(() => (settings.date ? formatDate(now(), locale().lang) : undefined));

  createEffect(() => saveSettings({ ...settings }));
  // With "Feste automatisch" the festive theme of the day wins over the chosen one.
  const theme = createMemo(() => (settings.festive && festival(now())) || settings.theme);
  createEffect(() => (document.documentElement.dataset.theme = theme()));
  createEffect(() => {
    if (ready()) document.title = text() || 'QLOCK';
  });

  let toastTimer: ReturnType<typeof setTimeout> | undefined;
  const notify = (message: string) => {
    setToast(message);
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => setToast(''), 1400);
  };

  const toggleFullscreen = () => {
    if (document.fullscreenElement) void document.exitFullscreen();
    else void document.documentElement.requestFullscreen?.();
  };

  const toggleSeconds = () => {
    if (!showSeconds() && !secondsFrame(locale().grid, 0)) {
      notify('Sekunden passen nicht in dieses Raster');
      return;
    }
    setShowSeconds((v) => !v);
  };

  // A double click (fullscreen) also fires two clicks; only toggle seconds once it's clearly a single click.
  let clickTimer: ReturnType<typeof setTimeout> | undefined;
  const onClick = (event: MouseEvent) => {
    if (event.detail !== 1 || !(event.target as Element).closest('.clock')) return;
    clickTimer = setTimeout(toggleSeconds, 250);
  };
  const onDblClick = (event: MouseEvent) => {
    if ((event.target as Element).closest('.picker, .help, button')) return;
    clearTimeout(clickTimer);
    toggleFullscreen();
  };

  const stepTheme = (step: number) => {
    setSettings('theme', cycle(THEMES, settings.theme, step));
    notify(`Theme: ${settings.theme}`);
  };

  const stepLocale = (step: number) => {
    const next = cycle(LOCALES, locale(), step);
    setSettings('locale', next.id);
    notify(next.name);
  };

  const onKey = (event: KeyboardEvent) => {
    if (event.metaKey || event.ctrlKey || event.altKey) return;
    switch (event.key.toLowerCase()) {
      case 't':
        setHelpOpen(false);
        setLangOpen(false);
        setPickerOpen((v) => !v);
        break;
      case 'arrowright':
        stepTheme(1);
        break;
      case 'arrowleft':
        stepTheme(-1);
        break;
      case 'l':
        setHelpOpen(false);
        setPickerOpen(false);
        setLangOpen((v) => !v);
        break;
      case 'arrowdown':
        stepLocale(1);
        break;
      case 'arrowup':
        stepLocale(-1);
        break;
      case 'm':
        setSettings('mode', cycle(MODES, settings.mode));
        notify(`Modus: ${settings.mode}`);
        break;
      case 'i':
        setSettings('intro', (v) => !v);
        notify(settings.intro ? 'Intro an' : 'Intro aus');
        break;
      case 'p':
        setSettings('dots', (v) => !v);
        notify(settings.dots ? 'Punkte an' : 'Punkte aus');
        break;
      case 's':
        setSettings('stretch', (v) => !v);
        notify(settings.stretch ? 'Vollflächig' : 'Quadratisch');
        break;
      case 'e':
        setSettings('effect', (v) => !v);
        notify(settings.effect ? 'Buchstaben-Effekt an' : 'Buchstaben-Effekt aus');
        break;
      case 'b':
        setSettings('ambient', (v) => !v);
        notify(settings.ambient ? 'Hintergrund-Effekt an' : 'Hintergrund-Effekt aus');
        break;
      case 'u':
        setSettings('transition', (v) => !v);
        notify(settings.transition ? 'Übergangseffekt an' : 'Übergangseffekt aus');
        break;
      case 'k':
        setSettings('festive', (v) => !v);
        notify(settings.festive ? `Feste automatisch an${festival(now()) ? ` (${festival(now())})` : ''}` : 'Feste automatisch aus');
        break;
      case 'd':
        setSettings('date', (v) => !v);
        notify(settings.date ? 'Datum an' : 'Datum aus');
        break;
      case 'r':
        setSettings('progress', (v) => !v);
        notify(settings.progress ? 'Sekundenrahmen an' : 'Sekundenrahmen aus');
        break;
      case 'f':
        toggleFullscreen();
        break;
      case ' ':
        toggleSeconds();
        break;
      case '?':
      case 'h':
        setHelpOpen((v) => !v);
        break;
      case 'escape':
        setHelpOpen(false);
        setPickerOpen(false);
        setLangOpen(false);
        setShowSeconds(false);
        break;
      default:
        return;
    }
    event.preventDefault();
  };

  let idleTimer: ReturnType<typeof setTimeout> | undefined;
  const wake = () => {
    setIdle(false);
    clearTimeout(idleTimer);
    idleTimer = setTimeout(() => setIdle(true), IDLE_MS);
  };

  onMount(() => {
    // Start dark, then let the words light up one by one.
    const readyTimer = setTimeout(() => setReady(true), 600);
    window.addEventListener('keydown', onKey);
    window.addEventListener('pointermove', wake);
    wake();
    onCleanup(() => {
      clearTimeout(readyTimer);
      clearTimeout(idleTimer);
      clearTimeout(toastTimer);
      clearTimeout(clickTimer);
      window.removeEventListener('keydown', onKey);
      window.removeEventListener('pointermove', wake);
    });
  });

  return (
    <main
      class="stage"
      classList={{
        idle: idle() && !helpOpen() && !pickerOpen() && !langOpen(),
        still: STILL,
        stretch: settings.stretch,
        seconds: showSeconds(),
        fx: settings.effect,
        amb: settings.ambient,
        // The seconds digits change every second; a focus pull each time would be restless.
        tx: settings.transition && !showSeconds(),
      }}
      onClick={onClick}
      onDblClick={onDblClick}
    >
      <Clock
        grid={locale().grid}
        frame={frame()}
        label={text()}
        showDots={settings.dots}
        progress={settings.progress && !showSeconds() ? now().getSeconds() : undefined}
        caption={caption()}
        ambient={settings.ambient}
      />
      <Show when={theme() === 'halloween'}>
        <Spooky />
      </Show>
      <div class="toast" classList={{ visible: toast() !== '' }} role="status">
        {toast()}
      </div>
      <Help open={helpOpen()} onClose={() => setHelpOpen(false)} />
      <LanguagePicker
        open={langOpen()}
        current={settings.locale}
        onSelect={(next) => {
          setSettings('locale', next.id);
          setLangOpen(false);
        }}
        onClose={() => setLangOpen(false)}
      />
      <ThemePicker
        open={pickerOpen()}
        current={settings.theme}
        onSelect={(theme) => {
          setSettings('theme', theme);
          setPickerOpen(false);
        }}
        onClose={() => setPickerOpen(false)}
      />
    </main>
  );
}
