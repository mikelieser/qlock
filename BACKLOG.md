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
- [x] Fest-Themes automatisch zum Datum (Taste `K`, im Bildschirmschoner „Feste automatisch“).
- [x] Theme nach Sonnenstand (Taste `A`, „Tageslauf“), fest für Koblenz. Offen: Ort vom Hub, weiche Überblendung,
      eigene Zuordnung pro Person.
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

## Produktidee (01.11.2026, offen)

- Web-Uhr frei auf einer Website als Schaufenster. Bildschirmschoner für Mac und Windows als Einmalkauf für
  9,99 € (entschieden 01.11.2026, eine Lizenz für beide Systeme, alle Themes und Updates), alle Optionen.
  Zum Start eventuell ein Einführungspreis.
- Eigene Themes und Zifferblätter bauen (Theme-Editor im Browser, Vorschau per WebAssembly-Renderer),
  Community-Store mit Abstimmung.
- Vor jedem Verkauf: anderer Name (nicht „QLOCK“), eigene Raster für Deutsch und Englisch (die jetzigen stammen
  von 2014 und lehnen sich an das Original an), Marken- und Designrecht prüfen lassen, Lizenz des eigenen Codes
  klären (derzeit GPL-2.0 in package.json).
- Technik: Mac-Signatur und Notarisierung (Apple Developer, ca. 99 €/Jahr), Windows-Bildschirmschoner (.scr) mit
  Code-Signatur.
- Vertrieb (zu prüfen): Mac App Store nimmt keine reinen Bildschirmschoner (.saver) an, nur sandboxed Apps →
  Bildschirmschoner direkt verkaufen, App Store höchstens für eine Uhr-App. Microsoft Store nimmt klassische
  Installer (EXE/MSI) an, darüber ginge die .scr. Zahlung beim Direktverkauf: Stripe Managed Payments
  (Merchant of Record, Stripe hat Lemon Squeezy übernommen) – entschieden 01.11.2026, kein Paddle.
- Plattformen (01.11.2026): Mac- und Windows-Bildschirmschoner, Web-Uhr. Dazu interessant: iPhone/iPad-Widgets
  inklusive StandBy (Nachttischuhr am Ladekabel; Widgets aktualisieren je Minute, kaum Animation). Apple TV nur für
  Anlässe, bei denen die Uhrzeit im Mittelpunkt steht (z. B. Silvester-Countdown, Weihnachten). Wallpaper Engine,
  Smartwatches, Android TV vorerst nein.
- Konten (01.11.2026): Anschauen, Blättern, Einstellungen im Web für alle ohne Konto. Für mehr (eigene Themes und
  Buchstabenblätter speichern, veröffentlichen, abstimmen) Anmeldung per E-Mail mit Magic Link, Newsletter als
  eigenes Häkchen (Double-Opt-in). Kaufkunden: MCP-Zugang für Themes und Buchstabenblätter, KI-Budget für den
  Theme-Assistenten. Gebaut wird nur im Web; Bildschirmschoner sind reine Konsumenten (Anmeldung per Code im
  Browser, Token im Schlüsselbund, Themes und Blätter per API holen und offline zwischenspeichern).
- Server: Next.js mit Turso (SQLite) als Datenbank, Magic-Link-Mails über einen Mail-Dienst, Stripe-Webhook für
  Lizenzen, MCP-Endpunkt und KI-Proxy im selben Projekt. Achtung: Vercel Hobby ist nur nicht-kommerziell; für den
  Verkauf Vercel Pro oder ein anderer Hoster.
- Store: Themes nur als Daten (geprüfte CSS-Teilmenge, keine eigenen Skripte), Moderation, Melde-Funktion,
  Rechte an hochgeladenen Bildern, Impressum und Datenschutz.
