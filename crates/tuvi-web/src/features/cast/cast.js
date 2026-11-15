// The birth form: the parts that Topcoat runtime expressions cannot do.
//
// - Wheels: when a scroll settles, check the radio in the middle band.
// - Clock: drag around the face and the hand follows the finger; tap an
//   hour to jump.
// - Presets: YOLO spins the wheels, flicks AM/PM and gender, winds the clock
//   to a random birth moment; the heart saves one birth on this device and
//   fills it in again the same way.
// - Card: from 1024px the card and the draw button end where the birth
//   form ends.
// - Submit: a toast for a missing hour or a date that is not on
//   the calendar, then the casting loader, then the real POST.
//
// Every pick goes through a native `change` event. The Topcoat handlers on
// the radios then update the signals, the clock readout and the preview
// shard. Without this file the radios still work by tap, the form still
// posts, and the server checks every field.

const ROW_PX = 46;
const SETTLE_MS = 120;
const DRAG_START_PX = 6;
// Near the centre the finger angle jumps around: ignore it there.
const CLOCK_DEAD_ZONE = 0.12;
const TICK_VIBRATE_MS = 6;
const MIN_CASTING_MS = 2600;
const TOAST_MS = 2600;
const LAND_VIBRATE_MS = 12;
// Random birth years: people alive today, not the whole wheel.
const RANDOM_YEAR_FROM = 1930;
const FAVORITE_KEY = "thienhac.favorite";
const WIDE = "(min-width: 1024px)";
const CARD_RATIO = 9 / 16;

const form = document.querySelector("[data-cast]");
if (form) setUp(form);

/** Checks `input` and tells the page, like a tap would. */
function choose(input) {
  if (!input || input.checked) return;
  input.checked = true;
  input.dispatchEvent(new Event("change", { bubbles: true }));
}

const still = () => matchMedia("(prefers-reduced-motion: reduce)").matches;

/** Checks `input` without telling the page yet; `announce` tells it. */
function hold(input) {
  input.checked = true;
}

/** Tells the page about held picks, all in one go. */
function announce(inputs) {
  inputs.forEach((input) => input.dispatchEvent(new Event("change", { bubbles: true })));
}

/** Plays the CSS animation behind class `name` from the start. The class
 * goes once the `keyframes` animation ends. */
function play(element, name, keyframes) {
  element.classList.remove(name);
  void element.offsetWidth;
  element.classList.add(name);
  const end = (event) => {
    if (event.animationName !== keyframes) return;
    element.classList.remove(name);
    element.removeEventListener("animationend", end);
  };
  element.addEventListener("animationend", end);
}

const wait = (ms) => new Promise((done) => setTimeout(done, ms));

const toast = document.querySelector("[data-toast]");
let toastTimer;

/** Shows `text` in the toast for a moment. */
function say(text) {
  toast.textContent = text;
  toast.dataset.show = "";
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => delete toast.dataset.show, TOAST_MS);
}

/** Whether the wheels show a date on the calendar. */
function realDate(form) {
  const value = (name) => Number(form.elements[name].value);
  const [day, month, year] = [value("day"), value("month"), value("year")];
  const date = new Date(Date.UTC(year, month - 1, day));
  return date.getUTCDate() === day && date.getUTCMonth() === month - 1;
}

function setUp(form) {
  const wheels = {};
  form.querySelectorAll("[data-wheel]").forEach((wheel) => {
    wheels[wheel.querySelector("input").name] = setUpWheel(wheel);
  });
  const clock = form.querySelector("[data-clock]");
  const hand = clock && setUpClock(clock);
  setUpSubmit(form, clock);
  const tools = form.querySelector("[data-tools]");
  if (tools && hand) setUpPresets(form, tools, wheels, hand);
  setUpCardFit(form);
}

/** From 1024px the card and the draw button end where the birth form ends:
 * the card's width follows the form's height. Without this the CSS clamp
 * sizes it. */
function setUpCardFit(form) {
  const panel = form.querySelector("[data-birth-panel]");
  const draw = form.querySelector("[data-draw]");
  if (!panel || !draw || !("ResizeObserver" in window)) return;
  const column = draw.parentElement;
  const wide = matchMedia(WIDE);
  let height = 0;
  const fit = () => {
    const gap = parseFloat(getComputedStyle(column).rowGap) || 0;
    const card = height - draw.offsetHeight - gap;
    if (wide.matches && card > 0) {
      form.style.setProperty("--stage-card", `${card * CARD_RATIO}px`);
    } else {
      form.style.removeProperty("--stage-card");
    }
  };
  new ResizeObserver((entries) => {
    for (const entry of entries) {
      if (entry.target === panel) height = entry.contentRect.height;
    }
    fit();
  }).observe(panel);
  // An error line over the button makes it taller.
  new ResizeObserver(fit).observe(draw);
  wide.addEventListener("change", fit);
}

function setUpWheel(wheel) {
  const options = [...wheel.querySelectorAll("input[type=radio]")];
  const centre = (input, behavior) => {
    const top = options.indexOf(input) * ROW_PX;
    if (Math.abs(wheel.scrollTop - top) > 1) wheel.scrollTo({ top, behavior });
  };
  centre(options.find((option) => option.checked), "instant");

  let timer;
  wheel.addEventListener(
    "scroll",
    () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        const index = Math.round(wheel.scrollTop / ROW_PX);
        choose(options[Math.max(0, Math.min(options.length - 1, index))]);
      }, SETTLE_MS);
    },
    { passive: true },
  );
  // A tap or an arrow key checks a radio: bring it to the middle band.
  wheel.addEventListener("change", (event) => centre(event.target, "smooth"));

  return {
    options,
    /** Spins to `input` like a slot reel, easing out to a stop. Holds the
     * pick when it lands. */
    async spin(input, ms) {
      const from = wheel.scrollTop;
      const to = options.indexOf(input) * ROW_PX;
      if (!still()) {
        wheel.classList.add("spinning");
        const start = performance.now();
        await new Promise((done) => {
          const frame = (now) => {
            const t = Math.min(1, (now - start) / ms);
            wheel.scrollTop = from + (to - from) * (1 - (1 - t) ** 3);
            if (t < 1) requestAnimationFrame(frame);
            else done();
          };
          requestAnimationFrame(frame);
        });
        wheel.classList.remove("spinning");
      }
      wheel.scrollTo({ top: to, behavior: "instant" });
      hold(input);
      navigator.vibrate?.(LAND_VIBRATE_MS);
    },
  };
}

function setUpClock(clock) {
  // Face order: 12 at the top, then 1 to 11 clockwise.
  const hours = [...clock.querySelectorAll("input[name=gio]")];
  const current = () => hours.findIndex((hour) => hour.checked);
  const wrap = (position) => ((position % 12) + 12) % 12;

  // The hand angle in degrees. It keeps the total turns, so the hand moves
  // the short way from 11 to 12 and never spins back.
  let angle = 0;
  const setAngle = (degrees) => {
    angle += ((((degrees - angle) % 360) + 540) % 360) - 180;
    clock.style.setProperty("--hand-angle", `${angle}deg`);
  };
  if (current() >= 0) setAngle(current() * 30);

  /** The pointer angle (0 at 12, clockwise) and its distance from the centre,
   * as a fraction of the clock width. */
  const pointer = (event) => {
    const rect = clock.getBoundingClientRect();
    const x = event.clientX - rect.left - rect.width / 2;
    const y = event.clientY - rect.top - rect.height / 2;
    return {
      degrees: (Math.atan2(x, -y) * 180) / Math.PI,
      radius: Math.hypot(x, y) / rect.width,
    };
  };

  let drag = null;
  clock.addEventListener("change", (event) => {
    const position = hours.indexOf(event.target);
    // During a drag the hand follows the finger, not the hour.
    if (position >= 0 && !drag?.moved) setAngle(position * 30);
  });

  clock.addEventListener("pointerdown", (event) => {
    if (event.pointerType === "mouse" && event.button !== 0) return;
    drag = { id: event.pointerId, x: event.clientX, y: event.clientY, moved: false };
    clock.setPointerCapture(event.pointerId);
  });
  clock.addEventListener("pointermove", (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    if (!drag.moved) {
      if (Math.hypot(event.clientX - drag.x, event.clientY - drag.y) <= DRAG_START_PX) return;
      drag.moved = true;
      clock.classList.add("dragging");
    }
    const { degrees, radius } = pointer(event);
    if (radius < CLOCK_DEAD_ZONE) return;
    setAngle(degrees);
    const position = wrap(Math.round(angle / 30));
    if (position !== current()) {
      choose(hours[position]);
      navigator.vibrate?.(TICK_VIBRATE_MS);
    }
  });
  const end = (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    const { moved } = drag;
    drag = null;
    clock.classList.remove("dragging");
    if (moved) {
      // Settle the hand on the picked hour.
      if (current() >= 0) setAngle(current() * 30);
      return;
    }
    if (event.type !== "pointerup") return;
    // A tap on the ring picks the hour under it. A tap in the readout does
    // nothing.
    const { degrees, radius } = pointer(event);
    if (radius < 0.24) return;
    const position = wrap(Math.round(degrees / 30));
    choose(hours[position]);
    hours[position].focus({ preventScroll: true });
  };
  clock.addEventListener("pointerup", end);
  clock.addEventListener("pointercancel", end);
  // The pointer handlers own taps. A keyboard click (detail 0) still works.
  clock.addEventListener(
    "click",
    (event) => {
      if (event.detail > 0) event.preventDefault();
    },
    true,
  );

  return {
    hours,
    /** Winds the hand clockwise, whole `turns` and on to `hour`. Holds the
     * pick when it lands. */
    async spin(hour, turns, ms) {
      angle += ((((hours.indexOf(hour) * 30 - angle) % 360) + 360) % 360) + 360 * turns;
      clock.style.setProperty("--spin-ms", `${ms}ms`);
      clock.classList.add("spinning");
      clock.style.setProperty("--hand-angle", `${angle}deg`);
      await wait(still() ? 0 : ms);
      clock.classList.remove("spinning");
      hold(hour);
      navigator.vibrate?.(LAND_VIBRATE_MS);
    },
  };
}

/** Flicks two options back and forth, slowing down, until they land on
 * `target`. `group` holds the `labels` and takes the `rolling` class. Holds
 * the pick when it lands. */
async function flick(group, labels, target, flips) {
  const goal = labels.findIndex((label) => label.contains(target));
  let lit = Math.max(0, labels.findIndex((label) => label.querySelector(":checked")));
  if (!still()) {
    // The last flip must land on the goal.
    if ((lit + flips) % 2 !== goal) flips += 1;
    group.classList.add("rolling");
    let delay = 110;
    for (; flips > 0; flips -= 1) {
      lit = (lit + 1) % 2;
      labels.forEach((label, index) => label.classList.toggle("flash", index === lit));
      delay *= 1.2;
      await wait(delay);
    }
    group.classList.remove("rolling");
    labels.forEach((label) => label.classList.remove("flash"));
  }
  hold(target);
}

const randomOf = (list) => list[Math.floor(Math.random() * list.length)];

/** A random option from `allowed`, at least `gap` rows from the checked one
 * when there is room, so the reel has somewhere to spin. */
function farOption(options, allowed, gap) {
  const now = options.findIndex((option) => option.checked);
  const far = allowed.filter((option) => Math.abs(options.indexOf(option) - now) >= gap);
  return randomOf(far.length ? far : allowed);
}

/** The saved birth, or null. Storage can be off or hold junk. */
function loadFavorite() {
  try {
    const saved = JSON.parse(localStorage.getItem(FAVORITE_KEY));
    return saved && typeof saved === "object" ? saved : null;
  } catch {
    return null;
  }
}

function storeFavorite(saved) {
  try {
    if (saved) localStorage.setItem(FAVORITE_KEY, JSON.stringify(saved));
    else localStorage.removeItem(FAVORITE_KEY);
    return true;
  } catch {
    return false;
  }
}

function setUpPresets(form, tools, wheels, clock) {
  const stage = document.querySelector("[data-card-stage]");
  const random = tools.querySelector("[data-random]");
  const favorite = tools.querySelector("[data-favorite]");
  const favoriteText = favorite.querySelector("[data-favorite-text]");
  const forget = tools.querySelector("[data-forget]");
  const chipsOf = (name) => form.elements[name][0].closest(".chips");
  const chipLabels = (name) => [...chipsOf(name).querySelectorAll("label")];
  const radio = (name, value) => [...form.elements[name]].find((input) => input.value === String(value));
  let rolling = false;
  tools.hidden = false;

  /** Spins every control to `picks`, then tells the page in one go, so the
   * card wakes once, when all have landed. */
  const roll = async (button, picks) => {
    rolling = true;
    tools.setAttribute("aria-busy", "true");
    if (button === random) play(button, "rolling", "dice-roll");
    if (stage) play(stage, "shuffling", "card-shuffle");
    // The reels stop one after another.
    await Promise.all([
      wheels.day.spin(picks.day, 1800),
      wait(250).then(() => wheels.month.spin(picks.month, 1900)),
      wait(500).then(() => wheels.year.spin(picks.year, 2100)),
      flick(chipsOf("buoi"), chipLabels("buoi"), picks.buoi, 6),
      clock.spin(picks.gio, 1, 2400),
      flick(chipsOf("gender"), chipLabels("gender"), picks.gender, 7),
    ]);
    announce([picks.day, picks.month, picks.year, picks.buoi, picks.gio, picks.gender]);
    rolling = false;
    tools.removeAttribute("aria-busy");
  };

  random.addEventListener("click", () => {
    if (rolling) return;
    // A real date: the year, then the month, then a day that month has.
    const { day, month, year } = wheels;
    const thisYear = new Date().getFullYear();
    const yearPick = farOption(
      year.options,
      year.options.filter((option) => +option.value >= RANDOM_YEAR_FROM && +option.value <= thisYear),
      12,
    );
    const monthPick = farOption(month.options, month.options, 3);
    const days = new Date(Date.UTC(+yearPick.value, +monthPick.value, 0)).getUTCDate();
    roll(random, {
      day: farOption(day.options, day.options.slice(0, days), 6),
      month: monthPick,
      year: yearPick,
      gio: randomOf(clock.hours),
      buoi: randomOf([...form.elements.buoi]),
      gender: randomOf([...form.elements.gender]),
    });
  });

  /** The saved birth as radios, or null when it no longer fits the form. */
  const favoritePicks = () => {
    const saved = loadFavorite();
    if (!saved) return null;
    const picks = {
      day: radio("day", saved.day),
      month: radio("month", saved.month),
      year: radio("year", saved.year),
      gio: radio("gio", saved.gio),
      buoi: radio("buoi", saved.buoi),
      gender: radio("gender", saved.gender),
    };
    return Object.values(picks).every(Boolean) ? { saved, picks } : null;
  };

  /** The heart shows the saved date, or offers to save. */
  const show = () => {
    const found = favoritePicks();
    favorite.classList.toggle("saved", Boolean(found));
    forget.hidden = !found;
    if (found) {
      const { day, month, year, gio, buoi } = found.saved;
      favoriteText.textContent = `${day}/${month}/${year}`;
      const label = `${favorite.dataset.load} ${day}/${month}/${year} · ${gio} ${buoi}`;
      favorite.setAttribute("aria-label", label);
      favorite.title = label;
    } else {
      favoriteText.textContent = favorite.dataset.save;
      favorite.removeAttribute("aria-label");
      favorite.removeAttribute("title");
    }
  };
  show();

  favorite.addEventListener("click", () => {
    if (rolling) return;
    const found = favoritePicks();
    if (found) {
      roll(favorite, found.picks);
      return;
    }
    const value = (name) => form.elements[name].value;
    if (!value("gio")) {
      say(favorite.dataset.noHour);
      return;
    }
    if (!realDate(form)) {
      say(toast.dataset.invalidDate);
      return;
    }
    const saved = {};
    for (const name of ["day", "month", "year", "gio", "buoi", "gender"]) saved[name] = value(name);
    if (!storeFavorite(saved)) return;
    show();
    say(favorite.dataset.saved);
  });

  forget.addEventListener("click", () => {
    if (rolling || !storeFavorite(null)) return;
    show();
    favorite.focus();
    say(favorite.dataset.forgotten);
  });
}

function setUpSubmit(form, clock) {
  const casting = document.querySelector("[data-casting]");
  const crane = casting.querySelector("[data-crane]");
  let cast = false;

  // Missing hour: a toast and a pulse, not the browser bubble on a hidden
  // radio. Gender always has a pick (Male by default).
  let first = null;
  form.addEventListener(
    "invalid",
    (event) => {
      const field = event.target;
      if (first === null) {
        first = field;
        setTimeout(() => (first = null));
      }
      if (field.name !== "gio") return;
      event.preventDefault();
      if (field !== first) return;
      say(toast.dataset.noHour);
      clock.classList.remove("need");
      void clock.offsetWidth;
      clock.classList.add("need");
      clock.scrollIntoView({ behavior: "smooth", block: "center" });
    },
    true,
  );

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (cast) return;
    if (!realDate(form)) {
      say(toast.dataset.invalidDate);
      return;
    }
    cast = true;
    casting.hidden = false;
    // Under reduced motion the crane stays still on its first frame.
    if (!matchMedia("(prefers-reduced-motion: reduce)").matches) crane.play().catch(() => {});
    // The interlude has a floor, from the design. The POST then runs the
    // server checks and redirects to the card.
    setTimeout(() => form.submit(), MIN_CASTING_MS);
  });

  // Back from the card: the browser may restore this page as it was.
  window.addEventListener("pageshow", (event) => {
    if (!event.persisted) return;
    cast = false;
    casting.hidden = true;
    crane.pause();
  });
}
