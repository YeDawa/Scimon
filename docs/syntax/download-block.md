# Download's Block

### URL List

You can specify multiple URLs for downloading files. Each URL should be placed on a new line. Optionally, you can append `!ignore` to a URL to indicate that it should be skipped during the download process.

#### Example Usage:

```scimon        
downloads {
    https://example.com/file1.pdf !ignore
    https://example.com/file2.pdf
    https://example.com/file3.pdf !ignore
    https://example.com/file4.pdf
    https://example.com/file5.pdf !ignore
    https://example.com/file6.pdf
}
```

In this example:

- `https://example.com/file1.pdf` will be skipped because it is followed by `!ignore`.
- `https://example.com/file2.pdf` will be downloaded.
- `https://example.com/file3.pdf` will be skipped because it is followed by `!ignore`.
- `https://example.com/file4.pdf` will be downloaded.
- `https://example.com/file5.pdf` will be skipped because it is followed by `!ignore`.
- `https://example.com/file6.pdf` will be downloaded.

### Special URLs

Some URLs aren't just downloaded — Scimon recognizes them and applies a dedicated strategy automatically:

- **Provider links** (ChatGPT/Gemini share links, Wikipedia, Arxiv, …) are fetched and rendered to PDF. See [Providers](../providers.md).
- **`.tex` files** are compiled to PDF on the fly. See the [Compiler](../compile.md).
- **Markdown sources** (`.md`) are rendered to PDF — or, when you rename the entry with an `.epub` name, packaged as an EPUB instead.

You can still combine these with `as "name.pdf"` and `!ignore`.

### EPUB output

A Markdown URL can be turned into an EPUB by giving the entry an `.epub` name with `as`:

```scimon
downloads {
    https://raw.githubusercontent.com/owner/repo/main/README.md as "book.epub"
}
```

The book title comes from the output file name and the author is set to `Scimon`.

### Path Configuration

You can specify the directory where the downloaded files should be stored by setting the `path` variable. This ensures that all files are saved in the specified folder in your file system.

#### Example Usage:

```scimon
path "path/to/folder"
```

In this example:

- All downloaded files will be stored in the directory `path/to/folder`.

### Groups (subfolders)

Organize a list into subdirectories with `group "name" { ... }` inside the
`downloads` block. Files declared in a group are saved under that subfolder of
`path`:

```scimon
path "downloads/"

downloads {
    https://example.com/intro.pdf

    group "papers" {
        https://arxiv.org/pdf/2203.08877 as "a.pdf"
        https://arxiv.org/pdf/2405.01513 as "b.pdf"
    }

    group "readmes" {
        https://raw.githubusercontent.com/facebook/react/main/README.md
    }
}
```

This produces:

```
downloads/intro.pdf
downloads/papers/a.pdf
downloads/papers/b.pdf
downloads/readmes/README.md
```

Notes:

- Groups may be nested (`group "a" { group "b" { ... } }` → `path/a/b/`).
- The subfolder is created automatically.
- Everything else — `as "name"`, the macros, `||` fallbacks and ranges — works
  the same inside a group.

### Ignoring Specific URLs

The `!ignore` macro allows you to skip specific URLs in your download list. This is useful if you have certain files that you do not want to download during a particular operation.

#### Example Usage:

```scimon
https://example.com/file1.pdf !ignore
```

In this example:

- The URL `https://example.com/file1.pdf` will be omitted from the download process because it is followed by the `!ignore` directive.

### Line macros

Macros are `!`-prefixed flags appended to a download line. They can be combined
with `as "name"` and with each other.

| Macro        | Effect                                                                                  |
| ------------ | --------------------------------------------------------------------------------------- |
| `!ignore`    | Skip this line entirely.                                                                |
| `!only`      | Focus mode — when **any** line carries `!only`, only the marked lines run (others skip).|
| `!no-qr`     | Exclude this line from QR code generation (when the `qrcode` directive is set).         |
| `!retry(N)`  | Re-attempt the download up to `N` extra times if it fails.                              |
| `!unzip`     | Extract the downloaded archive (`.zip`, `.tar.gz`, `.tgz`, `.tar`). Alias: `!extract`.  |

#### `!only` (focus mode)

Handy for testing a large list without commenting everything out: tag the
entries you want and leave the rest untouched.

```scimon
downloads {
    https://example.com/file1.pdf
    https://example.com/file2.pdf !only
    https://example.com/file3.pdf
    https://example.com/file4.pdf !only
}
```

Here only `file2.pdf` and `file4.pdf` are downloaded; `file1.pdf` and
`file3.pdf` are skipped. Covers, compression and QR codes follow suit, since
they only act on what was actually downloaded.

#### `!no-qr`

When the [`qrcode`](./qrcode.md) directive is set, Scimon makes a QR code for
every download. Append `!no-qr` to opt a single entry out:

```scimon
qrcode "downloads/qrcodes/"

downloads {
    https://example.com/file1.pdf
    https://example.com/file2.pdf !no-qr
}
```

`file2.pdf` is still downloaded, but no QR code is generated for it.

#### `!retry(N)`

Network downloads can fail transiently. Append `!retry(N)` to give an entry up
to `N` extra attempts before it's reported as failed:

```scimon
downloads {
    https://example.com/flaky.pdf !retry(3)
    https://example.com/also.pdf as "also.pdf" !retry(5)
}
```

`flaky.pdf` is attempted up to 4 times total (1 + 3 retries). Each retry prints
a `Retrying (n/N)` notice. A failed download stays non-fatal — the rest of the
list keeps going. Retries apply to the actual file download; it combines freely
with `as "name"` and the other macros.

#### `!unzip` (or `!extract`)

Download an archive and unpack it. Supported formats: `.zip`, `.tar.gz`, `.tgz`
and `.tar`.

```scimon
downloads {
    https://example.com/dataset.zip !unzip
}
```

The archive is downloaded, then extracted into a folder named after it next to
the file — `dataset.zip` unpacks into `dataset/`. `!unzip` also forces the
download of non-PDF archives (which are otherwise skipped), and combines with
`as "name"`, `!retry(N)` and `||` fallbacks. The archive itself is kept.

List mirrors for the same file by separating URLs with `||`. Scimon tries them
left to right and stops at the first one that downloads successfully:

```scimon
downloads {
    https://primary.example/file.pdf || https://mirror.example/file.pdf as "file.pdf"
}
```

- The `as "name"` and any macros apply to the whole entry and go after the last
  URL.
- When a candidate fails, a `... failed, trying fallback ...` notice is printed
  and the next URL is attempted; only when all fail is the entry reported failed.
- Combines with `!retry(N)`: each candidate is retried `N` times before moving
  on to the next.

```scimon
downloads {
    https://primary.example/a.pdf || https://mirror.example/a.pdf as "a.pdf" !retry(2)
}
```

Fallback applies to direct file downloads (the common case: PDF mirrors).

### URL ranges

A numeric range `{A..B}` expands one line into several — one download per value:

```scimon
downloads {
    https://arxiv.org/pdf/{2203.08877..08880}
}
```

This downloads `2203.08877`, `2203.08878`, `2203.08879` and `2203.08880`. The
width of `B` sets the zero-padding and the non-counter part of `A` is kept as a
prefix, so `{1..3}` yields `1, 2, 3` and `{08..10}` yields `08, 09, 10`.

The token is replaced **everywhere it appears on the line**, so repeat it in the
name to keep each file unique:

```scimon
downloads {
    https://example.com/vol-{1..3}.pdf as "volume-{1..3}.pdf"
}
```

This produces `volume-1.pdf`, `volume-2.pdf` and `volume-3.pdf`. A fixed `as`
name without the range would make every copy collide, so either include the
range in the name or leave the entry to be auto-named.

Notes:

- Only ascending ranges expand; very large ranges are rejected to guard against
  typos.
- Ranges combine with `as`, the macros and `||` fallbacks (each expanded line is
  processed independently).

#### Auto-renaming Files

When downloading files, you can also specify a custom name for the downloaded file using the `as` variable. This allows you to rename the file as it is saved to your system.

#### Example Usage:

```scimon
https://example.com/file1.pdf as "new_name.pdf"
```

In this example:

- The URL `https://example.com/file1.pdf` will be downloaded and saved as `new_name.pdf` instead of its original name.

### Pipes (filters)

You can chain PDF post-processing filters directly on a download entry using the pipe (`|`) character. Supported filter operations include:
- `rotate <angle>`: Rotates the PDF pages by the specified angle (must be a multiple of 90).
- `watermark "<text>"`: Adds a semi-transparent text watermark.
- `watermark image "<path>"`: Stamps the PDF pages with an image.

#### Example Usage:

```scimon
downloads {
    https://example.com/document.pdf as "final.pdf" | rotate 90 | watermark "CONFIDENTIAL"
}
```

In this example:
- The file is downloaded and compiled/saved as `final.pdf`.
- It is then rotated by 90 degrees.
- Finally, it is stamped with a semi-transparent text watermark reading `"CONFIDENTIAL"`.

### Summary

1. **Download URLs**: List URLs line by line. Append `!ignore` to skip specific URLs.

   ```scimon
   downloads {
       https://example.com/file1.pdf !ignore
       https://example.com/file2.pdf
   }
   ```
2. **Set Download Directory**: Define where the files should be saved using the `path` variable.

   ```scimon
   path "path/to/folder"
   ```
3. **Skip Specific URLs**: Use `!ignore` to bypass certain URLs.

   ```scimon
   https://example.com/file1.pdf !ignore
   ```

By following these instructions, you can efficiently manage your download list, specify storage directories, and selectively ignore certain files.
