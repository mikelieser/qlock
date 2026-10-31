# Hinweise für Agents

- Web-Wortuhr (Vite + Solid + TypeScript). Doku auf Deutsch, Code und Kommentare auf Englisch.
- Uhrenlogik ist rein (`src/core`, `src/locales`) und per Vitest getestet: nach Änderungen `bun run test` und `bun run build` (nicht `bun test`, das ist Buns eigener Runner). Pakete nur mit `bun add`, kein npm.
- Wörter immer über `grid.word(row, col, 'TEXT')` adressieren, nie über rohe Zellnummern (Ausnahmen: Bilder im Formrausch-Locale, Pixelziffern in `core/seconds.ts`). Neue Dialekte am einfachsten mit `layoutGrid` aus Wortlisten.
- Sprachgruppen stehen in `src/locales/index.ts`, Theme-Gruppen in `src/settings.ts`; beide bestimmen auch die Reihenfolge der Pfeiltasten.
- Bedienung nur über Tastatur, keine sichtbaren Buttons auf der Uhr.
- Themes nur über Custom Properties in `src/styles.css`; keine Laufzeit-Abhängigkeiten ohne guten Grund.
- Offene Ideen stehen in `BACKLOG.md`, erledigte Punkte dort abhaken. Hardware-Konzept in `docs/hardware.md`, Sprachanalyse in `docs/sprachen.md`.
- Privates Familienprojekt; eigene Raster statt QLOCKTWO-Raster (außer `de`/`en` aus 2014).
