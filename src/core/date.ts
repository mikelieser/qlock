/** Short weekday and date for the caption, e.g. "Sa · 31. Oktober" or "Sat · 31 October". */
export function formatDate(date: Date, lang: string): string {
  // German short weekdays come with a dot ("Sa."), which looks odd in the spaced-out caps.
  const weekday = new Intl.DateTimeFormat(lang, { weekday: 'short' }).format(date).replace(/\.$/, '');
  const day = new Intl.DateTimeFormat(lang, { day: 'numeric', month: 'long' }).format(date);
  return `${weekday} · ${day}`;
}
