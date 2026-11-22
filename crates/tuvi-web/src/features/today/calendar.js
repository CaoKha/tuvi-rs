// The calendar of the roadmap steps between months. The server renders the
// month of the day shown; this script redraws the page of days for
// the other months, with the same links and marks. The lunar calendar of
// the engine covers 1800–2199.

const FIRST_YEAR = 1801;
const LAST_YEAR = 2198;
const DAY_MS = 86_400_000;

const calendar = document.querySelector("[data-calendar]");
if (calendar) {
  const grid = calendar.querySelector("[data-cal-grid]");
  const title = calendar.querySelector("[data-cal-title]");
  const { path, today, shown } = calendar.dataset;
  let year = Number(calendar.dataset.year);
  let month = Number(calendar.dataset.month);

  // Day counts from 1970-01-01, in UTC so no time zone shifts a day.
  const count = (y, m, d) => Math.round(Date.UTC(y, m - 1, d) / DAY_MS);
  const iso = (n) => new Date(n * DAY_MS).toISOString().slice(0, 10);
  const parse = (text) => {
    const [y, m, d] = text.split("-").map(Number);
    return count(y, m, d);
  };
  const todayN = parse(today);
  const shownN = parse(shown);
  const months = new Intl.DateTimeFormat(document.documentElement.lang || "en", {
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  });

  const draw = () => {
    const start = count(year, month, 1);
    const length = count(year, month + 1, 1) - start;
    // 1970-01-01 was a Thursday, 3 days after Monday.
    const offset = (((start + 3) % 7) + 7) % 7;
    const cells = [];
    for (let i = 0; i < offset; i += 1) cells.push("<span></span>");
    for (let day = 1; day <= length; day += 1) {
      const n = start + day - 1;
      const marks = [n === todayN && "is-today", n === shownN && "is-shown"].filter(Boolean);
      const current = n === shownN ? ' aria-current="date"' : "";
      cells.push(
        `<a href="${path}?day=${iso(n)}" class="cal-day ${marks.join(" ")}"${current}><span>${day}</span></a>`,
      );
    }
    grid.innerHTML = cells.join("");
    const label = months.format(new Date(start * DAY_MS));
    title.textContent = label.charAt(0).toUpperCase() + label.slice(1);
  };

  calendar.addEventListener("click", (event) => {
    const step = event.target.closest("[data-cal-step]");
    if (!step) return;
    // Months counted from year 0, so a step wraps the year with the month.
    const current = year * 12 + month - 1;
    const next = current + Number(step.dataset.calStep);
    // Stop at the ends; a page the server drew past them may still step in.
    const first = Math.min(FIRST_YEAR * 12, current);
    const last = Math.max(LAST_YEAR * 12 + 11, current);
    if (next < first || next > last) return;
    [year, month] = [Math.floor(next / 12), (next % 12) + 1];
    draw();
  });
}
