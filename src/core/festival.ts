/** Festive themes by date, for the "Feste automatisch" setting. Mirrored in native/core (festival). */

export type Festival = 'karneval' | 'ostern' | 'halloween' | 'weihnachten' | 'silvester';

/** Easter Sunday (Gregorian, anonymous algorithm) as [month 1–12, day]. */
export function easter(year: number): [number, number] {
  const a = year % 19;
  const b = Math.floor(year / 100);
  const c = year % 100;
  const d = Math.floor(b / 4);
  const e = b % 4;
  const f = Math.floor((b + 8) / 25);
  const g = Math.floor((b - f + 1) / 3);
  const h = (19 * a + b - d - g + 15) % 30;
  const i = Math.floor(c / 4);
  const k = c % 4;
  const l = (32 + 2 * e + 2 * i - h - k) % 7;
  const m = Math.floor((a + 11 * h + 22 * l) / 451);
  const month = Math.floor((h + l - 7 * m + 114) / 31);
  const day = ((h + l - 7 * m + 114) % 31) + 1;
  return [month, day];
}

const DAY = 86_400_000;

/**
 * The festival a date belongs to, if any:
 * Karneval Weiberfastnacht bis Veilchendienstag, Ostern Palmsonntag bis Ostermontag,
 * Halloween 24.–31. Oktober, Weihnachten 1. Advent bis 26. Dezember, Silvester 27.–31. Dezember.
 */
export function festival(date: Date): Festival | undefined {
  const year = date.getFullYear();
  const today = Date.UTC(year, date.getMonth(), date.getDate());
  const [em, ed] = easter(year);
  const easterDay = Date.UTC(year, em - 1, ed);
  const offset = Math.round((today - easterDay) / DAY);
  const month = date.getMonth() + 1;
  const day = date.getDate();
  // First Advent: four Sundays before Christmas, between 27 November and 3 December.
  // Fourth Advent is the Sunday on or before 24 December, the first three weeks earlier.
  const advent = Date.UTC(year, 11, 24) - (new Date(Date.UTC(year, 11, 24)).getUTCDay() + 21) * DAY;

  if (offset >= -52 && offset <= -47) return 'karneval';
  if (offset >= -7 && offset <= 1) return 'ostern';
  if (month === 10 && day >= 24) return 'halloween';
  if (month === 12 && day >= 27) return 'silvester';
  if (today >= advent && (month < 12 || day <= 26)) return 'weihnachten';
  return undefined;
}
