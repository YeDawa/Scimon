# Convert

Convert a local file from one format to another with the `convert` directive. It
takes an input path and an output path, separated by `>`:

```scimon
convert "doc.md" > "doc.pdf"
```

The conversion is chosen from the **input and output extensions**.

## Supported conversions

| From            | To       | Notes                                                        |
| --------------- | -------- | ------------------------------------------------------------ |
| `.md`           | `.pdf`   | Renders Markdown to a styled PDF (MathJax/Mermaid supported).|
| `.md`           | `.html`  | Writes the rendered, styled HTML document.                   |
| `.md`           | `.epub`  | Packages the Markdown as a single-chapter EPUB.              |
| `.tex`          | `.pdf`   | Compiles LaTeX to PDF with the built-in compiler.            |
| `.html`/`.htm`  | `.pdf`   | Prints an existing HTML file to PDF.                         |

Any other combination reports an "unsupported conversion" error and the rest of
the list keeps going.

## Multiple conversions

You can declare more than one `convert` line; each runs independently:

```scimon
convert "report.md"  > "report.pdf"
convert "paper.tex"  > "paper.pdf"
convert "notes.md"   > "notes.epub"
```

## When it runs

`convert` runs at the end of a list, after downloads, `.tex` → PDF conversions
and `merge`, so files produced earlier are available as inputs.

```scimon
path "downloads/"

downloads {
    https://raw.githubusercontent.com/owner/repo/main/README.md as "readme.md"
}

convert "downloads/readme.md" > "downloads/readme.pdf"
```

## Notes

- Paths are resolved relative to the directory you run Scimon from.
- `.md` → `.pdf`/`.html` honors the list's [`style`](./style.md) directive.
- `.md` → `.epub` uses the output file name as the title and the `@author`
  metadata as the author (falling back to `Scimon`).
- Rendering to PDF uses a headless Chromium, the same as the rest of Scimon.
