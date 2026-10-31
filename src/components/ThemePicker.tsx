import { For } from 'solid-js';
import { THEME_GROUPS, themeLabel, type Theme } from '../settings';

interface ThemePickerProps {
  open: boolean;
  current: Theme;
  onSelect: (theme: Theme) => void;
  onClose: () => void;
}

/**
 * Every theme as a small preview, grouped by owner. Each swatch carries its own data-theme,
 * so the theme's custom properties (and per-corner dot colours) apply to the preview directly.
 */
export function ThemePicker(props: ThemePickerProps) {
  return (
    <div class="picker" classList={{ open: props.open }} onClick={() => props.onClose()} aria-hidden={!props.open}>
      <div class="picker-inner" onClick={(event) => event.stopPropagation()}>
        <header class="picker-head">
          <h1>Themes</h1>
          <button type="button" class="picker-close" aria-label="Schließen" onClick={() => props.onClose()}>
            ×
          </button>
        </header>
        <For each={THEME_GROUPS}>
          {(group) => (
            <section class="picker-group">
              <h2>
                {group.name} <span>{group.note}</span>
              </h2>
              <div class="picker-grid">
                <For each={group.themes}>
                  {(theme) => (
                    <button
                      type="button"
                      class="swatch"
                      classList={{ active: props.current === theme }}
                      data-theme={theme}
                      onClick={() => props.onSelect(theme)}
                    >
                      <span class="swatch-face">
                        <For each={[1, 2, 3, 4]}>{() => <span class="swatch-dot" />}</For>
                        <span class="swatch-word">
                          <b>ES</b>
                          <i>K</i>
                          <b>IST</b>
                        </span>
                      </span>
                      <span class="swatch-name">{themeLabel(theme)}</span>
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
