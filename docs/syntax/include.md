# Include

Split a large list across several files and stitch them together with
`include`. It reads a **local** `.mon` file and splices its content in place:

```scimon
include "common.mon"
```

`include` runs as a preprocessing step — before functions and variables — so an
included file's `@fn` and `@var` definitions are shared with the rest of the
document, and vice versa.

## Example

`headers.mon`:

```scimon
@var gist "https://gist.githubusercontent.com/Kremilly"

@fn arxiv(id, name) {
    https://arxiv.org/pdf/${id} as "${name}"
}
```

`list.mon`:

```scimon
include "headers.mon"

path "downloads/"

downloads {
    @arxiv("2203.08877", "paper.pdf")
    ${gist}/abc123/notes.tex as "notes.pdf"
}
```

Running `list.mon` behaves as if the contents of `headers.mon` were pasted at
the top, so the `@arxiv` function and `${gist}` variable resolve normally.

## `include` vs `import`

- **`include "file.mon"`** splices a **local** file into the current list
  (this page).
- **[`import "package"`](./import.md)** pulls and runs a shared package from
  [Monlib](https://monlib.net).

## Notes

- Paths are resolved relative to the directory you run Scimon from.
- Includes may nest (an included file can `include` another); cycles are
  detected and skipped.
- A missing include is skipped rather than aborting the run.
- Comments in an included file are stripped, just like the main file.
