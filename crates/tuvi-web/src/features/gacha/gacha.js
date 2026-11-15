// The draw button: show the shuffle loader, then send the POST.
//
// Without this file the form still posts and the deck still opens; only the
// loader is missing.

const MIN_SHUFFLE_MS = 2600;

const form = document.querySelector("[data-gacha]");
if (form) setUp(form);

function setUp(form) {
  const loader = document.querySelector("[data-casting]");
  const crane = loader.querySelector("[data-crane]");
  let drawing = false;

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (drawing) return;
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
