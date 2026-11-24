// The visitor's clock: keep its offset from UTC in the `tz` cookie, in
// minutes east (Vietnam is 420), then reload once so the server labels the
// day plan in local time. Without the cookie, or with cookies blocked, the
// page keeps Vietnam time.

const TZ_COOKIE = "tz";
const TZ_MAX_AGE = 60 * 60 * 24 * 365;
const offset = String(-new Date().getTimezoneOffset());
const saved = document.cookie
  .split("; ")
  .find((cookie) => cookie.startsWith(`${TZ_COOKIE}=`));

if (saved !== `${TZ_COOKIE}=${offset}`) {
  document.cookie = `${TZ_COOKIE}=${offset}; path=/; max-age=${TZ_MAX_AGE}; samesite=lax`;
  // One reload per zone and session: a page that cannot keep cookies must
  // not reload forever.
  const guard = `thienhac.tz.${offset}`;
  let first = false;
  try {
    first = sessionStorage.getItem(guard) === null;
    sessionStorage.setItem(guard, "1");
  } catch {
    // No session store: keep the cookie, but do not risk a reload loop.
  }
  if (first) location.reload();
}
