# Metadata

A Monset list can declare metadata at the top of the file. These variables
describe the list as a package — they're used when publishing to Monlib with
`scimon push` — and are ignored during a normal `run`.

| Variable       | Description                                  |
| -------------- | -------------------------------------------- |
| `@name`        | Package name.                                |
| `@version`     | Version string (e.g. `"1.0.0"`).             |
| `@description` | Short description of the list.               |
| `@author`      | Author name.                                 |
| `@license`     | License identifier (e.g. `"MIT"`).           |
| `@privacy`     | Visibility, e.g. `"Public"` or `"Private"`.  |
| `@homepage`    | Project or author homepage URL.              |

## Example

```scimon
@name "Scimon"
@version "1.0.0"
@description "A collection of papers and resources."
@author "Kremilly"
@license "MIT"
@privacy "Public"
@homepage "https://kremilly.com"

path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "arxiv_paper.pdf"
}
```

Each value is a double-quoted string and each declaration goes on its own line.
