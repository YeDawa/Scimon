# Split PDFs

Split a PDF into one file per page with the `split` directive — the inverse of
[`merge`](./merge.md). It takes the input PDF and an output template, separated
by `>`:

```scimon
split "report.pdf" > "pages/page-{n}.pdf"
```

`{n}` in the template is replaced by the page number, producing
`pages/page-1.pdf`, `pages/page-2.pdf`, and so on.

## Output naming

- With a `{n}` placeholder, it's replaced by the page number anywhere in the
  path: `"out/{n}.pdf"`, `"chapter-{n}.pdf"`, …
- Without `{n}`, the page number is inserted before the extension:
  `"out.pdf"` → `out-1.pdf`, `out-2.pdf`, …

## Multiple splits

You can declare more than one `split` line; each runs independently:

```scimon
split "a.pdf" > "a/{n}.pdf"
split "b.pdf" > "b/{n}.pdf"
```

## When it runs

`split` runs after downloads and `merge`, so a freshly downloaded or merged PDF
can be split:

```scimon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "paper.pdf"
}

split "downloads/paper.pdf" > "downloads/pages/page-{n}.pdf"
```

## Notes

- Paths are resolved relative to the directory you run Scimon from.
- Each output keeps the page's own resources (fonts, images), so pages render
  on their own.
- The source PDF is left untouched.
- A missing or unreadable input reports an error and the rest of the list keeps
  going.
