# Watermark PDFs

Stamp a watermark onto every page of a PDF with the `watermark` directive — as
**text** or an **image**.

## Text watermark

```scimon
watermark "report.pdf" "CONFIDENTIAL" > "report-wm.pdf"
```

The text is drawn in light gray, rotated ~45° across each page and rendered
semi-transparently so the page content stays readable.

## Image watermark

Add the `image` keyword before the path to stamp a logo/picture instead:

```scimon
watermark "report.pdf" image "logo.png" > "report-wm.pdf"
```

The image is centered, scaled to about 40% of the page width (aspect ratio
preserved) and drawn semi-transparently. PNG and JPEG are supported.

## Multiple watermarks

You can declare more than one `watermark` line; each runs independently:

```scimon
watermark "a.pdf" "DRAFT"        > "a-wm.pdf"
watermark "b.pdf" image "seal.png" > "b-wm.pdf"
```

## When it runs

`watermark` runs after downloads and the other PDF steps (`merge`, `split`,
`rotate`), so a freshly produced PDF can be stamped:

```scimon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "paper.pdf"
}

watermark "downloads/paper.pdf" "PREPRINT" > "downloads/paper-wm.pdf"
```

## Notes

- Paths are resolved relative to the directory you run Scimon from.
- The watermark is applied to **every** page.
- The source PDF is left untouched; the result is written to the output path.
- Text uses the built-in Helvetica font (no embedding needed).
- An unreadable input or image reports an error, and the rest of the list keeps
  going.
