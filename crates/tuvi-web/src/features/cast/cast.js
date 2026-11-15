// The birth form: the parts that Topcoat runtime expressions cannot do.
//
// - Wheels: when a scroll settles, check the radio in the middle band.
// - Clock: drag around the face and the hand follows the finger; tap an
//   hour to jump.
// - Submit: a toast for a missing hour or gender or a date that is not on
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

const form = document.querySelector("[data-cast]");
if (form) setUp(form);

/** Checks `input` and tells the page, like a tap would. */
function choose(input) {
  if (!input || input.checked) return;
  input.checked = true;
  input.dispatchEvent(new Event("change", { bubbles: true }));
}

function setUp(form) {
  form.querySelectorAll("[data-wheel]").forEach(setUpWheel);
  const clock = form.querySelector("[data-clock]");
  if (clock) setUpClock(clock);
  setUpSubmit(form, clock);
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
}

function setUpSubmit(form, clock) {
  const casting = document.querySelector("[data-casting]");
  const crane = casting.querySelector("[data-crane]");
  const toast = document.querySelector("[data-toast]");
  const gender = form.querySelector("[data-gender]");
  let toastTimer;
  let cast = false;

  const say = (text) => {
    toast.textContent = text;
    toast.dataset.show = "";
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => delete toast.dataset.show, TOAST_MS);
  };

  // Missing hour or gender: a toast and a pulse, not the browser bubble on a
  // hidden radio.
  let first = null;
  form.addEventListener(
    "invalid",
    (event) => {
      const field = event.target;
      if (first === null) {
        first = field;
        setTimeout(() => (first = null));
      }
      if (field.name !== "gio" && field.name !== "gender") return;
      event.preventDefault();
      if (field !== first) return;
      if (field.name === "gio") {
        say(toast.dataset.noHour);
        clock.classList.remove("need");
        void clock.offsetWidth;
        clock.classList.add("need");
        clock.scrollIntoView({ behavior: "smooth", block: "center" });
      } else {
        say(toast.dataset.noGender);
        gender.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    },
    true,
  );

  const realDate = () => {
    const value = (name) => Number(form.elements[name].value);
    const [day, month, year] = [value("day"), value("month"), value("year")];
    const date = new Date(Date.UTC(year, month - 1, day));
    return date.getUTCDate() === day && date.getUTCMonth() === month - 1;
  };

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (cast) return;
    if (!realDate()) {
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
