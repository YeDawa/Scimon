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
- ZIP files (`.zip`) open a viewer that lists their contents (names and sizes)
  **without extracting** the archive.
- EPUB files (`.epub`) open in a built-in reader (powered by
  [epub.js](https://github.com/futurepress/epub.js)). It loads the reader from a CDN, so it
  needs network access the first time.
- It honors your system dark/light theme, with a toggle in the corner.
- This is the same server as the [`serve`](../usage/commands.md#serve) command — use
  the command for an arbitrary directory, or `server "PORT"` to wire it into a list.
