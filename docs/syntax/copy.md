# Copy folder

To copy the folder with the final files to another location, use the `copy`
variable and assign the destination path as its value. Scimon copies everything
inside the [`path`](./what-is.md) folder (preserving subdirectories) into the
destination, creating it if it does not exist.

```scimon
path "downloads/"

copy "backup/"
```

In this example, after all files are produced, the contents of `downloads/` are
copied into `backup/`. The destination can be any absolute or relative path:

```scimon
copy "D:/archive/papers"
```

The copy runs at the end of the process, so the generated PDFs, rendered
Markdown and AI files are all included.
