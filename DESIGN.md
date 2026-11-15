# Design

The product name is **Thiên Hạc**. The look is a dark "twilight" sky with
frosted glass cards. One accent color comes from the user's chart.

**Reference:** [`docs/design/prototype.html`](docs/design/prototype.html) is the
full design prototype. Open it in a browser. When this file and the prototype
do not agree, the prototype is correct.

**Code:** the tokens and utilities are in
[`crates/tuvi-web/styles.css`](crates/tuvi-web/styles.css) (Tailwind v4).

## Colors

| Token | Value | Use |
| --- | --- | --- |
| `page` | `#120a24` | Page background |
| `bg-0`, `bg-1`, `bg-2` | `#1a1033`, `#2e1a4a`, `#3b2063` | Twilight shades. `bg-0` is also text on accent fills. |
| `ink` | `#f3effa` | Main text |
| `muted` | `#b9aed1` | Labels and secondary text |
| `coral` | `#ff9e7d` | Flags, for example "Thân" |
| `fire` | `#ff5d6c` | Errors |
| `glass`, `glass-line` | white 6%, ink 14% | Card fill and hairlines |
| `accent`, `accent-2`, `accent-glow` | per chart | Highlights, star names, primary button |

Never use black. Never put a color value in a `.rs` file: use a token class.

## Accent from ngũ hành

The accent is the element of the chính tinh in cung Mệnh:

| Element | Color | Chính tinh |
| --- | --- | --- |
| Kim | `#e0dbcf` | Thất Sát, Vũ Khúc |
| Mộc | `#63c79a` | Tham Lang, Thiên Cơ, Thiên Lương |
| Thủy | `#79b7ff` | Phá Quân, Thái Âm, Thiên Tướng, Thiên Đồng, Cự Môn |
| Hỏa | `#ff5d57` | Thái Dương, Liêm Trinh |
| Thổ | `#b3855a` | Tử Vi, Thiên Phủ |

- One star: `accent` is the element, `accent-2` is its light shade.
- Two stars: `accent` is the first element, `accent-2` is the second.
- No star (vô chính diệu) or no chart: the neutral lavender `#bca6f5`.

The card pages set `--accent` on their `<main>`
(`crates/tuvi-web/src/features/charts/accent.rs`). The draw screen has no
chart, so it keeps the neutral accent.

## Type

| Font | Use |
| --- | --- |
| Fraunces (`font-display`) | Titles (`h1`–`h3`), star names, Địa Chi |
| Be Vietnam Pro (`font-sans`) | Everything else |

Both fonts load with the Vietnamese subset. Small labels are uppercase, bold,
10px, with wide letter spacing (`tracking-[0.2em]` or more).

## Surfaces and controls

| Class | Use |
| --- | --- |
| `glass` | Every card and panel: glass fill, hairline border, 26px radius, blur |
| `pill-primary` | The one main action on a screen: accent gradient, round |
| `pill-ghost` | Other choices, for example the gender toggle |
| `cosmos` | The fixed page backdrop, with two slow nebulae |

## Header

Every page has one header: the logo and the name, linked to the draw screen
(`crates/tuvi-web/src/features/shell/`), and on the right the language switch
(`pill-ghost`, labelled in the other language: "Tiếng Việt" or "English").
There is no tab bar and no login.

## Language

English is the main language, Vietnamese the second. Every interface text is
a Fluent message (`crates/tuvi-web/locales/`). Tử Vi terms (star names,
Địa Chi, Can Chi, Tràng Sinh stages) stay in Vietnamese in both languages;
the English text glosses them where it helps, e.g. "Bureau (Cục)".

## Draw screen

The home page (`crates/tuvi-web/src/features/gacha/`). It is not in the
prototype: it reuses the prototype's An sao title, card and loader styles.

- Order on a phone: title, the face-down card, the date wheels, the main
  button. From 768px: the title, the wheels and the button on the left, the
  card on the right.
- The date wheels (`.wheel`, from the prototype's `.picker`): day, month and
  year 1900–2099 in a `glass` card, three scroll-snap wheels over one accent
  band. Each row is a native radio; the picked row has the accent gradient.
  A date that is not on the calendar shows a toast and does not post.
- The face-down card (`.card-back`): the 9:16 twilight card of Lá Bài Bản
  Mệnh, with "THIÊN HẠC" and "TỬ VI", two quiet skies and a ✦. It floats on
  a 6 s cycle.
- The main button "✦ Draw a card" posts the date. `gacha.js` first shows the
  shuffle loader for 2.6 s: orbit rings, the crane video
  (`mix-blend-mode: screen`) and three rotating lines. Under reduced motion,
  the crane stays on its first frame.
- Without JavaScript the form still posts; only the loader is missing.

## Bản mệnh screen

A new draw lands here (`crates/tuvi-web/src/features/deck/`). The prototype
calls it `#s-home`; the roadmap is new and not in the prototype.

- The header: the card name ("Female · Bính Tý year" / "Nữ mạng Bính Tý"),
  the solar date as the title, a line with the hour, giờ and the lunar date,
  a badge with the chính tinh of cung Mệnh, and a chip with the lucky color
  (the element of Bản mệnh). With song tinh, the first name has `accent` and
  the second has `accent-2`.
- Below `lg` the card and the roadmap stack. From `lg` the card is on the
  left (364px, sticky) and the roadmap on the right.
- Lá Bài Bản Mệnh (main card): 9:16, a twilight gradient, an accent border
  and glow. It shows the archetype chip, the constellations (`.chom-*`) and
  the mantra in italic Fraunces. Vô chính diệu shows
  the scattered sky.
- Today's roadmap (`crates/tuvi-web/src/features/today/`), two `glass`
  panels:
  - The day: date, lunar date and day Can Chi; "Today's palace" in a light
    accent box with the theme, ✓ Do (accent) and ✕ Avoid (coral); then the
    four Hóa of the day, each with a dot in its color (Lộc `element-moc`,
    Quyền `element-hoa`, Khoa `element-thuy`, Kỵ `coral`).
  - The timeline: 12 canh giờ on a vertical line. Each hour is a
    `<details>`: the summary shows the time, the hour's Can Chi, badges, the
    hour god (accent when hoàng đạo), the palace, the Hóa chips and the
    advice; the body shows the hour god's advice and the palace theme. The
    current hour is open, with an accent fill and a lit dot; past hours fade
    to 55%. "✦ Best" marks up to three hours; "Take care" (coral) marks
    hours with Hóa Kỵ.
  - A "Method and sources" `<details>` under the panels.
- Under the deck: "✦ Draw another card for this date" (`pill-primary`) and
  "See all 12 palaces" (`pill-ghost`).
- Each chính tinh of cung Mệnh shows its constellation (`.chom-*`): a faint
  ghost portrait (24% opacity, radial mask) under a star pattern in the
  accent. A wave draws the lines every 4.2 s, and each star pulses on a
  4.5 s cycle. Song tinh show two smaller figures side by side. Vô chính
  diệu shows the scattered sky. Under reduced motion, the lines and stars
  stay still and lit.

## Chart

- A 4×4 grid. The 12 cung go around the edge, read clockwise from Tị at the top
  left. The center 2×2 shows the name and birth data.
- Each cung shows: role (top left), Địa Chi (top right), chính tinh in the
  accent color ("Vô chính diệu" when empty), the first two phụ tinh and "+N",
  coral flags (Cung Thân, Tuần, Triệt), and on the bottom row the Tràng Sinh
  stage (left) and the Đại hạn ages (right).
- Cung Mệnh has an accent border and a light accent fill.
- A tap on a cung opens its popup: Thiên Can, Địa Chi and Âm/Dương, the
  Tràng Sinh stage, Đại hạn and Tiểu hạn, all chính tinh with Tứ Hóa (in the
  order Lộc, Quyền, Khoa, Kỵ), and all phụ tinh. Each star has a dot in the
  color of its element; stars without a known element use `muted`.
- Popups use the HTML `popover` attribute, so the browser handles Esc, a tap
  outside and focus. Do not add a script for this.

## Rules

- Mobile first. Check every page at 390px wide and at 1280px wide.
- Honor `prefers-reduced-motion` (`styles.css` handles it once for all pages).
- Scrollbars are hidden, but every region still scrolls.
