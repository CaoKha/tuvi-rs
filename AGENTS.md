# AGENTS.md

## 1. Think Before Coding

- State your assumptions. If you are not sure, ask.
- If a request has more than one meaning, list the meanings.
- If a simpler approach exists, say so.
- Do not invent Tử Vi rules. If a rule is not in the repo, ask.

## 2. Simplicity First

- Add no features, options or abstractions that the user did not ask for.
- Add no error handling for cases that cannot occur.
- Put star positions and cách cục in the YAML data, not in Python branches.

## 3. Surgical Changes

- Change only the lines that the request needs.
- Follow the existing style. Do not refactor code that works.
- Remove only the dead code that your change made.
- Keep `tuvi_engine` free of network, database and AI calls.
- Name code with Vietnamese words without accents: `cung`, `sao`, `dia_chi`.

## 4. Goal-Driven Execution

- Write a failing test first, then make it pass.
- Before you say a task is done, run `uv run tuvi check` and `uv run tuvi test`.
