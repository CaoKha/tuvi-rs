# Data

Reference data. The app reads `daily_guide.json`; the other files are for
features that are not ported yet.

| File | What it is | Format |
| --- | --- | --- |
| `daily_guide.json` | The roadmap texts, in English and Vietnamese, with sources: each cung role (theme, do, avoid), each Hóa in each cung, each hour god. The app reads it. | JSON |
| `star_patterns.yaml` | Cách cục: known star patterns, their conditions and meanings | Rules in the language of [`docs/star_pattern_rules.md`](../docs/star_pattern_rules.md) |
| `verses.json` | Câu phú: a classic verse for each mix of chính tinh, position and Tuần/Triệt in cung Mệnh (480 cases) | JSON |

The book *Tử Vi Tân Biên* is not in git. Its files go in `data/tuvitanbien*/`,
which git ignores. Ask a maintainer for them.
