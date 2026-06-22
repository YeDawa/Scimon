# Server

The `server` variable starts Scimon's built-in web server right after a list
finishes running, so you can browse, preview, and download the generated files in
your browser without a separate command.

```scimon
server "8080"
```

The value is the port to listen on. The server serves the list's
[`path`](./download-block.md) directory (or the default downloads folder when no
`path` is set) and runs until you stop it with `Ctrl+C`.

## Example

```scimon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "paper.pdf"
}

server "8080"
```

Running this downloads the file and then serves `downloads/` at
`http://127.0.0.1:8080`.

A list can also be server-only — no `downloads` block required:

```scimon
path "downloads/"

server "8080"
```

## Notes

- The server is local only (binds to `127.0.0.1`).
- PDFs, images, and checksum files (`.sha256`, `.md5`, `.crc32`, …) open in an
  in-page lightbox; other files download or open inline.
- EPUB files (`.epub`) open in a built-in reader (powered by
  [epub.js](https://github.com/futurepress/epub.js)). It loads the reader from a CDN, so it
  needs network access the first time.
- It honors your system dark/light theme, with a toggle in the corner.
- This is the same server as the [`serve`](../commands.md#serve) command — use
  the command for an arbitrary directory, or `server "PORT"` to wire it into a list.

# Variables

You can declare reusable variables with `@var` and reference them anywhere in
the list with `${name}`. This keeps long, repetitive lists (shared URL bases,
common folders, recurring names) DRY.

A declaration is `@var`, a name, and a double-quoted value, each on its own line:

```scimon
@var name "value"
```

- **Name** — starts with a letter or `_`, followed by letters, digits or `_`.
- **Value** — any double-quoted string.

References use `${name}` and can appear inside URLs, strings or anywhere else.
The expansion happens before the list runs, so every block (`downloads`,
`commands`, …) and the web server see the already-resolved content.

## Example

```scimon
@var gist "https://gist.githubusercontent.com/Kremilly"
@var folder "downloads/"

path "${folder}"

downloads {
    ${gist}/da424.../math.tex as "math" !ignore
    ${gist}/2f4cf.../test.tex as "teste-math.pdf" !ignore
}
```

## Notes

- `@var` declarations are stripped from the list after they are collected, so
  they never affect downloads, validation or the served source.
- An unknown reference (e.g. `${missing}`) is left untouched rather than
  replaced with an empty string, making typos easy to spot.
- Variables are resolved in a single pass; a variable value is not itself
  scanned for `${...}` references.
