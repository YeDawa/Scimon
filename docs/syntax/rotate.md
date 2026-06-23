# Rotate PDFs

Rotate every page of a PDF with the `rotate` directive. It takes the input PDF,
an angle in degrees, and an output path:

```scimon
rotate "scan.pdf" 90 > "scan-rotated.pdf"
```

## Angle

- Must be a multiple of `90` (`90`, `180`, `270`, …).
- Can be negative (`-90` rotates counter-clockwise).
- It is **added** to each page's current rotation and normalized into the
  `0..360` range, so rotating an already-rotated page accumulates as expected.

## Multiple rotations

You can declare more than one `rotate` line; each runs independently:

```scimon
rotate "a.pdf" 90  > "a-rot.pdf"
rotate "b.pdf" 180 > "b-rot.pdf"
```

## When it runs

`rotate` runs after downloads, `merge` and `split`, so a freshly produced PDF
can be rotated:

```scimon
path "downloads/"

downloads {
    https://example.com/landscape.pdf as "doc.pdf"
}

rotate "downloads/doc.pdf" 90 > "downloads/doc-portrait.pdf"
```

## Notes

- Paths are resolved relative to the directory you run Scimon from.
- All pages are rotated by the same angle.
- The source PDF is left untouched; the result is written to the output path.
- A non-multiple-of-90 angle or an unreadable input reports an error, and the
  rest of the list keeps going.
