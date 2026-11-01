import { For } from 'solid-js';

const KEYS: [string, string][] = [
  ['T', 'Alle Themes zur Auswahl'],
  ['← →', 'Theme zurück / weiter'],
  ['A', 'Tageslauf: Theme nach Sonnenstand (Sunrise, Lagoon, Sunset, Nightsurf)'],
  ['K', 'Feste automatisch: Karneval, Ostern, Mutter-/Vatertag, Halloween, Nikolaus, Advent, Silvester'],
  ['L', 'Alle Sprachen zur Auswahl'],
  ['↑ ↓', 'Sprache zurück / weiter'],
  ['M', 'Modus: Live, Demo, alle Wörter'],
  ['I', '„Es ist" ein/aus'],
  ['P', 'Minutenpunkte ein/aus'],
  ['F', 'Vollbild (oder Doppelklick)'],
  ['S', 'Uhr vollflächig statt quadratisch'],
  ['R', 'Sekundenrahmen ein/aus'],
  ['D', 'Wochentag und Datum ein/aus'],
  ['E', 'Buchstaben-Effekt ein/aus (je Theme)'],
  ['U', 'Übergangseffekt beim Zeitwechsel ein/aus'],
  ['B', 'Hintergrund-Effekt ein/aus (Blätter, Blasen, Konfetti …)'],
  ['Leertaste', 'Sekunden groß anzeigen (oder Klick auf die Uhr)'],
  ['?', 'Diese Hilfe'],
];

export function Help(props: { open: boolean; onClose: () => void }) {
  return (
    <div class="help" classList={{ open: props.open }} onClick={() => props.onClose()} aria-hidden={!props.open}>
      <dl>
        <For each={KEYS}>
          {([key, label]) => (
            <>
              <dt>
                <kbd>{key}</kbd>
              </dt>
              <dd>{label}</dd>
            </>
          )}
        </For>
      </dl>
      <p>Einstellungen gehen auch per URL: ?locale=en&amp;theme=metal&amp;mode=demo&amp;stretch=1</p>
    </div>
  );
}
