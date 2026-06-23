# Metadata

A list's metadata describes it as a package — it's used when publishing to
Monlib with `scimon push` and is ignored during a normal `run`. Like a crate's
`Cargo.toml`, metadata lives in a **`package.yml`** file next to your `.mon`
list.

```yaml
# package.yml
name: "Scimon"
description: "A collection of papers and resources."
author: "YeDawa"
license: "MIT"
privacy: "Public"
homepage: "https://kremilly.com"
```

| Field         | Description                                    |
| ------------- | ---------------------------------------------- |
| `name`        | Package name.                                  |
| `description` | Short description of the list.                 |
| `author`      | Author name, or a list of authors.             |
| `license`     | License identifier (e.g.`"MIT"`).            |
| `privacy`     | Visibility, e.g.`"Public"` or `"Private"`. |
| `homepage`    | Project or author homepage URL.                |

All fields are optional. `package.yml` is read from the same directory as the
`.mon` file being run.

## Multiple authors

`author` may be a single value or a list — and `authors` works as an alias:

```yaml
author:
  - "YeDawa"
  - "Another Dev"

# inline list form
author: ["YeDawa", "Another Dev"]

# alias
authors: ["YeDawa", "Another Dev"]
```

A list is joined into a single comma-separated value (`Kremilly, Another Dev`).

## Example layout

```
my-list/
├── package.yml
└── scimon.mon
```

```scimon
# scimon.mon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "arxiv_paper.pdf"
}
```

The `author` is reused elsewhere too — for example as the EPUB author when an
`ai`/download entry produces an `.epub`.
