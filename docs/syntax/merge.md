# Merge PDFs

Combine several PDF files into a single document with the `merge` directive. It
takes a [glob](https://en.wikipedia.org/wiki/Glob_(programming)) pattern and an
output path, separated by `>`:

```scimon
merge "papers/*.pdf" > "compiled.pdf"
```

This matches every `.pdf` inside `papers/`, concatenates them in alphabetical
order, and writes the result to `compiled.pdf`.

## Explicit list

Instead of a glob, you can pass an explicit list of files in `[ ... ]`. They are
merged in **exactly the order given** (not sorted), which is handy when order
matters:

```scimon
merge [ "intro.pdf", "chapter1.pdf", "appendix.pdf" ] > "book.pdf"
```

Items are comma-separated and may use single or double quotes.

## Multiple merges

You can declare more than one `merge` line; each produces its own output:

```scimon
merge "papers/*.pdf"  > "papers.pdf"
merge "reports/*.pdf" > "reports.pdf"
```

## When it runs

`merge` runs after downloads (including `.tex` → PDF conversions) and after the
`ai` block, so every generated file is already on disk and can be picked up by
the pattern. It also runs **before** `covers` and `compress`, so the combined
PDF gets a cover extracted and is included in the archive.

The processing order is:

```
downloads → ai → merge → covers → compress → qrcode → math → convert
```

```scimon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "a.pdf"
    https://arxiv.org/pdf/2405.01513 as "b.pdf"
}

merge "downloads/*.pdf" > "downloads/all.pdf"
```

## Notes

- The pattern is resolved relative to the directory you run Scimon from.
- Files that aren't readable PDFs are skipped; if nothing valid matches, the
  merge reports an error and the rest of the list keeps going.
- Output is a fresh PDF — the source files are left untouched.
- Bookmarks/outlines from the source PDFs are not carried over.
