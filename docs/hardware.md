# Hardware-Konzept: eigene Wortuhren für zu Hause

Stand: 31.10.2026. Sammlung aus der Diskussion, noch keine Entscheidung. Nichts davon eilt.

Die Web-App in diesem Repository ist der Prototyp: Themes, Sprachen und Effekte entstehen hier und sind die
Vorlage für die Geräte.

## Idee

Jeder in der Familie (Mike, Susanna, Henry) hat eine eigene Wortuhr mit eigenen Themes. Die Uhren laufen
zeitsynchron, wechseln das Theme nach Tageszeit und Sonnenstand und können Nachrichten, Bilder und
Erinnerungen anzeigen („Aufwachen", „Fertig machen", „Auf geht's zur Schule"). Ton kommt nicht aus der
Uhr, sondern über Quak auf die Sonos-Boxen: **ein API-Aufruf, zwei Spuren**, Audio auf Sonos und das Bild
auf dem Display im selben Raum – das Display als visuelles „Tor ins Zimmer".

Rahmenbedingungen:

- **Display statt LEDs hinter Buchstabenschablone**: Die Uhr muss grafisch jederzeit komplett anpassbar sein.
- **Nur quadratische Uhren**, keine runden Displays.
- **Kein Touch**, höchstens zwei bis drei Knöpfe seitlich am 3D-gedruckten Gehäuse.
- **Kein Lautsprecher, kein Mikrofon** (Ton über Sonos/Quak).
- **Bewegungsmelder** ja: Uhr nur an, wenn jemand im Raum ist.
- **Strom per Kabel** (USB-C, 5 V) – außer bei E-Ink, dort wäre Akku möglich.
- **Nativ statt Web**: Auf den Geräten läuft keine Web-App, sondern eigene Firmware. Die Web-App in diesem
  Repository ist der Prototyp und Theme-Spielplatz.
- **Firmware-Updates per WLAN** und **Smart-Home-Integration** sind Pflicht.

## Größenstufen

Zoll = Diagonale, 1 Zoll = 2,54 cm. Quadratische Displays gibt es fertig nur bis 4 Zoll; darüber nimmt man
ein rechteckiges Panel hinter einem Rahmen mit quadratischem Ausschnitt (bei OLED ist der Rest einfach
schwarz).

| Display | Auflösung | PPI | Quadrat für die Uhr | Buchstaben¹ | Pixel unsichtbar ab² | Gut lesbar bis³ | Hardware | Preis ca.⁴ |
|---|---|---|---|---|---|---|---|---|
| 4" quadratisch | 480×480 | 170 | 7,2 × 7,2 cm | 2,3 mm | 51 cm | 0,7 m (max. 1,2 m) | ESP32-S3 | 20–28 $ |
| 4" quadratisch | 720×720 | 255 | 7,2 × 7,2 cm | 2,3 mm | 34 cm | 0,7 m (max. 1,2 m) | ESP32-P4 | 65–75 $ |
| 7" | 1024×600 | 170 | 9,0 × 9,0 cm | 2,9 mm | 51 cm | 0,9 m (max. 1,4 m) | ESP32-P4 | 47–56 $ |
| 10,1" | 800×1280 | 149 | 13,6 × 13,6 cm | 4,4 mm | 59 cm | 1,3 m (max. 2,2 m) | ESP32-P4 | 74–88 $ |
| 13,3" | 1920×1080 | 166 | 16,6 × 16,6 cm | 5,3 mm | 53 cm | 1,6 m (max. 2,7 m) | Linux-Board | offen |
| 15,6" | 1920×1080 | 141 | 19,4 × 19,4 cm | 6,2 mm | 62 cm | 1,9 m (max. 3,1 m) | Linux-Board | offen |
| 15,6" | 3840×2160 | 282 | 19,4 × 19,4 cm | 6,2 mm | 31 cm | 1,9 m (max. 3,1 m) | Linux-Board | offen |
| 24" | 1920×1080 | 92 | 29,9 × 29,9 cm | 9,6 mm | 95 cm | 2,9 m (max. 4,8 m) | Linux-Board | offen |

¹ Höhe der Großbuchstaben im heutigen Layout (10 Zeilen, 14 % Rand).
² Abstand, ab dem das Auge einzelne Pixel nicht mehr auflöst (1 Bogenminute ≈ 87 cm ÷ PPI × 100).
³ Faustregel aus der Beschilderung: 1 mm Buchstabenhöhe ≈ 30 cm bequem, ≈ 50 cm gerade noch lesbar.
⁴ Händler- und Testangaben (Oktober 2026), keine aktuellen Angebote.

**Einordnung nach Raum** (vorläufig, wird mit Papiermustern getestet):

- **Neben dem Bett, am Schrank**: 4" reicht, man ist nah dran.
- **Zimmeruhr** (Küche, Flur, Kinderzimmer, Schreibtisch): 7–10".
- **Wohnzimmerwand**: ab 15", quer durch den Raum lesbar.

Zum Testen gibt es Papiermuster in Originalgröße: [`groessen-muster.pdf`](groessen-muster.pdf) direkt drucken
(Quelle: [`groessen-muster.html`](groessen-muster.html), in Chrome als PDF druckbar; 4", 7", 10,1", 13,3", 15,6"; Seitenformate A4/A3 sind hinterlegt). Unbedingt bei 100 % bzw.
„Tatsächliche Größe" drucken; auf jeder Seite ist eine 10-cm-Kontrolllinie. Gestrichelt ist jeweils das ganze
Panel eingezeichnet.

## Displaytechnik

### LCD (IPS)

Standard für alle ESP-Größen. Scharf, farbig, alle Themes und Effekte. Schwachpunkt bei einer Wortuhr:
**Schwarz ist nicht schwarz**, dunkle Themes leuchten nachts als graues Quadrat. Gegenmittel: nachts stark
dimmen, Bewegungsmelder, getönte Frontscheibe (wie bei der echten QLOCKTWO), helle Themes.

### OLED

Echtes Schwarz, Buchstaben schweben – optisch die schönste Variante.

- Für ESP quadratisch nur 2,16" mit 480×480 (Uhrenfläche knapp 4 × 4 cm, zu klein). Mittlere Größen gibt es
  für ESP praktisch nicht.
- Ab 13,3" gibt es OLED als tragbare Monitore (USB-C/HDMI) → Linux-Klasse. Kandidat für die Wohnzimmerwand.
- **Einbrennen** ist bei einer Wortuhr der kritische Punkt („ES IST" leuchtet rund um die Uhr an derselben
  Stelle). Gegenmittel von Anfang an im Renderer: Raster wandert langsam ein paar Pixel, gedimmte
  Helligkeit, aus bei Abwesenheit (Bewegungsmelder), wechselnder Farbton über den Tag.

### E-Ink

Papier-Look, kein eigenes Licht, praktisch kein Strom im Ruhezustand → **Akkubetrieb möglich**.

- Leuchtet nicht: im dunklen Schlafzimmer unsichtbar (ohne Frontlicht, das die üblichen Module nicht haben).
- Keine Effekte, kein Video. Farb-E-Ink (Spectra 6) braucht Sekunden bis Dutzende Sekunden pro Bild, nur für
  Bilder und Nachrichten geeignet, nicht für die Uhr.

| E-Ink (schwarz-weiß) | Auflösung | PPI | Teil-Update | Voll-Update | Anschluss |
|---|---|---|---|---|---|
| 7,5" | 800×480 | 124 | 0,4 s | 4 s | SPI am ESP |
| 10,3" | 1872×1404 | 227 | ja | < 1 s | Parallel, braucht Extra-Controller (IT8951) |
| 13,3" | 1600×1200 | 150 | 0,7 s | 5 s | SPI am ESP |

Herstellerangaben Waveshare; beim 7,5" widersprechen sich zwei Produktseiten (0,4 s vs. 1,5 s), Datenblatt
prüfen.

**Taktung bei E-Ink:**

- Jede Minute ein Teil-Update (Minutenpunkte, alle 5 Minuten neue Wörter), ca. 0,5 s ohne Flackern.
- Einmal pro Stunde ein Voll-Update gegen Geisterbilder (blinkt kurz schwarz-weiß), z. B. zur vollen Stunde
  oder wenn der Bewegungsmelder niemanden sieht.
- Dazwischen Tiefschlaf; ohne Bewegung im Raum Updates auslassen.
- Genaue Zeit über einen RTC-Baustein (z. B. DS3231), einmal täglich per NTP abgleichen – der interne Takt des
  ESP driftet im Tiefschlaf.
- Kompromiss: Im Tiefschlaf ist WLAN aus, Nachrichten kommen mit bis zu einer Minute Verzögerung an.
  Millisekundengenaue Durchsagen gehören auf die leuchtenden Uhren.

### Vorläufige Zuordnung

| Ort | Technik | Warum |
|---|---|---|
| Neben dem Bett | LCD 4", getönte Scheibe, nachts gedimmt, Bewegungsmelder | E-Ink nachts unsichtbar, OLED in der Größe nicht verfügbar |
| Küche, Flur, Kinderzimmer tagsüber | E-Ink 7,5" mit Akku | kein Kabel, ruhig, Papier-Look |
| Zimmeruhr mit Effekten | LCD 7–10", ESP32-P4 | alle Themes und Effekte |
| Wohnzimmerwand | OLED 15,6", Linux-Board | echtes Schwarz, mit Einbrennschutz |

## Prozessor-Plattformen

- **ESP32-P4** (RISC-V, 400 MHz, 32 MB PSRAM, MIPI-DSI bis 1080p, 2D-Beschleuniger PPA, JPEG-Codec, WLAN über
  zusätzlichen ESP32-C6): eine Plattform für **alle Größen von 4" bis 10,1"**. Pro Gerät ändern sich nur
  Displaytreiber und Auflösung. H.264 ist nur als Encoder belegt, nicht als Decoder.
- **ESP32-S3**: günstiger, schwächer, ausgereiftes Rust. Nur als Rückfallebene, sonst hätten wir zwei
  Plattformen zu pflegen.
- **Linux-Board** (Linux-Einplatinenrechner, z. B. mit Rockchip-Chip; kein Raspberry Pi gewünscht) ab 13":
  echte GPU, HDMI/USB-C-Monitore, Video.
- Gegen Raspberry Pi und iPad/Tablet entschieden: zu groß, zu viel Overhead, nicht die Richtung.

## Software

### Auf den Uhren: Rust

- Basis ESP-IDF über `esp-idf-svc` (WLAN, SNTP, OTA, MQTT/HTTP); Displaytreiber und PPA kommen aus ESP-IDF.
- **Eigener kleiner Renderer** statt UI-Framework: Hintergrundbild + vorgerenderte Glyphen + Alpha-Blending
  + Effekte. Alternativen: Slint (Rust, GPL bzw. kommerziell ab ca. 1 $/Gerät) oder LVGL (C, am
  ausgereiftesten auf ESP).
- **Tempo** hängt an der Architektur, nicht an der Sprache (Rust ≈ C): nur geänderte Bereiche neu zeichnen,
  Mischen per PPA, Double-Buffering, alles vorgerendert. Der Engpass ist die PSRAM-Bandbreite (720×720 ≈
  1 MB pro Bild, 800×1280 ≈ 2 MB).
- **Renderer plattformneutral**: dieselbe Zeichenlogik auf ESP und Linux, nur die unterste Schicht wechselt.
- **Risiko**: Rust auf dem P4 ist jung (Ziel `riscv32imafc-esp-espidf` ist gelistet, Vollständigkeit
  ungeprüft). Rückfall: Grafik mit LVGL, Logik in Rust – oder S3.
- Uhrenlogik (Zeit → Wörter) kommt als Daten aus der Web-App, siehe nächster Abschnitt.

### Gemeinsamer Renderer (Mac-Bildschirmschoner und Uhren)

Stand 01.11.2026, Branch `rust-renderer`, Code in `native/`. Auslöser: Ein Web-Bildschirmschoner kann auf
aktuellem macOS nicht flüssig animieren. Der Systemprozess hängt die Ansicht aus ihrem Fenster, WebKit hält die
Seite dann für unsichtbar, friert Animationen ein und verwirft nach ein paar Sekunden die Darstellung. Ein
eigener Renderer löst das für den Mac und ist zugleich der Renderer der Uhren.

```
src/locales (TypeScript)  ──export──▶  native/data/locales.json   jede Minute → leuchtende Wörter, alle Sprachen
native/themes/*.toml                                               Themes als Daten (Farben, Verläufe, Pfade, Effekte)
native/core     qlock-core     Daten → Frame (Zellen, Punkte), no_std
native/render   qlock-render   Sprites einmal rastern (tiny-skia + Inter), pro Frame nur Instanzen
                               (welches Sprite, wo, Deckkraft, Größe); alle Animationszeiten hier
        ├── Mac:   Instanzen 6 s voraus abgetastet (30/s) und als Keyframe-Spuren an Core Animation;
        │          der Render-Server interpoliert mit Bildschirmrate (saver/, über native/ffi)
        └── ESP32: kleiner Software-Blitter, nur geänderte Bereiche (noch nicht gebaut)
```

- **Sprachen bleiben in TypeScript.** `native/export-locales.ts` schreibt für jede Sprache jede Minute des
  Tages als Tabelle. Der Rust-Kern schlägt nur nach, damit sind alle 27 Raster ohne Port identisch.
- **Sprite-Prinzip:** Teures wird einmal gerastert (Hintergrund mit Verlauf und Deko, jeder Buchstabe aus,
  an und unscharf für den Übergang, Punkte, Teilchen, Datum). Pro Frame entstehen nur ca. 350 Instanzen in
  unter 0,1 ms. Der Mac setzt sie per Core Animation zusammen, der ESP32 blittet nur, was sich bewegt.
- **Mac-Ruckeln:** Jeden Frame Ebenen setzen ruckelte sichtbar (der Bildschirmschoner-Prozess reicht seinen
  Ebenenbaum nur stockend an den Bildschirm weiter). Deshalb Spuren statt Einzelbilder: ein Commit alle
  paar Sekunden, die Bewegung selbst rechnet der Render-Server.
- **Gemessen:** Engine-Aufbau 20–100 ms (einmal pro Sprache/Theme/Größe), Update pro Frame ca. 30 µs.
  Volle Software-Zusammensetzung 1600×1200 ca. 40 ms auf dem Mac, für den ESP32 also nur mit geänderten
  Bereichen.
- **Portiert:** Formrausch und Sunset, Effekte Atmen, Übergang (Fokus und Punkte) und aufsteigende Funken.
  Offen: übrige 30 Themes, Schimmer und andere Buchstaben-Effekte, Sekundenrahmen, Bild-Locale-Details.
- **Später:** derselbe Kern als WebAssembly im Browser, dann zeigen Web, Mac und Uhr pixelgleich dasselbe.

### Zentrale: Elixir-Hub

Elixir gehört nicht auf den ESP (nur AtomVM, keine brauchbare Grafik; Nerves bräuchte Linux), passt aber
perfekt als Hub zu Hause (Phoenix/LiveView):

- Theme-Designer mit Live-Vorschau, Profile pro Person, Tagespläne nach Sonnenstand
- Erinnerungen und Nachrichten, gekoppelt mit Quak (Audio auf Sonos)
- Verteilung von Theme-Paketen und Firmware-Updates
- MQTT mit Home-Assistant-Discovery: jede Uhr als Gerät mit Helligkeit, Theme, Bewegungsmelder

### Themes als Datenpakete

Ein Theme ist **kein Code**, sondern ein Paket: Hintergrundbild(er) in Zielauflösung, vorgerenderte Schrift,
Farben (an/aus/Eckpunkte), Effekt-Parameter, Layout (Ränder, Raster, eigene Buchstabenraster). Der Hub schickt
es, die Uhr zeigt es ohne neue Firmware an. Nur neue Effekt-Arten brauchen ein Firmware-Update.

Jedes Gerät meldet seine Fähigkeiten (Farbe/Graustufen, Effekte ja/nein, Taktung); Themes bekommen bei Bedarf
eine E-Ink-Fassung (Typografie, Muster, Graustufen-Raster). Die Themes aus der Web-App sind die Vorlage
(Verläufe und Szenen als Hintergrundbilder exportieren).

### Inhalte per API

| Art | ESP32-P4 | Hinweis |
|---|---|---|
| Text | problemlos | Stil, Dauer, Priorität |
| Bild | gut | JPEG laut Waveshare per Hardware; der Hub skaliert vorher |
| Animation, kurzes Video | eingeschränkt | Hub wandelt in Motion-JPEG; Bildrate testen |
| Echtes Video, Streams, Kamera live | eher nicht | Linux-Klasse; Standbild der Türkamera geht |

Der Hub wandelt alles in ein gerätefertiges Format um, die Geräte bleiben einfach. Adressierung wie bei Quak
(Raum, Person, Gruppe).

### Zeit und Synchronität

Jede Uhr gleicht ihre Zeit per NTP ab (im Heimnetz typischerweise wenige Millisekunden genau). Termine
(Erinnerungen, Themenwechsel) werden vorab verteilt und zur exakten Uhrzeit lokal ausgelöst – dadurch
passiert es auf allen Geräten gleichzeitig, auch synchron zur Quak-Ansage.

### Updates

ESP-IDF-OTA mit zwei Firmware-Partitionen und automatischem Rollback, verteilt vom Hub.

## Gehäuse und Bedienung

- 3D-Druck, Rahmen mit quadratischem Ausschnitt, optional getönte Frontscheibe.
- Zwei bis drei Knöpfe, z. B. Theme weiter · Info (kurz Datum, lang Sekunden) · Helligkeit bzw. Erinnerung
  quittieren.
- Bewegungsmelder: PIR (z. B. AM312) erkennt Bewegung; mmWave-Radar (z. B. LD2410) erkennt auch Menschen, die
  still sitzen oder schlafen – fürs Kinderzimmer besser.

## Strom

Display + Prozessor brauchen dauerhaft etwa 2–6 W (50–150 Wh pro Tag) → Akku nicht sinnvoll, außer E-Ink.
Optionen: dünnes USB-C-Kabel im Kabelkanal, Unterputz-Steckdose bzw. USB-Dose hinter der Uhr (Elektriker),
oder Platz mit vorhandenem Strom. Nachts dimmen bzw. aus spart zusätzlich.

## Das Original: Hersteller, Preise, Rechte

**Kaufen**: direkt im Shop auf qlocktwo.com (Warenkorb für EARTH und W), über Händler wie Wempe und in
Flagship-Stores. MOON nur auf Anfrage, EARTH 180 mit ca. 6 Monaten Lieferzeit. Die alten Namen Classic,
Large und Touch heißen heute EARTH; die Zahl ist die Kantenlänge in cm.

| Modell | Größe | Variante | Preis (qlocktwo.com, Oktober 2026) |
|---|---|---|---|
| EARTH 13.5 | Tischuhr | Full Metal / Rust / Silver & Gold | 760 € / 1.260 € / 1.390 € |
| EARTH 45 | 45 × 45 cm | Black Pepper / Stainless Steel | 1.600 € / 1.785 € |
| EARTH 45 | 45 × 45 cm | Creator's Edition Desert / Rust / Platinum | ab 2.690 € / 2.780 € / 4.180 € |
| EARTH 90 | 90 × 90 cm | Black Pepper / Rust / Platinum | 6.950 € / 10.300 € / 13.500 € |
| EARTH 180 | 180 × 180 cm | diverse | ohne Preis |
| MOON 90 / 180 | | Dawn, Midnight, Nightfall | auf Anfrage |
| W (Armbanduhr) | | Black Steel, Fine Steel | 1.200 € |

**Digital**: nur die Begleit-App Flashsetter (Uhrzeit bzw. Mondphase per Handy übertragen). Kein offizieller
Bildschirmschoner, keine TV-App gefunden.

**Rechte**:

- Die Marken (QLOCKTWO, Wortfolgen wie ITLISASTIME) hält die QLOCKTWO License GmbH in Schwäbisch Gmünd,
  angemeldet u. a. für Uhren und für Software/Computerspiele. Uhren und Shop betreibt die QLOCKTWO
  Manufacture GmbH (Geschäftsführer Marco Biegert). Dazu US-Designpatente auf „Matrix clock".
- Belegt ist ein exklusiver Franchise-Vertrieb in Israel. Lizenzen an Dritte für Produkte haben wir nicht
  gefunden – die frühere Annahme „die lizenzieren schon" war zu weit gegriffen.
- Wortuhren an sich sind nicht geschützt. Privat unproblematisch; bei Veröffentlichung eigener Name und
  eigene Raster (nicht „QLOCK"). Kooperation anfragen wäre möglich, am besten mit fertiger Demo. Keine
  Rechtsberatung.

## Offene Fragen

- Welche Größen bleiben nach dem Papiertest übrig?
- Lohnt E-Ink mit Akku für Küche/Flur, oder doch überall LCD mit Kabel?
- OLED-Wanduhr: welches Linux-Board, welcher Monitor, Kosten?
- Rust auf dem ESP32-P4: Wie weit ist die Unterstützung wirklich? (Durchstich klärt das.)
- Wo läuft der Elixir-Hub (eigener Rechner, vorhandener Server, NAS)?
- Wie hängt der Hub mit Quak zusammen – eigener Dienst oder Display als zweiter Ausgabekanal in Quak?

## Nächste Schritte

1. Papiermuster aufhängen und Größen festlegen.
2. Testhardware bestellen: ein ESP32-P4 mit 4"-LCD (720×720) und ein 7,5"-E-Ink-Display.
3. Durchstich auf dem P4: Rust-Firmware bootet und zeigt ein Bild → NTP, Uhrenlogik, ein Theme aus festem
   Paket → Überblendung und ein Effekt, dabei Bildrate und Stromverbrauch messen → Theme per WLAN vom Hub.
4. Danach Hub (Elixir) mit Profilen, Tagesplan und Quak-Kopplung.

## Quellen

- [Guition ESP32-S3-4848S040 (atomic14)](https://www.atomic14.com/esp32/boards/guition-4848s040/)
- [Makerfabs ESP32-S3 Parallel TFT 4" 480×480 (Tindie)](https://www.tindie.com/products/makerfabs/esp32-s3-parallel-tft-with-touch-40-480480)
- [Waveshare ESP32-P4 Boards](https://www.waveshare.com/product/esp32-related/boards-kits/esp32-p4.htm)
- [Waveshare MIPI-Displays](https://www.waveshare.com/product/esp32-related/displays/lcd-mipi.htm?mode=list)
- [Waveshare ESP32-P4-WIFI6 Wiki](https://www.waveshare.com/wiki/ESP32-P4-WIFI6)
- [Waveshare ESP32-P4 Smart 86 Box, 4" 720×720 (Abra)](https://abra-electronics.com/robotics-embedded-electronics/esp-series/wave-31570-esp32-p4-smart-86-box-development-board-based-on-esp32-p4-controller-with-risc-v-32-bit-dual-core-and-single-core-processors-onboard-4inch-720-720-pixel-touch-display-5-point-touch.html)
- [Waveshare ESP32-P4 7" Board (Abra)](https://abra-electronics.com/opto-illumination/lcds-and-displays/lcd-oled-displays/5-15.6/wave-32511-esp32-p4-wifi6-7inch-5-point-touch-display-development-board-1024-600-resolution-ov5647-camera.html)
- [Guition JC1060P470, 7" ESP32-P4 (atomic14)](https://www.atomic14.com/esp32/boards/guition-jc1060p470/)
- [Espressif ESP32-P4 (Mouser)](https://www.mouser.co.za/new/espressif/espressif-esp32-p4-socs/)
- [Waveshare ESP32 AMOLED-Übersicht](https://www.waveshare.com/product/esp32-related/displays/amoled.htm?mode=list)
- [Waveshare 7,5" E-Paper](https://www.waveshare.com/7.5inch-e-Paper.htm)
- [Waveshare 7,5" E-Paper (B)](https://www.waveshare.com/7.5inch-e-paper-b.htm)
- [Waveshare 13,3" E-Paper (K)](https://www.waveshare.com/product/13.3inch-e-paper-k.htm)
- [Waveshare E-Paper mit ESP32](https://docs.waveshare.com/ESP32-Peripheral-Tutorials/Display/E-Paper)
- [esp-idf-svc](https://github.com/ivmarkov/esp-idf-svc)
- [esp32p4 PAC (esp-rs)](https://docs.rs/esp32p4/latest/settings.html)
- [Slint Preise und Lizenz](https://slint.dev/pricing)
- [ESPHome HUB75](https://esphome.io/components/display/hub75.html)
- [Flashsetter (Biegert & Funk) im App Store](https://apps.apple.com/lu/app/flashsetter/id1110375389)
- [QLOCKTWO License GmbH (Markenregister)](https://gleanmark.com/owner/qlocktwolicensegmbh)
- [QLOCKTWO License GmbH (Markinton)](https://markinton.com/company/qlocktwo-license-gmbh)
- [qlocktwo.com: EARTH](https://qlocktwo.com/de-de/earth), [W](https://qlocktwo.com/de-de/w),
  [Impressum](https://qlocktwo.com/de-de/imprint)
- [Franchise Israel (Jerusalem Post)](https://stgmobile.jpost.com/brandblend/award-winning-watch-brand-qlocktwo-enters-the-israeli-market-824220)
- [Biegert & Funk Product GmbH & Co. KG (Patente)](https://trea.com/organization/biegert--funk-product-gmbh--co-kg/b297cdfc-951d-457e-acd4-27274e3e2928)
