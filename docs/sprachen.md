# Sprachen und Dialekte

Stand: 31.10.2026. Welche Sprachen gut auf eine Wortuhr passen und warum. Was davon umgesetzt ist, steht im
[README](../README.md#sprachen).

## Woran es hängt

1. Die Uhrzeit lässt sich mit wenigen festen Wörtern in immer gleicher Reihenfolge sagen.
2. Die Wörter ändern ihre Form nicht je nach Stunde (keine Fälle, keine Endungen).
3. Die Schrift lässt sich in einzelne Zellen zerlegen: keine verbundenen Buchstaben, am besten links nach rechts.
4. Die Wörter sind kurz genug für etwa 11 Spalten.

Ein Test prüft für jede Sprache alle 1440 Minuten des Tages: Jede Minute hat einen Satz, und die Wörter
leuchten in Leserichtung.

## Zwei Zählweisen im Deutschen

- **Viertel zur nächsten Stunde**: „viertel vier" = 3:15, „dreiviertel vier" = 3:45. Schwäbisch, Badisch,
  Bayerisch, Fränkisch, Wienerisch, Sächsisch, Berlinerisch. Umgesetzt mit `quarterMinutes` bzw. `quarter`.
- **Viertel nach / vor**: Hochdeutsch, Kölsch, Koblenz, Hessisch, Pfälzisch, Saarländisch, Ruhrpott,
  Hamburger Platt, Ostfriesisch. Schweizerdeutsch sagt „ab" statt „nach".

Koblenz sagt um 2:20 „zehn vor halber drei" (wie Niederländisch und älteres Rheinländisch), das spart das
lange ZWANZISCH.

## Umgesetzt

Deutsch, Formrausch, 16 Dialekte, Englisch, Niederländisch, Dänisch, Schwedisch, Französisch, Spanisch,
Italienisch, Griechisch, Japanisch.

Besonderheiten:

- **Französisch**: ab :35 zur nächsten Stunde („TROIS HEURES MOINS VINGT-CINQ"), MIDI und MINUIT ohne
  HEURES, „UNE HEURE" in der Einzahl.
- **Spanisch / Italienisch**: 1 Uhr in der Einzahl („ES LA UNA", „È L'UNA"), sonst Mehrzahl.
- **Schwedisch / Dänisch**: halbe Stunde zur nächsten wie im Deutschen („fem i halv tre" = 2:25).
- **Niederländisch**: ab :20 zur halben Stunde („tien voor half drie"), UUR nur zur vollen Stunde.
- **Griechisch**: Großbuchstaben ohne Akzente, 1, 3 und 4 in weiblicher Form (ΜΙΑ, ΤΡΕΙΣ, ΤΕΣΣΕΡΙΣ).
- **Japanisch**: 6×6 Kanji. Zahlen aus Einzelziffern (十 + 一 = 11, 二 + 十 + 五 = 25), 午前/午後 für
  vor- und nachmittags, 半 für halb. Füllzeile mit Wochentags-Kanji. Titel ohne Leerzeichen.

Die Dialekte sind nicht von Muttersprachlern geprüft, am unsichersten Koblenz, Pfalz und Ostfriesland.

## Weitere Kandidaten

| Gut | Mit Aufwand | Schlecht |
|---|---|---|
| Norwegisch (wie Schwedisch) | Finnisch (gebeugte Minuten, lange Wörter) | Arabisch, Persisch, Urdu (verbundene Schrift, Formen je nach Position) |
| Ungarisch (NEGYED HÁROM = 2:15) | Katalanisch (eigene Viertel-Logik) | Thai (keine Leerzeichen, Sechs-Stunden-Blöcke mit eigenen Stundenwörtern) |
| Portugiesisch | Polnisch, Tschechisch, Russisch (Fallformen je Stunde) | sehr lange Zahlwörter mit Fällen (z. B. Isländisch) |
| Vietnamesisch | Türkisch (Endungen je nach Zahl) | |
| Chinesisch (wie Japanisch) | Hebräisch (rechts nach links) | |
| Koreanisch (Hangul-Blöcke) | Hindi (Silbenzellen, सवा/ढाई/पौने) | |

Für die echte QLOCKTWO nennen Händler über 20 Sprachen; Französisch, Spanisch, Schwedisch, Arabisch,
Chinesisch, Katalanisch und Schwäbisch sind dort belegt.
