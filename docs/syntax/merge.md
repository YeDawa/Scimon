# Merge PDFs

Combine several PDF files into a single document with the `merge` directive. It
takes a [glob](https://en.wikipedia.org/wiki/Glob_(programming)) pattern and an
output path, separated by `>`:

```scimon
merge "papers/*.pdf" > "compilado.pdf"
```

This matches every `.pdf` inside `papers/`, concatenates them in alphabetical
order, and writes the result to `compilado.pdf`.

## Multiple merges

You can declare more than one `merge` line; each produces its own output:

```scimon
merge "papers/*.pdf"  > "papers.pdf"
merge "reports/*.pdf" > "reports.pdf"
```

## When it runs

`merge` runs at the end of a list, after downloads (and any `.tex` → PDF
conversions) finish, so the generated files are already on disk and can be
picked up by the pattern.

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
