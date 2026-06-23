# Import

`import` brings another `.mon` list into the current one. The source can be a
**local file**, a **remote URL**, or a **Monlib package** — in every case its
contents are fetched and **spliced in place**, so you can compose large lists
from smaller, shareable pieces.

```scimon
import "common.mon"                                  # local file
import "https://example.com/shared.mon"              # remote URL
import "base-pack"                                   # Monlib package
```

How the source is resolved:

| Source             | Resolved as                                              |
| ------------------ | -------------------------------------------------------- |
| starts with `http` | fetched over HTTP                                         |
| an existing path   | read from the local filesystem                           |
| anything else      | a package name pulled from [Monlib](https://monlib.net)  |

## How it works

`import` runs as a preprocessing step — before functions, loops and variables —
so an imported file's `@fn`/`@var` definitions are shared with the rest of the
document, and vice versa. Imports may nest (an imported list can `import`
others); cycles are detected and skipped.

```scimon
import "headers.mon"

path "downloads/"

downloads {
    @arxiv("2203.08877", "paper.pdf")   # @fn arxiv comes from headers.mon
}
```

## Monlib packages

A bare name (no path, no URL) is pulled from Monlib, which needs a valid
`MONLIB_API_KEY` in your [`.env` file](../configs/env-file.md):

```ini
MONLIB_API_KEY="your-key"
```

If the key is missing, the Monlib import is skipped and the rest of the list
still runs.

## Notes

- Local paths and the document being imported are resolved relative to the
  directory you run Scimon from.
- A source that can't be loaded (missing file, network error, unknown package)
  is skipped rather than aborting the run.
- Because imports are spliced, the imported list shares the current list's
  context (`path`, blocks, …) — mind duplicate directives like `server`.
