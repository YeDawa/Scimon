# Contributing to Scimon

First off, thank you for taking the time to contribute! 🎉

This document explains how to set up the project, the conventions we follow, and
how to get your changes merged. Whether you're fixing a typo, reporting a bug, or
adding a whole new feature, you're welcome here.

## Ways to contribute

- 🐛 **Report bugs** — open an [issue](https://github.com/YeDawa/Scimon/issues) with steps to reproduce.
- 💡 **Suggest features** — open an issue describing the use case and what you'd expect.
- 📖 **Improve the docs** — everything under [`docs/`](docs) powers [docs.scimon.dev](https://docs.scimon.dev).
- 🔧 **Submit code** — fix a bug or implement a feature via a pull request.

## Reporting issues

Before opening an issue, please:

1. Search existing issues to avoid duplicates.
2. Use a clear, descriptive title.
3. For bugs, include:
   - What you ran (the command and, when relevant, the `.mon` or `.tex` input).
   - What you expected to happen and what actually happened.
   - Your OS and Scimon version (`scimon --version`).
   - Any error output.

## Development setup

You'll need:

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (stable).
- A **Chromium/Chrome** install — required to render HTML/Markdown/LaTeX to PDF.
- [pdfium binaries](https://github.com/bblanchon/pdfium-binaries) — only if you work on the `covers` feature.

Clone and build:

```bash
git clone https://github.com/YeDawa/Scimon.git
cd Scimon
cargo build
```

Run the CLI during development:

```bash
cargo run -- run scimon.mon
cargo run -- compile paper.tex
```

## Making changes

1. **Fork** the repository and create a branch off `main`:

   ```bash
   git checkout -b fix/short-description
   ```

2. Make your changes in small, focused commits.
3. Make sure the project still builds before pushing:

   ```bash
   cargo build
   ```

   If you have them set up, `cargo fmt` and `cargo clippy` are appreciated too.

4. Push your branch and open a pull request against `main`.

## Coding guidelines

- Match the style of the surrounding code (naming, formatting, module layout).
- Prefer clear, self-explanatory code over comments; add comments where the
  *why* isn't obvious.
- **Don't suppress warnings — fix them.** In particular, remove unused code
  instead of silencing it with `#[allow(dead_code)]`.
- Keep changes scoped: avoid mixing unrelated refactors into a feature/bug fix.
- When you add or change behavior, update the relevant docs under [`docs/`](docs).

## Commit messages

This project follows [Conventional Commits](https://www.conventionalcommits.org/).
Prefix each commit with a type:

| Type        | When to use it                                  |
| ----------- | ----------------------------------------------- |
| `feat:`     | A new feature.                                  |
| `fix:`      | A bug fix.                                       |
| `docs:`     | Documentation-only changes.                     |
| `refactor:` | Code changes that neither fix a bug nor add a feature. |
| `chore:`    | Tooling, dependencies, or maintenance.          |
| `test:`     | Adding or fixing tests.                          |

Examples:

```
feat: add TypeScript support to the commands runner
fix: embed lazy-loaded images before printing the PDF
docs: document the LaTeX compiler command
```

## Pull requests

- Keep PRs focused and reasonably small — they're easier to review and merge.
- Describe **what** changed and **why**. Link any related issues (e.g. `Closes #123`).
- Make sure `cargo build` passes and the docs are updated when behavior changes.
- Be responsive to review feedback; we're happy to help get your PR over the line.

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](LICENSE) that covers this project.
