// The draw form: the parts that HTML alone cannot do.
//
// - Wheels: when a scroll settles, check the radio in the middle band.
// - Submit: check that the date is on the calendar, show the shuffle
//   loader, then send the POST.
//
// Without this file the radios still work by tap, the form still posts, and
// the server checks the date.

const ROW_PX = 46;
const SETTLE_MS = 120;
const MIN_SHUFFLE_MS = 2600;
const TOAST_MS = 2600;

const form = document.querySelector("[data-gacha]");
if (form) setUp(form);

function setUp(form) {
  form.querySelectorAll("[data-wheel]").forEach(setUpWheel);
  setUpSubmit(form);
}

/** Checks `input` and tells the page, like a tap would. */
function choose(input) {
  if (!input || input.checked) return;
  input.checked = true;
  input.dispatchEvent(new Event("change", { bubbles: true }));
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

function setUpSubmit(form) {
  const loader = document.querySelector("[data-casting]");
  const crane = loader.querySelector("[data-crane]");
  const toast = document.querySelector("[data-toast]");
  let toastTimer;
  let drawing = false;

  const say = (text) => {
    if (!toast) return;
    toast.textContent = text;
    toast.dataset.show = "";
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => delete toast.dataset.show, TOAST_MS);
  };

  const realDate = () => {
    const value = (name) => Number(form.elements[name].value);
    const [day, month, year] = [value("day"), value("month"), value("year")];
    const date = new Date(Date.UTC(year, month - 1, day));
    return date.getUTCDate() === day && date.getUTCMonth() === month - 1;
  };

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (drawing) return;
    if (!realDate()) {
      say(toast?.dataset.invalid ?? "");
      return;
    }
    drawing = true;
    loader.hidden = false;
    // Under reduced motion the crane stays still on its first frame.
    if (!matchMedia("(prefers-reduced-motion: reduce)").matches) crane.play().catch(() => {});
    // The interlude has a floor, from the design. The POST then draws the
    // card and redirects to its deck.
    setTimeout(() => form.submit(), MIN_SHUFFLE_MS);
  });

  // Back from the deck: the browser may restore this page as it was.
  window.addEventListener("pageshow", (event) => {
    if (!event.persisted) return;
    drawing = false;
    loader.hidden = true;
    crane.pause();
  });
}
