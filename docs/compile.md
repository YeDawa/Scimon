# Compiler

The `compile` command turns a source file into a PDF, picking the format from the
file extension:

- **[LaTeX](./compile/latex.md)** (`.tex`) — a built-in engine, no TeX
  distribution (TeX Live, MiKTeX, etc.) required.
- **[Markdown](./compile/markdown.md)** (`.md` / `.markdown`) — rendered with the
  generic template.

Both are typeset with [MathJax](https://www.mathjax.org/) and printed to PDF
through the same headless-browser pipeline.

## Usage

Compile a local file — the format is chosen from its extension:

```shell
scimon compile paper.tex
scimon compile notes.md
```

The PDF is written next to the input, reusing its name (`paper.pdf`, `notes.pdf`).

### Custom output

Use `-o` / `--output` to choose the output file. The extension is normalized to
`.pdf` automatically:

```shell
scimon compile paper.tex -o build/final.pdf
```

### Remote files

You can compile a file straight from a URL:

```shell
scimon compile https://example.com/paper.tex
scimon compile https://example.com/readme.md
```

When no `--output` is given, the file name is derived from the URL.

### Inside a downloads block

Any `.tex` URL listed in a `downloads { }` block is compiled to PDF
automatically, so you can mix LaTeX sources with regular downloads:

```scimon
downloads {
    https://example.com/paper.tex as "my-paper.pdf"
}
```

## Formats

- [LaTeX](./compile/latex.md) — supported commands, environments and bundled
  packages.
- [Markdown](./compile/markdown.md) — rendering details and options.
