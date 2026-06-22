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
