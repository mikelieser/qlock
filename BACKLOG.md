# Backlog

## Bildschirmschoner (Hauptziel)

- [x] Eigenes `QLOCK.saver` (Swift + `WKWebView`), kein GitHub Pages, kein WebViewScreenSaver.
- [x] Build als einzelne Datei (`vite build --mode saver` mit `vite-plugin-singlefile`).
- [x] Optionsdialog in den Systemeinstellungen → URL-Parameter, Listen automatisch aus der Web-App.
- [ ] Animationen im Bildschirmschoner: WebKit hält die Seite dort für unsichtbar (kein Fenster) und friert
      CSS-Animationen ein; deshalb läuft er mit `?still`. Ideen: Effekte per JS-Timer statt CSS, eigenes Fenster.
- [ ] Im echten Betrieb testen: Systemeinstellungen, Sperrbildschirm, Aufwachen.
- [ ] Pro Bildschirm eigenes Theme oder „zufällig".
- [ ] Theme `auto` nach Tageszeit / Sonnenstand (passt gut, weil man den Bildschirmschoner nicht bedient).
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
- [ ] Uhr als Abbild der Wetterlage draußen, so genau wie möglich (Idee 01.11.2026, über Home Assistant):
      Sonnenstand und Mondphase (offline berechenbar), Regen und wie stark, Schnee, Wind/Sturm, Wolken,
      Gewitter, Nebel, Temperatur; Unwetterwarnungen deutlich sichtbar. Themes werden nach und nach
      „echtzeitwettertauglich“ (nicht alle müssen), wie genau ist noch offen. Details in
      [docs/hardware.md](docs/hardware.md#umgebung-und-wetter).
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
