import { For } from 'solid-js';
import { frameText, type Locale } from '../core/locale';
import { LOCALE_GROUPS } from '../locales';

interface LanguagePickerProps {
  open: boolean;
  current: string;
  onSelect: (locale: Locale) => void;
  onClose: () => void;
}

// A quarter past three shows the differences best: "viertel vier", "kvart över tre", "三時十五分".
const SAMPLE = new Date(2026, 0, 1, 15, 15);

/** Every language with a sample sentence, grouped like the theme picker and drawn in the current theme. */
export function LanguagePicker(props: LanguagePickerProps) {
  return (
    <div class="picker" classList={{ open: props.open }} onClick={() => props.onClose()} aria-hidden={!props.open}>
      <div class="picker-inner" onClick={(event) => event.stopPropagation()}>
        <header class="picker-head">
          <h1>Sprachen</h1>
          <button type="button" class="picker-close" aria-label="Schließen" onClick={() => props.onClose()}>
            ×
          </button>
        </header>
        <For each={LOCALE_GROUPS}>
          {(group) => (
            <section class="picker-group">
              <h2>
                {group.name} <span>{group.note}</span>
              </h2>
              <div class="picker-grid lang-grid">
                <For each={group.locales}>
                  {(locale) => (
                    <button
                      type="button"
                      class="swatch lang"
                      classList={{ active: props.current === locale.id }}
                      onClick={() => props.onSelect(locale)}
                    >
                      <span class="swatch-face lang-face" lang={locale.lang}>
                        {frameText(locale.grid, locale.frame(SAMPLE, { intro: true }))}
                      </span>
                      <span class="swatch-name">{locale.name}</span>
                    </button>
                  )}
                </For>
              </div>
            </section>
          )}
        </For>
      </div>
    </div>
  );
}
