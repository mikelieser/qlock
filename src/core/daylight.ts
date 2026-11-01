/** Theme by the sun ("Tageslauf"). Mirrored in native/core (daylight). */

/** Home for now: Koblenz. Later the hub sends the place. */
export const HOME = { lat: 50.36, lon: 7.59 };

export type Daylight = 'nightsurf' | 'sunrise' | 'lagoon' | 'sunset';

/** Sun elevation in degrees at `date` for a place (NOAA approximation, good to a fraction of a degree). */
export function sunElevation(date: Date, lat = HOME.lat, lon = HOME.lon): number {
  const rad = Math.PI / 180;
  const start = Date.UTC(date.getUTCFullYear(), 0, 1);
  const dayOfYear = Math.floor((date.getTime() - start) / 86_400_000) + 1;
  const hours = date.getUTCHours() + date.getUTCMinutes() / 60 + date.getUTCSeconds() / 3600;
  const g = ((2 * Math.PI) / 365) * (dayOfYear - 1 + (hours - 12) / 24);
  const eqTime =
    229.18 * (0.000075 + 0.001868 * Math.cos(g) - 0.032077 * Math.sin(g) - 0.014615 * Math.cos(2 * g) - 0.040849 * Math.sin(2 * g));
  const decl =
    0.006918 - 0.399912 * Math.cos(g) + 0.070257 * Math.sin(g) - 0.006758 * Math.cos(2 * g) + 0.000907 * Math.sin(2 * g) -
    0.002697 * Math.cos(3 * g) + 0.00148 * Math.sin(3 * g);
  const solarMinutes = hours * 60 + eqTime + 4 * lon;
  const hourAngle = (solarMinutes / 4 - 180) * rad;
  const cosZenith = Math.sin(lat * rad) * Math.sin(decl) + Math.cos(lat * rad) * Math.cos(decl) * Math.cos(hourAngle);
  return 90 - Math.acos(Math.min(1, Math.max(-1, cosZenith))) / rad;
}

/**
 * Night below −6° (civil twilight), sunrise and sunset theme while the sun is low (below 8°) in the
 * morning or the evening, the lagoon in between.
 */
export function daylight(date: Date, lat = HOME.lat, lon = HOME.lon): Daylight {
  const elevation = sunElevation(date, lat, lon);
  if (elevation < -6) return 'nightsurf';
  if (elevation >= 8) return 'lagoon';
  // Morning or evening: is the sun rising? Compare with ten minutes later.
  const later = sunElevation(new Date(date.getTime() + 600_000), lat, lon);
  return later > elevation ? 'sunrise' : 'sunset';
}
