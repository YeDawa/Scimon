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

The book title comes from the output file name and the author from the `@author` metadata (falling back to `Scimon`).

### Path Configuration

You can specify the directory where the downloaded files should be stored by setting the `path` variable. This ensures that all files are saved in the specified folder in your file system.

#### Example Usage:

```scimon
path "path/to/folder"
```

In this example:

- All downloaded files will be stored in the directory `path/to/folder`.

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

| Macro      | Effect                                                                                  |
| ---------- | --------------------------------------------------------------------------------------- |
| `!ignore`  | Skip this line entirely.                                                                |
| `!only`    | Focus mode — when **any** line carries `!only`, only the marked lines run (others skip).|
| `!no-qr`   | Exclude this line from QR code generation (when the `qrcode` directive is set).         |

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

#### Auto-renameing Files

When downloading files, you can also specify a custom name for the downloaded file using the `as` variable. This allows you to rename the file as it is saved to your system.

#### Example Usage:

```scimon
https://example.com/file1.pdf as "new_name.pdf"
```

In this example:

- The URL `https://example.com/file1.pdf` will be downloaded and saved as `new_name.pdf` instead of its original name.

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
