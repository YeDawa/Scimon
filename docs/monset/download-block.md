# Download's Block

### URL List

You can specify multiple URLs for downloading files. Each URL should be placed on a new line. Optionally, you can append `!ignore` to a URL to indicate that it should be skipped during the download process.

#### Example Usage:

```plaintext
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

### Path Configuration

You can specify the directory where the downloaded files should be stored by setting the `path` variable. This ensures that all files are saved in the specified folder in your file system.

#### Example Usage:

```plaintext
path "path/to/folder"
```

In this example:

- All downloaded files will be stored in the directory `path/to/folder`.

### Ignoring Specific URLs

The `!ignore` macro allows you to skip specific URLs in your download list. This is useful if you have certain files that you do not want to download during a particular operation.

#### Example Usage:

```plaintext
https://example.com/file1.pdf !ignore
```

In this example:

- The URL `https://example.com/file1.pdf` will be omitted from the download process because it is followed by the `!ignore` directive.

#### Auto-renameing Files

When downloading files, you can also specify a custom name for the downloaded file using the `as` variable. This allows you to rename the file as it is saved to your system.

#### Example Usage:

```plaintext
https://example.com/file1.pdf as "new_name.pdf"
```

In this example:

- The URL `https://example.com/file1.pdf` will be downloaded and saved as `new_name.pdf` instead of its original name.

### Summary

1. **Download URLs**: List URLs line by line. Append `!ignore` to skip specific URLs.

   ```plaintext
   downloads {
       https://example.com/file1.pdf !ignore
       https://example.com/file2.pdf
   }
   ```
2. **Set Download Directory**: Define where the files should be saved using the `path` variable.

   ```plaintext
   path "path/to/folder"
   ```
3. **Skip Specific URLs**: Use `!ignore` to bypass certain URLs.

   ```plaintext
   https://example.com/file1.pdf !ignore
   ```

By following these instructions, you can efficiently manage your download list, specify storage directories, and selectively ignore certain files.
