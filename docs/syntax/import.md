# Import

The `import` variable pulls a package from [Monlib](https://monlib.net) and runs
it **before** the current list. Use it to reuse a shared package (a common set of
downloads, scripts, or a README) across several lists.

```scimon
import "package"

path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "paper.pdf"
}
```

In the example, the `base-pack` package is fetched from Monlib and executed
first, then the rest of the list runs.

## Multiple imports

You can import several packages; they run in the order they are declared:

```scimon
import "package"
```

## Requirements

Imports are resolved through the Monlib API, so a valid `MONLIB_API_KEY` must be
set in your [`.env` file](../configs/env-file.md):

```ini
MONLIB_API_KEY="your-key"
```

If the key is missing, the imports are skipped with a clear message and the main
list still runs.

## Notes

- Each imported package runs with its own settings (its own `path`, blocks, etc.).
- Imports are resolved one level deep: a package imported this way does not, in
  turn, process its own `import` lines.
- Packages are validated before running, the same way `scimon pull` does.
