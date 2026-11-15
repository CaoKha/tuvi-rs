// The theme switch without a reload: flip the class on <html>, keep the
// pick in the cookie the server reads, and point the form at the other
// theme. Where the browser has view transitions, the page cross-fades.

const YEAR_S = 60 * 60 * 24 * 365;
const BAR = { dark: "#1a1033", light: "#d8cbea" };

const form = document.querySelector("[data-theme-form]");
if (form) {
  const button = form.querySelector("button");
  const field = form.elements.theme;

  const apply = (theme) => {
    const root = document.documentElement;
    root.classList.toggle("dark", theme === "dark");
    root.classList.toggle("light", theme === "light");
    document.cookie = `theme=${theme}; path=/; max-age=${YEAR_S}; samesite=lax`;
    document.querySelector('meta[name="theme-color"]')?.setAttribute("content", BAR[theme]);
    field.value = theme === "dark" ? "light" : "dark";
    const label = theme === "dark" ? button.dataset.toDay : button.dataset.toNight;
    button.setAttribute("aria-label", label);
    button.setAttribute("title", label);
  };

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const theme = field.value;
    const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (document.startViewTransition && !still) document.startViewTransition(() => apply(theme));
    else apply(theme);
  });
}
