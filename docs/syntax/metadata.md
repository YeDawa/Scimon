# Metadata

A list's metadata describes it as a package — it's used when publishing to
Monlib with `scimon push` and is ignored during a normal `run`. Like a crate's
`Cargo.toml`, metadata lives in a **`package.yml`** file next to your `.mon`
list.

```yaml
# package.yml
name: "Scimon"
version: "1.0.0"
description: "A collection of papers and resources."
author: "Kremilly"
license: "MIT"
privacy: "Public"
homepage: "https://kremilly.com"
```

| Field         | Description                                  |
| ------------- | -------------------------------------------- |
| `name`        | Package name.                                |
| `version`     | Version string (e.g. `"1.0.0"`).             |
| `description` | Short description of the list.               |
| `author`      | Author name.                                 |
| `license`     | License identifier (e.g. `"MIT"`).           |
| `privacy`     | Visibility, e.g. `"Public"` or `"Private"`.  |
| `homepage`    | Project or author homepage URL.              |

All fields are optional. `package.yml` is read from the same directory as the
`.mon` file being run.

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
