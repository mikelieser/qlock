# Backlog

## Bildschirmschoner (Hauptziel)

- [ ] Weg festlegen: macOS-`.saver` mit `WKWebView`, der den `dist/`-Build lädt, oder ein fertiger
      Web-Screensaver-Wrapper (z. B. WebViewScreenSaver). Erst Wrapper testen, dann ggf. eigenes Bundle.
- [ ] Build als einzelne Datei (`vite-plugin-singlefile` o. ä.): `type="module"`-Skripte laden nicht
      von `file://`, ein `.saver` braucht aber lokale Dateien.
- [ ] Optionen ohne Tastatur: Einstellungsdialog des Screensavers → URL-Parameter.
- [ ] Mehrere Bildschirme / Retina prüfen, CPU-Last messen (Ziel: praktisch 0 im Leerlauf).
- [ ] Burn-in-Schutz: Uhr langsam ein paar Pixel wandern lassen (für OLED ein Muss).
- [ ] CPU-Last der Effekte messen (Schimmer, Flackern, Teilchen).

## Hardware

- [ ] Eigene Wortuhren mit Display für zu Hause: Konzept, Größen, Displaytechnik und nächste Schritte in
      [docs/hardware.md](docs/hardware.md).

## Familie und Alltag

- [ ] Profile pro Person (`?profile=henry`): Lieblings-Themes, Sprache, Tagesplan.
- [ ] Theme nach Sonnenstand: Morgen, Tag, Abend, Nacht für den eigenen Ort offline berechnen,
      z. B. `sunrise` → `lagoon` → `sunset` → `nightsurf`; weich überblenden.
- [ ] Erinnerungen mit Textanzeige („Aufwachen", „Auf geht's zur Schule"), gekoppelt mit Quak-Ansagen.
- [ ] Dialekte von Leuten aus der Gegend gegenlesen lassen (v. a. Koblenz, Pfalz, Ostfriesland, Schweiz).

## Web

- [x] Theme- und Sprachübersicht (`T`, `L`) mit Live-Vorschau, gruppiert.
- [ ] Bedienung ohne Tastatur für Tablets, aber ohne sichtbare Buttons (z. B. langer Druck auf die Uhr).
- [ ] Screen Wake Lock im Vollbild, damit Tablet/Monitor nicht ausgeht.
- [ ] PWA: Manifest + Service Worker, offline und „Zum Home-Bildschirm".
- [ ] Nachtmodus: abends automatisch gedimmt (Zeitfenster oder `prefers-color-scheme`).
- [ ] Auto-Theme: Theme pro Tag oder Stunde wechseln (der alte Stand konnte ein zufälliges aus einer Liste wählen).
- [x] Effekte: Buchstaben (`E`), Übergang beim Zeitwechsel (`U`), Hintergrund (`B`), je Theme eigene.
- [ ] Weitere Übergänge (Buchstaben fliegen, Scramble).
- [x] Weitere Sprachen und 16 Dialekte, siehe [docs/sprachen.md](docs/sprachen.md).
- [ ] Weitere Kandidaten: Norwegisch, Ungarisch, Portugiesisch, Chinesisch, Koreanisch.
- [x] Sekunden-Variante: großer Sekundenzähler wie die QLOCKTWO „Seconds"-Ansicht beim Antippen.
- [x] Alternative zur Sekundenzahl: feiner Fortschrittsrahmen, der einmal pro Minute ums Zifferblatt läuft.
- [x] Wochentag und Datum (`D`).
- [ ] Deploy (Vercel oder S3 im Privatkonto `mike`).

## Pflege

- [ ] Prettier/ESLint oder Biome, falls das Projekt wächst.
- [ ] Bilder (`metal/bg.png`, 118 kB) nach WebP/AVIF.
- [ ] Hintergrundmuster des Formrausch-Themes als CSS statt JPG.
