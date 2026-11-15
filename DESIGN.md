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
(`crates/tuvi-web/src/features/charts/accent.rs`). On the draw screen the
preview sets it once the hour is picked.

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
(`crates/tuvi-web/src/features/shell/`). On the right: the language switch
(`pill-ghost`, the lucide `languages` icon and "En" or "Vi"), then the night
and day switch. The language switch opens a `popover` dropdown
(`.language-menu`) with "En · English" and "Vi · Tiếng Việt"; the current one
is checked.

## Language

English is the main language, Vietnamese the second. Every interface text is
a Fluent message (`crates/tuvi-web/locales/`). Tử Vi terms (star names,
Địa Chi, Can Chi, Tràng Sinh stages) stay in Vietnamese in both languages;
the English text glosses them where it helps, e.g. "Bureau (Cục)".

## Night and day

Night is the default: the twilight design above. The switch in the header
turns the page to day and back (a sun at night, a moon by day). A `theme`
cookie keeps the pick, so the server renders the page in it. Where the
browser can, the switch cross-fades.

Day is a calm, dim dawn: dusky lavender at the top, mint and sky blue in the
corners, a peach glow at the bottom, deep plum ink (`#261640`), soft frosted
glass, and deeper accents (neutral `#7457d6` to orchid `#b45aa8`; darker
element colors).

- The crane logo tones down a touch, with a faint shade instead of a glow.
- The cards turn to a dawn of their own (lavender to peach), with white star
  hearts ringed in the accent.
- The casting loader takes the dawn sky; its crane is inverted and
  multiplied, so it shows in bronze.
- The 12-cung grid keeps the night palette (`.night`).

## Draw screen

The home page (`crates/tuvi-web/src/features/cast/`).

- Order on a phone and a tablet: title, the card, the birth form, the main
  button. The card is as wide as the form.
- From 1024px: one glass table under a centred title. The birth form is on
  the left, the card with the button on the right, 5rem apart, tops
  aligned; the table hugs them with 2.5rem on each side. The card and the
  button end where the birth form ends, so the last row lines up with the
  button (`cast.js` sets `--stage-card`; without it, the card width follows
  the screen height, 220–280px).
- The birth form (`glass`): three scroll-snap date wheels (day, month, year
  1900–2099) over one accent band, then the birth clock (`.birth-clock`),
  then a list of rows (`.option-row`): a hairline above each, the
  `.dial-label` on the left, small round `.chip`s (32px) on the right.
  - Time of day (Vi "Buổi"): ☀ AM and ☾ PM (lucide `sun`, `moon`).
  - Gender: Male and Female, Male by default.
  - Presets (see below).
  The picked chip has the accent gradient. The clock has no label and no
  hint.
- The clock: 12 at the top. A tap picks an hour; a drag around the face
  turns the hand with the finger and picks the nearest hour (a short
  vibration per hour where the phone supports it); arrow keys step. The
  center shows the hour, its canh giờ and range. The hand is a fine needle
  that fades in from the center, with a small pointer that ends at the edge
  of the lit hour.
- Presets (Vi "Cài sẵn"), the last row. `cast.js` shows it, so without
  JavaScript it is hidden. Each chip has a tooltip (`data-tip`), centered
  above it, on hover and on keyboard focus.
  - YOLO (the same in Vi, the lucide `dices` icon) rolls a real date (year
    1930 to this year), an hour and a gender.
  - Favorite (Vi "Yêu thích", lucide `heart`) saves one birth on this device
    (`localStorage`). It needs the hour. Once saved, the heart fills and
    shows the date; a tap fills the form in again, and a small × next to it
    forgets it.
  - Both fill the form the same way, about 2.5 s with no zoom: the wheels
    spin like slot reels and stop one after another, AM/PM and gender flick
    back and forth, the clock hand winds a full turn (no hour is lit until it stops), and the card sways. The card wakes once all have landed.
- The card (`.card-back`): 9:16, with "THIÊN HẠC" and "TỬ VI". Before an
  hour is picked it shows a quiet sky and a ✦. Then each chính tinh of cung
  Mệnh shows as its constellation (`.chom-*`), filling the card (twin stars
  stack), and the page accent takes the chart's color; vô chính diệu keeps
  the sky. A new portrait cross-fades with the old one (0.6 s). It floats on
  a 6 s cycle.
- "✦ Draw a card" posts the form. `cast.js` first shows the casting loader
  for 2.6 s. A missing hour or a date that is not on the calendar shows a
  toast instead (the clock pulses for a missing hour).
- Every control is a native radio, so the form works without JavaScript.

## Bản mệnh screen

A new draw lands here (`crates/tuvi-web/src/features/deck/`). The prototype
calls it `#s-home`; the roadmap is new and not in the prototype.

- The header: the card name ("Female · Bính Tý year" / "Nữ mạng Bính Tý"),
  the solar date as the title, a line with the hour, giờ and the lunar date,
  a badge with the chính tinh of cung Mệnh, and a chip with the lucky color
  (the element of Bản mệnh). With song tinh, the first name has `accent` and
  the second has `accent-2`.
- Below `lg` the card and the roadmap stack. On a phone the card, the
  panels and the buttons share the same 16px gutters; on a tablet the card
  and the buttons stop at 448px, centred. From `lg` the card is on the
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
  - A "Method" `<details>` under the panels: how the day is computed,
    ending with "This is guidance for reflection, not a prediction."
- Under the deck: "✦ Draw another card" (`pill-primary`, back to the form)
  and "See all 12 palaces" (`pill-ghost`).
- Each chính tinh of cung Mệnh shows its constellation (`.chom-*`): a faint
  ghost portrait (24% opacity, radial mask) under a star pattern in the
  accent. A wave draws the lines every 4.2 s, and each star pulses on a
  4.5 s cycle. On this card a figure fills the room between the archetype
  chip and the mantra; song tinh stack, their faded edges overlapping.
  Vô chính diệu shows the scattered sky. Under reduced motion, the lines and stars
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
