# Thiên Hạc

Thiên Hạc is a web app for Tử Vi (Vietnamese astrology). You enter your birth
date and hour. The app draws your destiny card and shows a plan for today or any day you pick,
hour by hour.

- No accounts, no database, no AI. Nothing is stored.
- The card link holds the birth moment, so you can share it.
- English and Vietnamese.
- Night theme (default) and day theme.

## Features

- **Birth form.** Date wheels, a birth clock, and gender (Male by default).
  The card shows your constellation while you pick.
- **Destiny card.** The main stars of your Mệnh palace, an archetype and a
  short mantra.
- **Today.** The palace and the four transformations of the day, and the 12
  two-hour periods with what to do and what to avoid.
- **Full chart.** All 12 palaces with their stars.

## How it works

The browser talks to one Rust server. The server builds the chart and
renders every page as HTML. Two small scripts add the clock drag, the date
wheels, the loading animation and the theme switch.

| Part | Role |
| --- | --- |
| `crates/tuvi-core` | Chart engine: calendar, palaces, stars, daily flow |
| `crates/tuvi-web` | Web app, built with [Topcoat](https://github.com/tokio-rs/topcoat) |
| `crates/tuvi-cli` | Dev command (`cargo up`) |
| `data/` | Daily guide texts (English and Vietnamese) |
| `docs/` | Domain notes and the design prototype |

## Run it

You need Rust and the Topcoat CLI.

```bash
cargo install topcoat-cli
cargo up            # http://127.0.0.1:3000
```

The dev server reloads when you save a file.

## Test

```bash
cargo test --workspace
```

## Deploy

The app runs in a Docker container. It is on Render's free plan.

- `Dockerfile` builds the app.
- `render.yaml` describes the service.
- CI (`.github/workflows/ci.yml`) checks every push: format, lint, tests,
  translations and the Docker image. A push to `main` that passes deploys to
  Render.

A free Render service sleeps when idle. The first visit after that is slow.

## Android

`android/` wraps the same server in an Android app that works offline. A push
to `main` publishes the APK as the
[`android-latest` release](https://github.com/CaoKha/tuvi-rs/releases/tag/android-latest).
Read [`android/README.md`](android/README.md) to install it or set up signing.

## Contribute

- Read [`DESIGN.md`](DESIGN.md) before you change the look.
- Put interface text in `crates/tuvi-web/locales/` (English and Vietnamese).
- Keep the chart engine free of IO.
- Open a pull request to `main`.

## Limits

- Birth hours are Vietnam time.
- Supported years: 1800 to 2199.
- Some star rules follow the old engine, not the most common tables.
- The guidance is for reflection, not prediction.

## Docs

- [`DESIGN.md`](DESIGN.md): colors, fonts, layout
- [`docs/domain.md`](docs/domain.md): Tử Vi terms and how a chart works
- [`data/README.md`](data/README.md): the data files
