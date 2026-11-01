import { describe, expect, it } from 'vitest';
import { frameText } from '../core/locale';
import { da } from './da';
import { de } from './de';
import { bavarian, berlin, cologne, hamburgPlatt, hessian, swabian, swissGerman } from './dialects';
import { badish, eastFrisian, franconian, koblenz, palatinate, ruhr, saarland, saxon, viennese } from './dialects-more';
import { el } from './el';
import { en } from './en';
import { es } from './es';
import { formrauschLocale } from './formrausch';
import { fr } from './fr';
import { it as italiano } from './it';
import { ja } from './ja';
import { nl } from './nl';
import { sv } from './sv';
import type { Locale } from '../core/locale';
import { LOCALES } from '.';
import { boerse, derb, kids, nerd, zen } from './fun';

const at = (time: string, seconds = 0) => {
  const [h, m] = time.split(':').map(Number);
  return new Date(2026, 9, 31, h, m, seconds);
};

const say = (locale: Locale, time: string, intro = true) =>
  frameText(locale.grid, locale.frame(at(time), { intro }));

describe('de', () => {
  it.each([
    ['14:00', 'ES IST ZWEI UHR'],
    ['14:04', 'ES IST ZWEI UHR'],
    ['14:05', 'ES IST FÜNF NACH ZWEI'],
    ['14:10', 'ES IST ZEHN NACH ZWEI'],
    ['14:15', 'ES IST VIERTEL NACH ZWEI'],
    ['14:20', 'ES IST ZWANZIG NACH ZWEI'],
    ['14:25', 'ES IST FÜNF VOR HALB DREI'],
    ['14:30', 'ES IST HALB DREI'],
    ['14:35', 'ES IST FÜNF NACH HALB DREI'],
    ['14:40', 'ES IST ZWANZIG VOR DREI'],
    ['14:45', 'ES IST VIERTEL VOR DREI'],
    ['14:50', 'ES IST ZEHN VOR DREI'],
    ['14:59', 'ES IST FÜNF VOR DREI'],
    ['01:00', 'ES IST EIN UHR'],
    ['13:15', 'ES IST VIERTEL NACH EINS'],
    ['00:00', 'ES IST ZWÖLF UHR'],
    ['12:30', 'ES IST HALB EINS'],
    ['23:45', 'ES IST VIERTEL VOR ZWÖLF'],
    ['10:00', 'ES IST ZEHN UHR'],
    ['09:00', 'ES IST NEUN UHR'],
  ])('%s → %s', (time, text) => expect(say(de, time)).toBe(text));

  it('drops the intro', () => expect(say(de, '14:30', false)).toBe('HALB DREI'));

  it('lights one corner dot per minute past the five', () => {
    expect([0, 1, 2, 3, 4, 5].map((m) => de.frame(at(`14:${m}`), { intro: true }).dots)).toEqual([0, 1, 2, 3, 4, 0]);
  });
});

describe('en', () => {
  it.each([
    ['14:00', 'IT IS TWO OCLOCK'],
    ['14:15', 'IT IS A QUARTER PAST TWO'],
    ['14:25', 'IT IS TWENTY FIVE PAST TWO'],
    ['14:30', 'IT IS HALF PAST TWO'],
    ['14:35', 'IT IS TWENTY FIVE TO THREE'],
    ['14:45', 'IT IS A QUARTER TO THREE'],
    ['23:55', 'IT IS FIVE TO TWELVE'],
    ['11:00', 'IT IS ELEVEN OCLOCK'],
  ])('%s → %s', (time, text) => expect(say(en, time)).toBe(text));
});

describe('formrausch', () => {
  it('tells the time like de', () => expect(say(formrauschLocale, '14:25')).toBe('ES IST FÜNF VøR HALB DREI'));

  it('spells FEIERABEND and FøRMRAUSCH at six', () => {
    const grid = formrauschLocale.grid;
    const letters = (time: string) =>
      formrauschLocale
        .frame(at(time), { intro: true })
        .words.at(-1)!
        .map((c) => grid.chars[c])
        .join('');
    expect(letters('18:00')).toBe('FEIERABEND');
    expect(letters('18:01')).toBe('FøRMRAUSCH');
  });

  it('shows breakfast on odd minutes before noon', () => {
    expect(formrauschLocale.frame(at('11:15'), { intro: true }).title).toBe('FRÜHSTÜCKSZEIT ;)');
    expect(formrauschLocale.frame(at('11:16'), { intro: true }).title).toBeUndefined();
  });
});

describe('fr', () => {
  it.each([
    ['14:00', 'IL EST DEUX HEURES'],
    ['14:05', 'IL EST DEUX HEURES CINQ'],
    ['14:15', 'IL EST DEUX HEURES ET QUART'],
    ['14:25', 'IL EST DEUX HEURES VINGT-CINQ'],
    ['14:30', 'IL EST DEUX HEURES ET DEMIE'],
    ['14:35', 'IL EST TROIS HEURES MOINS VINGT-CINQ'],
    ['14:45', 'IL EST TROIS HEURES MOINS LE QUART'],
    ['14:55', 'IL EST TROIS HEURES MOINS CINQ'],
    ['01:10', 'IL EST UNE HEURE DIX'],
    ['12:00', 'IL EST MIDI'],
    ['12:30', 'IL EST MIDI ET DEMIE'],
    ['11:45', 'IL EST MIDI MOINS LE QUART'],
    ['00:20', 'IL EST MINUIT VINGT'],
    ['23:50', 'IL EST MINUIT MOINS DIX'],
    ['22:00', 'IL EST DIX HEURES'],
  ])('%s → %s', (time, text) => expect(say(fr, time)).toBe(text));
});

describe('es', () => {
  it.each([
    ['14:00', 'SON LAS DOS'],
    ['14:05', 'SON LAS DOS Y CINCO'],
    ['14:15', 'SON LAS DOS Y CUARTO'],
    ['14:25', 'SON LAS DOS Y VEINTICINCO'],
    ['14:30', 'SON LAS DOS Y MEDIA'],
    ['14:35', 'SON LAS TRES MENOS VEINTICINCO'],
    ['14:45', 'SON LAS TRES MENOS CUARTO'],
    ['01:00', 'ES LA UNA'],
    ['12:40', 'ES LA UNA MENOS VEINTE'],
    ['00:10', 'SON LAS DOCE Y DIEZ'],
    ['22:50', 'SON LAS ONCE MENOS DIEZ'],
  ])('%s → %s', (time, text) => expect(say(es, time)).toBe(text));
});

describe('it', () => {
  it.each([
    ['14:00', 'SONO LE DUE'],
    ['14:05', 'SONO LE DUE E CINQUE'],
    ['14:15', 'SONO LE DUE E UN QUARTO'],
    ['14:25', 'SONO LE DUE E VENTICINQUE'],
    ['14:30', 'SONO LE DUE E MEZZA'],
    ['14:35', 'SONO LE TRE MENO VENTICINQUE'],
    ['14:45', 'SONO LE TRE MENO UN QUARTO'],
    ['01:00', "È L' UNA"],
    ['12:50', "È L' UNA MENO DIECI"],
    ['00:00', 'SONO LE DODICI'],
    ['09:10', 'SONO LE NOVE E DIECI'],
  ])('%s → %s', (time, text) => expect(say(italiano, time)).toBe(text));
});

describe('sv', () => {
  it.each([
    ['14:00', 'KLOCKAN ÄR TVÅ'],
    ['14:05', 'KLOCKAN ÄR FEM ÖVER TVÅ'],
    ['14:15', 'KLOCKAN ÄR KVART ÖVER TVÅ'],
    ['14:20', 'KLOCKAN ÄR TJUGO ÖVER TVÅ'],
    ['14:25', 'KLOCKAN ÄR FEM I HALV TRE'],
    ['14:30', 'KLOCKAN ÄR HALV TRE'],
    ['14:35', 'KLOCKAN ÄR FEM ÖVER HALV TRE'],
    ['14:40', 'KLOCKAN ÄR TJUGO I TRE'],
    ['14:45', 'KLOCKAN ÄR KVART I TRE'],
    ['14:55', 'KLOCKAN ÄR FEM I TRE'],
    ['00:00', 'KLOCKAN ÄR TOLV'],
    ['11:30', 'KLOCKAN ÄR HALV TOLV'],
  ])('%s → %s', (time, text) => expect(say(sv, time)).toBe(text));
});

describe('ja', () => {
  it.each([
    ['14:00', '今は午後二時です'],
    ['14:05', '今は午後二時五分です'],
    ['14:15', '今は午後二時十五分です'],
    ['14:25', '今は午後二時二十五分です'],
    ['14:30', '今は午後二時半です'],
    ['14:55', '今は午後二時五十五分です'],
    ['09:40', '今は午前九時四十分です'],
    ['11:10', '今は午前十一時十分です'],
    ['00:35', '今は午前十二時三十五分です'],
    ['12:00', '今は午後十二時です'],
  ])('%s → %s', (time, text) => expect(say(ja, time)).toBe(text));

  it('drops the intro', () => expect(say(ja, '22:20', false)).toBe('午後十時二十分です'));
});

describe('nl', () => {
  it.each([
    ['14:00', 'HET IS TWEE UUR'],
    ['14:15', 'HET IS KWART OVER TWEE'],
    ['14:20', 'HET IS TIEN VOOR HALF DRIE'],
    ['14:25', 'HET IS VIJF VOOR HALF DRIE'],
    ['14:30', 'HET IS HALF DRIE'],
    ['14:40', 'HET IS TIEN OVER HALF DRIE'],
    ['14:45', 'HET IS KWART VOOR DRIE'],
    ['00:55', 'HET IS VIJF VOOR EEN'],
    ['10:00', 'HET IS TIEN UUR'],
  ])('%s → %s', (time, text) => expect(say(nl, time)).toBe(text));
});

describe('da', () => {
  it.each([
    ['14:00', 'KLOKKEN ER TO'],
    ['14:05', 'KLOKKEN ER FEM OVER TO'],
    ['14:20', 'KLOKKEN ER TYVE OVER TO'],
    ['14:25', 'KLOKKEN ER FEM I HALV TRE'],
    ['14:30', 'KLOKKEN ER HALV TRE'],
    ['14:35', 'KLOKKEN ER FEM OVER HALV TRE'],
    ['14:45', 'KLOKKEN ER KVART I TRE'],
    ['10:50', 'KLOKKEN ER TI I ELLEVE'],
  ])('%s → %s', (time, text) => expect(say(da, time)).toBe(text));
});

describe('el', () => {
  it.each([
    ['14:00', 'ΕΙΝΑΙ ΔΥΟ'],
    ['14:15', 'ΕΙΝΑΙ ΔΥΟ ΚΑΙ ΤΕΤΑΡΤΟ'],
    ['14:25', 'ΕΙΝΑΙ ΔΥΟ ΚΑΙ ΕΙΚΟΣΙΠΕΝΤΕ'],
    ['14:30', 'ΕΙΝΑΙ ΔΥΟ ΚΑΙ ΜΙΣΗ'],
    ['14:35', 'ΕΙΝΑΙ ΤΡΕΙΣ ΠΑΡΑ ΕΙΚΟΣΙΠΕΝΤΕ'],
    ['14:45', 'ΕΙΝΑΙ ΤΡΕΙΣ ΠΑΡΑ ΤΕΤΑΡΤΟ'],
    ['00:50', 'ΕΙΝΑΙ ΜΙΑ ΠΑΡΑ ΔΕΚΑ'],
    ['10:10', 'ΕΙΝΑΙ ΔΕΚΑ ΚΑΙ ΔΕΚΑ'],
  ])('%s → %s', (time, text) => expect(say(el, time)).toBe(text));
});

describe('dialects', () => {
  it.each([
    [swabian, '14:15', 'ES ISCH VIERTEL DREI'],
    [swabian, '14:45', 'ES ISCH DREIVIERTEL DREI'],
    [swabian, '14:25', 'ES ISCH FÜNF VOR HALB DREI'],
    [swabian, '00:30', 'ES ISCH HALB OINS'],
    [swabian, '06:40', 'ES ISCH ZEHN NACH HALB SIEBA'],
    [bavarian, '14:15', 'ES IS VIERTL DREI'],
    [bavarian, '14:30', 'ES IS HOIBE DREI'],
    [bavarian, '11:45', 'ES IS DREIVIERTL ZWÄIF'],
    [berlin, '14:15', 'ET IS VIERTEL DREI WA'],
    [berlin, '01:00', 'ET IS EENS WA'],
    [swissGerman, '14:15', 'ES ISCH VIERTEL AB ZWEI'],
    [swissGerman, '14:25', 'ES ISCH FÜF VOR HALBI DRÜÜ'],
    [swissGerman, '14:40', 'ES ISCH ZWÄNZG VOR DRÜÜ'],
    [swissGerman, '15:30', 'ES ISCH HALBI VIERI'],
    [hamburgPlatt, '14:15', 'KLOCK IS VEERDEL NA TWEE'],
    [hamburgPlatt, '14:30', 'KLOCK IS HALF DREE'],
    [hamburgPlatt, '10:45', 'KLOCK IS VEERDEL VÖR ÖLVEN'],
    [hessian, '14:20', 'ES IS ZWANZISCH NACH ZWAA GELL'],
    [hessian, '14:35', 'ES IS FINF NACH HALB DREI GELL'],
    [cologne, '14:15', 'ET ES VEEDEL NOH ZWEI'],
    [cologne, '14:25', 'ET ES FÜNNEF VÖR HALV DREI'],
    [cologne, '14:30', 'ET ES HALV DREI'],
    [cologne, '10:45', 'ET ES VEEDEL VÖR ELLEF'],
    [cologne, '08:40', 'ET ES ZWANZICH VÖR NÜNG'],
    [saxon, '14:15', 'ES IS VIERDL DREIE'],
    [saxon, '14:45', 'ES IS DREIVIERDL DREIE'],
    [saxon, '00:30', 'ES IS HALB EENS'],
    [viennese, '14:15', 'ES IS VIERTL DREI'],
    [viennese, '10:30', 'ES IS HOIB ÖF'],
    [franconian, '14:45', 'ES IS DREIVIERDL DREI'],
    [franconian, '06:30', 'ES IS HALB SIEM'],
    [badish, '14:15', 'ES ISCH VIERTEL DRÜ'],
    [badish, '14:05', 'ES ISCH FÜNF NOCH ZWEI'],
    [palatinate, '14:20', 'ES IS ZWANZISCH NOCH ZWÄÄ'],
    [palatinate, '14:45', 'ES IS VIERTEL VOR DREI'],
    [saarland, '14:15', 'ES IS VIERTEL NOH ZWOO'],
    [saarland, '07:55', 'ES IS FÜNNEF VOR AACHT'],
    [ruhr, '14:40', 'ET IS ZWANZICH VOR DREI'],
    [ruhr, '14:30', 'ET IS HALB DREI'],
    [eastFrisian, '14:15', 'DAT IS KWART NA TWEE'],
    [eastFrisian, '11:45', 'DAT IS KWART VÖR TWALF'],
    [koblenz, '14:15', 'ET ES VEERTEL NOH ZWEE'],
    [koblenz, '14:20', 'ET ES ZÄHN VÜR HALWER DREI'],
    [koblenz, '14:40', 'ET ES ZÄHN NOH HALWER DREI'],
    [koblenz, '14:45', 'ET ES VEERTEL VÜR DREI'],
  ])('%#: $1 → $2', (locale, time, text) => expect(say(locale, time)).toBe(text));

  it('says Berlin WA only with the intro', () => expect(say(berlin, '14:30', false)).toBe('HALB DREI'));
});

describe('all locales', () => {
  it.each(LOCALES)('$id has a phrase for every minute of the day', (locale) => {
    for (let minute = 0; minute < 1440; minute++) {
      const date = new Date(2026, 9, 31, 0, minute);
      const frame = locale.frame(date, { intro: true });
      expect(frame.words.every((w) => w.length > 0)).toBe(true);
      // Words must light up in reading order, otherwise the sentence reads jumbled on the grid.
      const cells = frame.words.flat();
      if (!frame.title?.includes(';)')) expect(cells).toEqual([...cells].sort((a, b) => a - b));
    }
  });
});

describe('fun faces', () => {
  it('kids', () => expect(say(kids, '15:15')).toBe('GUCK MAL ES IST VIERTEL NACH DREI JUHU'));
  it('banker', () => expect(say(boerse, '14:20')).toBe('ZEIT IST GELD ZEHN VOR HALB DREI BULLISH'));
  it('nerd', () => expect(say(nerd, '13:45')).toBe('SUDO ES IST VIERTEL VOR ZWEI EOF'));
  it('zen', () => expect(say(zen, '9:00')).toBe('JETZT IST NEUN OM'));
  it('rude', () => expect(say(derb, '23:55')).toBe('SCHEISSE ES IST SCHON FÜNF VOR ZWÖLF ALTER'));
  it('without intro and outro', () => expect(say(derb, '23:55', false)).toBe('FÜNF VOR ZWÖLF'));
});
