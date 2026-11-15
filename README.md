# Thiên Hạc

Thiên Hạc is a gacha app for **Lá Số Tử Vi** (Vietnamese astrology charts).
One tap draws a random Bản mệnh card. Each card is a real chart: a real date
and hour on the calendar, with the stars placed by the chart engine.

Thiên Hạc is one Rust binary. It has no database, no accounts and no AI.

## Parts

| Folder | What it is | Tech |
| --- | --- | --- |
| `crates/tuvi-core/` | Chart engine. No IO, no network, no database. | Rust |
| `crates/tuvi-web/` | Web app: pages and the draw | [Topcoat](https://github.com/tokio-rs/topcoat) |
| `crates/tuvi-cli/` | Dev command: `cargo up` | Rust, [clap](https://docs.rs/clap) |
| `data/` | Cách cục rules and câu phú text, for later features | YAML, JSON |
| `docs/` | Domain notes and the cách cục rule language | Markdown |

## How it works

```text
Browser ──► tuvi-web (:3000, server-rendered HTML) ──► tuvi-core (chart engine)
```

1. The home page shows a face-down card and the button **Rút lá bài**.
2. The button posts to `/la-bai`. The server picks a random year (1900–2099),
   month, day, hour (0–23) and gender. When the date is not on the calendar
   (for example 30/02), it draws again. So every real date, hour and gender
   is equally likely.
3. The server redirects to the card: `/la-bai/{code}`, for example
   `/la-bai/1996-04-15-22-nu`. The code is the birth moment.
4. The card page builds the chart from the code and shows the Bản mệnh deck:
   three cards, with the solar date, the hour, and the lunar date. The link
   **Xem đủ 12 cung** (`/la-bai/{code}/la-so`) draws the whole chart.

Nothing is stored. The URL is the card, so a card link can be shared. One
chart takes about 1 µs to build, so each page builds it again.

Topcoat renders all HTML on the server. There is no separate API and no
JavaScript build step. One small static script (`gacha.js`) shows the
shuffle animation before the draw.

## Setup

You need Rust (stable, edition 2024) and the Topcoat CLI:

```bash
cargo install topcoat-cli
```

Start the app (from any folder in the repo):

```bash
cargo up                  # http://127.0.0.1:3000
```

`cargo up` runs `topcoat dev`. The dev server rebuilds the app and reloads
the page when you save a file. Press Ctrl-C to stop it.

## Folders

```text
Cargo.toml                        workspace: every crate is in crates/
.cargo/config.toml                aliases: cargo tuvi, cargo up
crates/
  tuvi-core/                      chart engine
    src/lib.rs                    module list and public API
    src/chart.rs                  BirthInput, Chart, Cung, build_chart
    src/calendar.rs               solar to lunar date, birth hour branch
    src/calendar/lunar_table.rs   lunar years 1800–2199 (generated, do not edit)
    src/can_chi.rs                Thiên Can and Địa Chi
    src/cung.rs                   12 cung roles, Mệnh, Thân, Thiên Can of each cung
    src/cuc.rs                    Cục
    src/chinh_tinh.rs             the 14 main stars
    src/phu_tinh.rs               the other natal stars, Tràng Sinh ring included
    src/tu_hoa.rs                 Tứ Hóa of the birth year
    src/tuan_triet.rs             Tuần and Triệt
    src/ban_menh.rs               Bản mệnh of the birth year
    src/van_han.rs                Đại hạn and Tiểu hạn of each cung
    scripts/gen_fixtures.py       makes the reference data from the old Python engine
    tests/parity.rs               compares 359 charts with saved reference data
    tests/bench.rs                speed test
  tuvi-web/                       web app
    src/main.rs                   start-up
    src/app.rs                    root layout, home page (URL /), web manifest
    src/app/la_bai.rs             POST /la-bai (draws a card)
    src/app/la_bai/code.rs        GET /la-bai/{code} (the Bản mệnh deck)
    src/app/la_bai/code/la_so.rs  GET /la-bai/{code}/la-so (the 12 cung)
    src/features/gacha/           the draw, the card code, draw screen, loader, gacha.js
    src/features/ban_menh/        Bản mệnh deck: card text, chart facts, constellations
    src/features/charts/          chart grid, accent
    src/features/shell/           the logo and the name
    styles.css                    theme: tokens and utilities (see DESIGN.md)
    assets/                       app icons, crane video, god-ghosts/ portraits (asset!)
  tuvi-cli/                       dev command (cargo tuvi --help)
data/                             data for later features (see data/README.md)
docs/                             domain notes and the design prototype
DESIGN.md                         colors, fonts and layout rules
```

`src/app/` holds pages only: the file path is the URL path. Pages call
`src/features/` for data, rules and the UI parts of each feature.

## Tests

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Measure the speed of the engine:

```bash
cargo test --release -p tuvi-core --test bench -- --ignored --nocapture
```

There is no CI. Run the tests before you open a PR.

## How to contribute

**Change the chart engine:** edit `crates/tuvi-core/` and run
`cargo test -p tuvi-core`. The old Python engine made the reference charts in
`tests/fixtures/charts.json`. Change this file only when you change a result on
purpose, and say why in the PR.

**Add a page:** add a file in `crates/tuvi-web/src/app/`. The file path is the
URL path. Put data and rules in `crates/tuvi-web/src/features/`, not in the page.

**Change the look:** read [`DESIGN.md`](DESIGN.md) first. Use its token classes
(`text-muted`, `glass`, `pill-primary`, ...), not color values.

**Add a premade UI component:** from `crates/tuvi-web/`, run
`topcoat ui add <name>`. It goes in `src/components/`. The Topcoat UI token
names in `styles.css` already point to the design tokens.

**Run without `topcoat dev`:** run `topcoat asset bundle` first. The binary
reads its CSS and JS from `target/<profile>/assets/`. Without them, it does not start.

**Rules:**

- `tuvi-core` has no IO, network or database calls.
- Domain names use Vietnamese words without accents: `cung`, `sao`, `dia_chi`.
- Folder names match crate names: `crates/<crate-name>/`.
- No `unwrap` outside tests. No `unsafe`.
- Make a branch and open a PR to `main`.

## Not done yet

- Bản mệnh deck: the card flip, the focus zoom and the dots of the phone
  deck, the auras around the main card, and the "Lưu ảnh" and "Chia sẻ"
  buttons
- Card rarity
- Stars that move per period (lưu tinh, Tứ Hóa of a Đại hạn or a year),
  cách cục, câu phú

## Known issues

- The engine takes the birth hour as Vietnam time. It does not handle time zones.
- A date in a leap month gets the same month number as the regular month.
  The old engine did the same. Some schools of Tử Vi use the next month.
- Supported dates are 1800-01-25 (Tết 1800) to 2199-12-31 22:59.
- The engine copies the old Python rules, also where they differ from common
  Tử Vi tables: Cô Thần for Hợi Tý Sửu years is in Mùi (not Dần), Quả Tú for
  Thân Dậu Tuất years is in Dần (not Mùi), Lưu Hà for Tân years is in Thìn
  (not Mão), and Thiên Quan and Thiên Phúc for Ất and Bính years differ.
- Tiểu hạn goes in the same direction as Đại hạn (by gender and the Âm/Dương
  of the year), as in the old engine. Many schools use gender only: clockwise
  for a man, counter-clockwise for a woman.
- A cung lists its Tứ Hóa in the old engine's id order (Khoa, Kỵ, Lộc, Quyền),
  not in the usual order (Lộc, Quyền, Khoa, Kỵ).
- A draw is a POST, so the browser does not prefetch it. The draw saves
  nothing, so it needs no CSRF check.
- The ghost portraits (`assets/god-ghosts/*.svg`) are 77–414 KB each. The
  page loads one only when its star is on the card, and the router sends
  it compressed (for example 223 KB becomes 75 KB). The star coordinates
  follow these SVG portraits. The old app's smaller `*-ghost.webp` files are
  square crops that its last version did not use, so they are not ported.
- Topcoat is an early-stage project. Expect breaking changes when you
  update it.

## Docs

- [`DESIGN.md`](DESIGN.md): colors, fonts, accent and layout rules
- [`docs/design/prototype.html`](docs/design/prototype.html): the full design prototype
- [`docs/domain.md`](docs/domain.md): what a chart is and how its parts relate
- [`docs/cach_cuc_rules.md`](docs/cach_cuc_rules.md): the rule language of `data/cach_cuc.yaml`
- [`data/README.md`](data/README.md): the data files

## Words

| Word | Meaning |
| --- | --- |
| Lá số | The chart |
| Cung | One of 12 boxes on the chart. Each has a role: Mệnh (self), Tài Bạch (money), … |
| Địa Chi | The 12 positions (Tý to Hợi) |
| Thiên Can | The 10 heavenly stems |
| Sao | Star. Chính tinh = 14 main stars. Phụ tinh = minor stars. |
| Cục | Element from the Mệnh box. It decides where the star Tử Vi goes. |
| Thân | The second "self" cung. It is marked "(Thân)" on the chart. |
