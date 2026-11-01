# qlock

> **Versionsverwaltung: natives Git, kein GitButler.** Commits, Branches und Pushes laufen hier mit `git`
> selbst. Bitte kein `but` und keinen GitButler-Workspace verwenden – das gilt auch für Agents.

Eine Wortuhr für den Browser – und bald für die Wand. Die Uhr zeigt die Zeit als Satz in einem Buchstabenraster
(„ES IST FÜNF VOR HALB NEUN"), vier Eckpunkte zählen die Minuten dazwischen. Angelehnt an die QLOCKTWO von
Biegert & Funk; ein privates, nicht-kommerzielles Familienprojekt.

Angefangen 2014 als CoffeeScript-Spielerei, 2026 neu gebaut mit Vite, Solid und TypeScript – und dabei
ordentlich gewachsen: 26 Sprachen und Dialekte, 42 Themes für jedes Familienmitglied und jedes Fest, Effekte, Sekunden,
Datum und ein Konzept für eigene Hardware.

## Was die Uhr kann

- **26 Sprachen und Dialekte** – Hochdeutsch, 16 deutsche Dialekte von Schwäbisch bis Ostfriesisch, acht
  europäische Sprachen und Japanisch im Kanji-Raster. Jede Sprache bringt ihre Eigenheiten mit: „viertel
  vier" im Schwäbischen, „fem i halv tre" im Schwedischen, „MIDI" und „MINUIT" im Französischen.
- **42 Themes**, gruppiert nach Familienmitglied: Klassiker, Strand und Retro, Pastell, Chaos, Herbst, dazu Motto-Themes von Karneval bis Silvester.
- **Drei Effekt-Ebenen**, einzeln schaltbar: Buchstaben bewegen sich (wellen, wackeln, flackern,
  schimmern), Zeitwechsel werden sichtbar (neue Wörter stellen sich scharf), der Hintergrund lebt (fallende
  Blätter, Blubberblasen, Glut, Konfetti, pulsierender Regenbogen).
- **Sekunden**: dezenter Rahmen, der einmal pro Minute ums Zifferblatt läuft, oder große Ziffern im Raster.
- **Datum**: Wochentag und Datum klein im oberen Rand, in der Sprache der Uhr.
- **Vollbild und vollflächig**: quadratisch oder auf die ganze Bildschirmfläche gestreckt.
- **Auswahl-Übersichten** für Themes und Sprachen mit Live-Vorschau.
- **Einstellungen** bleiben im Browser gespeichert und lassen sich per URL setzen.
- **Easter Eggs**: Frühstück um 11:15 und Feierabend um 18 Uhr im Formrausch-Raster, regionale Wörter in
  den Füllbuchstaben der Dialekte, ein Halloween-Geburtstags-Theme mit Spinnen und Fledermaus.

## Loslegen

```sh
bun install
bun dev            # http://localhost:5173 mit Hot Reload
bun run test       # Uhrenlogik aller Sprachen (Vitest)
bun run build      # Typecheck + Build nach dist/
bun run preview    # gebauten Stand ansehen
bun run saver:install  # Mac-Bildschirmschoner bauen und installieren
```

Paketmanager ist [Bun](https://bun.sh). Achtung: `bun test` (ohne `run`) startet Buns eigenen Test-Runner
statt Vitest, deshalb `bun run test`.

## Bedienung

Alles läuft über die Tastatur, `?` zeigt die Kürzel. Es gibt bewusst keine Buttons auf der Uhr.

| Taste | Wirkung |
|---|---|
| `T` | Theme-Übersicht (Standard, Mike, Susanna, Henry, Katja) |
| `←` `→` | Theme zurück / weiter |
| `L` | Sprach-Übersicht mit Beispielsatz (Deutsch, Dialekte, Europa, Asien) |
| `↑` `↓` | Sprache zurück / weiter |
| `E` | Buchstaben-Effekt ein/aus |
| `U` | Übergangseffekt beim Zeitwechsel ein/aus |
| `B` | Hintergrund-Effekt ein/aus |
| `A` | Tageslauf: Theme nach Sonnenstand |
| `K` | Feste automatisch: Karneval, Ostern, Mutter-/Vatertag, Halloween, Nikolaus, Advent, Silvester |
| `R` | Sekundenrahmen ein/aus |
| Leertaste / Klick auf die Uhr | Sekunden groß im Raster, nochmal oder `Esc` zurück |
| `D` | Wochentag und Datum ein/aus |
| `I` | „Es ist" ein/aus (wie bei der echten Uhr; schaltet auch Berliner „WA" und hessisches „GELL") |
| `P` | Minutenpunkte ein/aus |
| `S` | Quadratisch oder vollflächig |
| `F` / Doppelklick | Vollbild |
| `M` | Modus: Live, Demo (springt zufällig durch den Tag), alle Wörter |
| `?` | Hilfe |

Nach 2,5 Sekunden ohne Mausbewegung verschwindet der Cursor. Der Tab-Titel zeigt die Uhrzeit als Text.

### Per URL

Alle Einstellungen lassen sich per URL setzen und gehen dem Gespeicherten vor, z. B.
`/?locale=swa&theme=autumn-leaves&ambient=1`:

| Parameter | Werte |
|---|---|
| `locale` | Sprachkürzel, siehe unten |
| `theme` | Theme-Name, siehe unten |
| `mode` | `live`, `demo`, `all` |
| `intro`, `dots`, `stretch`, `progress`, `date`, `effect`, `transition`, `ambient`, `festive`, `daylight` | `1` / `0` |

## Mac-Bildschirmschoner

`bun run saver:install` baut `QLOCK.saver` und legt ihn nach `~/Library/Screen Savers`. Danach in den
Systemeinstellungen unter **Hintergrundbild → Bildschirmschoner …** „QLOCK" wählen (einen eigenen Bereich
„Bildschirmschoner" gibt es nicht mehr; wann er startet, steht unter **Sperrbildschirm**). **Optionen …** öffnet
einen Dialog mit Live-Vorschau, Sprache, allen Themes als Vorschaubilder nach Gruppen und den Schaltern für
„Anzeige“ und „Bewegung“. Änderungen zeigt die Vorschau sofort; gespeichert wird mit OK.

- Der Bildschirmschoner zeichnet **nativ**: Swift-Hülle (`saver/Sources`) um den Rust-Renderer aus `native/`,
  zusammengesetzt mit Core Animation. Animationen laufen flüssig, ohne WebKit.
- Warum nicht die Web-App: macOS hängt die Ansicht im Bildschirmschoner-Prozess aus ihrem Fenster. WebKit hält
  die Seite dann für unsichtbar, friert Animationen ein und verwirft nach ein paar Sekunden die Darstellung.
- Sprachen und Themes kommen als Daten aus der Web-App: `native/export-locales.ts` (jede Minute jeder Sprache)
  und `native/export-themes.ts` (alle CSS-Variablen jedes Themes, Kaskade und `var()` aufgelöst). Der Rust-Renderer
  liest Verläufe, SVG-Deko, Schatten, Schriften und Effekte direkt aus diesen CSS-Werten; alle 42 Themes, alle
  Buchstaben-Effekte, Übergang, Hintergrund-Teilchen und Sekundenrahmen. Was CSS mit eigenen Selektoren macht
  (Spinnen, Fledermaus, Kerzen, Strahlen, Konturen) steht in `native/render/src/special.rs`.
- Vergleich Web gegen nativ: `native/tools/compare.sh <ordner> [themes…]` (braucht `bunx vite preview --port 4173`)
  legt für jedes Theme ein Bild „links Web, rechts nativ“ ab.
- Braucht Rust (`cargo`), gebaut wird für Apple Silicon. Ad-hoc signiert, für den eigenen Mac reicht das.
- Seit macOS Sonoma hält der Systemprozess alte Instanzen am Leben und ruft Start/Stopp unzuverlässig auf.
  QLOCK beendet den Prozess deshalb erst, wenn 10 Sekunden nach dem Ausblenden keine neue Instanz entstanden ist.

**Messen ohne Bildschirm:** `bun run saver:bench <ordner> [optionen]` lädt den gebauten Bildschirmschoner,
treibt ihn mit simulierter Zeit und rendert per `CARenderer` genau das, was Core Animation zeigen würde, als
PNG-Bilder. Ausgabe: Aufbauzeit, Zeit pro Frame, mit `only=particle:N` die Bahn eines einzelnen Funkens
(Ruckler, Stillstand, Sprünge). Beispiele:

```sh
bun run saver:bench /tmp/q only=particle:3 seconds=20 frames=none      # läuft ein Funke flüssig?
bun run saver:bench /tmp/q frames=png minute-change=3 video=~/Desktop/q.mp4  # Clip mit Minutenwechsel
```

Optionen: `theme=` `locale=` `seconds=` `fps=` `size=1512x982` `frames=png|none` `only=particles|particle:N`
`minute-change=<s>` `realtime=1` `video=<datei.mp4>` (braucht ffmpeg).

Architektur und Stand des gemeinsamen Renderers für Mac und Hardware-Uhren: [docs/hardware.md](docs/hardware.md#gemeinsamer-renderer-mac-bildschirmschoner-und-uhren).

## Sprachen

Außer `de` und `en` (übernommen aus dem Stand von 2014) sind alle Raster eigene Entwürfe. In den
Füllbuchstaben der Dialekte steckt jeweils ein Wort aus der Gegend.

**Deutsch**

| | Beispiel |
|---|---|
| `de` Deutsch | ES IST VIERTEL NACH DREI |
| `formrausch` Formrausch | wie Deutsch, mit Formrausch-Raster, Frühstücks- und Feierabend-Bildern |

**Dialekte** (von Süd nach Nord grob sortiert)

| | Beispiel | Versteckt |
|---|---|---|
| `swa` Schwäbisch | ES ISCH VIERTEL DREI (2:15) · DREIVIERTEL DREI | SCHAFFA |
| `bad` Badisch | ES ISCH VIERTEL DRÜ · FÜNF NOCH ZWEI | BÄCHLE |
| `bar` Bayerisch | ES IS HOIBE DREI · DREIVIERTL ZWÄIF | OZAPFTIS |
| `frk` Fränkisch | ES IS HALB SIEM · DREIVIERDL DREI | BRATWURST |
| `wien` Wienerisch | ES IS HOIB ÖF · VIERTL DREI | SCHMÄH, LEIWAND |
| `gsw` Schweizerdeutsch | ES ISCH FÜF VOR HALBI DRÜÜ | MERCI |
| `sxu` Sächsisch | ES IS DREIVIERDL DREIE · HALB EENS | BEMME |
| `ber` Berlinerisch | ET IS VIERTEL DREI WA | CURRYWURST |
| `nds` Hamburger Platt | KLOCK IS VEERDEL NA TWEE | MOIN |
| `ofr` Ostfriesisch | DAT IS KWART NA TWEE | MOIN |
| `ruhr` Ruhrpott | ET IS ZWANZICH VOR DREI | WATT |
| `ksh` Kölsch | ET ES VEEDEL NOH ZWEI · FÜNNEF VÖR HALV DREI | ALAAF |
| `kob` Koblenzer Platt | ET ES VEERTEL NOH ZWEE · ZÄHN VÜR HALWER DREI | |
| `hes` Hessisch | ES IS ZWANZISCH NACH ZWAA GELL | GELL |
| `pfl` Pfälzisch | ES IS ZWANZISCH NOCH ZWÄÄ | DUBBEGLAS |
| `saar` Saarländisch | ES IS VIERTEL NOH ZWOO (11 Zeilen) | HALLO |

Die Dialektformen sind nach bestem Wissen geschrieben, nicht von Muttersprachlern geprüft. Korrekturen sind
meist nur ein paar Wörter in einer Liste.

**Europa und Asien**

| | Beispiel |
|---|---|
| `en` Englisch | IT IS A QUARTER PAST THREE |
| `nl` Niederländisch | HET IS TIEN VOOR HALF DRIE (2:20) |
| `da` Dänisch | KLOKKEN ER FEM I HALV TRE |
| `sv` Schwedisch | KLOCKAN ÄR FEM I HALV TRE |
| `fr` Französisch | IL EST DEUX HEURES ET QUART · MIDI · MINUIT |
| `es` Spanisch | ES LA UNA · SON LAS DOS Y CUARTO |
| `it` Italienisch | È L'UNA · SONO LE DUE E UN QUARTO |
| `el` Griechisch | ΕΙΝΑΙ ΔΥΟ ΚΑΙ ΤΕΤΑΡΤΟ |
| `ja` Japanisch | 今は午後二時十五分です (6×6-Kanji-Raster) |

Welche weiteren Sprachen gut oder schlecht auf eine Wortuhr passen (Arabisch und Thai eher nicht), steht in
[docs/sprachen.md](docs/sprachen.md).

## Themes

| Gruppe | Themes |
|---|---|
| Standard – Klassiker | `standard`, `formrausch`, `graphite`, `paper`, `brass`, `sage`, `blue`, `red`, `metal`, `gradient`, `ticktack` (Kreidetafel), `timeonly` |
| Mike – Strand, Sonne, Retro | `sunrise`, `lagoon`, `sunset`, `nightsurf`, `dunes`, `retro`, `retro-sun`, `retro-stripes`, `retro-night`, `retro-pastel` |
| Susanna – Pastell | `cotton-candy`, `lavender`, `bubblegum`, `sky-dream`, `unicorn` |
| Henry – Chaos | `chaos`, `chaos-burst`, `chaos-night` |
| Katja – Herbst | `autumn-leaves`, `golden-hour`, `forest-mist`, `cozy` |
| Feste – Motto-Themes durchs Jahr | `karneval`, `ostern`, `muttertag`, `vatertag`, `halloween` (Spinnen, Fledermaus, Kerzen), `nikolaus`, `weihnachten`, `silvester` |

Mit „Feste automatisch“ (`K`) übernimmt das Fest-Theme zur passenden Zeit: Karneval
Weiberfastnacht bis Veilchendienstag, Ostern Palmsonntag bis Ostermontag, Muttertag (2. Sonntag im Mai), Vatertag (Christi Himmelfahrt),
Halloween 24.–31. Oktober, Nikolaus 6. Dezember, Weihnachten
1. Advent bis 26. Dezember, Silvester 27.–31. Dezember. Sonst gilt das gewählte Theme.

Mit „Tageslauf“ (`A`) folgt das Theme der Sonne über Koblenz: Sunrise in der Morgendämmerung, Lagoon am Tag,
Sunset am Abend, Nightsurf nachts (Grenzen: −6° und 8° Sonnenhöhe). Ein Fest-Theme geht vor. Rechnung in
`src/core/daylight.ts` und `native/core`, gegen dieselben Zeitpunkte getestet.

Ein Theme ist ein Block Custom Properties in `src/styles.css` unter `[data-theme='…']`: Hintergrund,
Buchstabenfarben, Eckpunkt-Farben (`--dot-1` … `--dot-4`), Buchstaben-Effekt (`--fx-*`) und
Hintergrund-Effekt (`--amb-*`). Feste Deko wie liegende Blätter oder Regenbögen gehört in `--face-deco`,
dann kann der Hintergrund-Effekt sie animieren.

## Aufbau

```
src/
  core/grid.ts          Raster aus Zeilen; grid.word(row, col, 'TEXT') liefert Zellen und prüft den Text
  core/layout.ts        Raster aus Wortlisten pro Zeile bauen (für Dialekte), Füllbuchstaben automatisch
  core/clock.ts         Baustein für Sprachen, die erst die Minuten und dann die Stunde nennen
  core/german.ts        Satzbau Hochdeutsch (de und formrausch)
  core/seconds.ts       Sekunden als 5×7-Pixelziffern
  core/date.ts          Wochentag und Datum in der Sprache der Uhr
  locales/*.ts          je Sprache Raster + Wörter + frame(now); Gruppen in locales/index.ts
  components/Clock.tsx  Raster, Eckpunkte, Sekundenrahmen, Datum, Hintergrund-Teilchen
  components/*Picker.tsx  Theme- und Sprachübersicht
  components/Spooky.tsx   Spinnen und Fledermaus fürs Katja-Theme
  App.tsx               Einstellungen, Tastatur, Timer, Vollbild
  settings.ts           Defaults, Theme-Gruppen, localStorage, URL-Parameter
native/
  core/                 Rust: Uhrenlogik aus exportierten Sprachdaten (no_std)
  render/               Rust: Themes (TOML), Sprites, Animation pro Frame
  ffi/                  C-Schnittstelle für den Bildschirmschoner
  preview/              Ein Frame als PNG, zum Vergleich mit der Web-App
  themes/*.toml         portierte Themes
  export-locales.ts     schreibt data/locales.json aus src/locales
saver/
  Sources/*.swift       Bildschirmschoner: Core-Animation-Hülle um die Rust-Engine, Optionsdialog
  options.ts            Sprach-/Theme-Listen für den Optionsdialog
  build.sh              baut QLOCK.saver (universal, ad-hoc signiert), --install installiert
docs/
  hardware.md           Konzept für eigene Hardware-Uhren
  sprachen.md           Welche Sprachen auf eine Wortuhr passen und warum
  groessen-muster.html  Papiermuster der Displaygrößen (Quelle)
  groessen-muster.pdf   dasselbe als fertiges PDF zum Drucken
```

**Neue Sprache:** Raster anlegen (von Hand mit `createGrid` oder per `layoutGrid` aus Wortlisten), Wörter
adressieren, `frame()` schreiben bzw. `minutesFirst()` nutzen, in `locales/index.ts` einer Gruppe zuordnen,
Testfälle in `locales.test.ts` ergänzen. Ein Test läuft für jede Sprache alle 1440 Minuten des Tages durch
und prüft, dass die Wörter in Leserichtung leuchten.

**Neues Theme:** Block in `src/styles.css`, Name in `THEME_GROUPS` in `src/settings.ts`.

## Hardware

Langfristig sollen echte Uhren an der Wand hängen: quadratische Displays von 4 bis 15 Zoll, ESP32-P4 mit
Rust-Firmware, ein Elixir-Hub zu Hause für Profile, Zeitpläne und Nachrichten, Ansagen über Quak auf die
Sonos-Boxen. Größen, Pixeldichten, LCD vs. OLED vs. E-Ink, Strom, Kosten und offene Fragen stehen in
[docs/hardware.md](docs/hardware.md).

## Vergleich mit dem Original

| | QLOCKTWO | qlock |
|---|---|---|
| Sprachen pro Uhr | eine (Frontplatte tauschen) | 26, per Taste |
| Optik | Frontplatte aus Material | 42 Themes, jederzeit änderbar |
| Effekte | – | drei Ebenen, pro Theme eigene |
| Sekunden | große Ziffern | große Ziffern oder Sekundenrahmen |
| Datum | – | ja |
| Preis | ab 760 € (EARTH 13.5) bzw. 1.600 € (EARTH 45) | Display ab ca. 25–75 $ (geplant) |

Das Original ist ein handgefertigtes Designobjekt; diese Uhr ist ein Bastelprojekt. Name und Raster der
QLOCKTWO sind geschützt, deshalb nur privat und mit eigenen Rastern.

## Geschichte

- **2014**: CoffeeScript, SCSS, jQuery, Velocity, Moment; Deutsch, Englisch, Formrausch, Tick-Tack.
- **2026**: Neubau mit Vite, Solid und TypeScript. Größe und Schrift rechnet CSS (`cqw`, `vmin`) statt
  JavaScript, Wörter werden über Zeile/Spalte plus Text adressiert statt über Zellnummern, die Uhrenlogik
  ist getestet. Tick-Tack ist als Sprache entfallen, das Theme ist geblieben.
