# Loops

Repeat a block of lines once per value with `for`. It's a template expansion
(like [functions](./functions.md) and [variables](./variables.md)): the body is
duplicated for each item, with `${name}` replaced by the current value.

## List form

```scimon
downloads {
    for id in ["2203.08877", "2405.01513"] {
        https://arxiv.org/pdf/${id} as "${id}.pdf"
    }
}
```

Expands to:

```scimon
downloads {
    https://arxiv.org/pdf/2203.08877 as "2203.08877.pdf"
    https://arxiv.org/pdf/2405.01513 as "2405.01513.pdf"
}
```

Items are comma-separated; surrounding double quotes are stripped.

## Range form

You can iterate a numeric [range](./download-block.md) instead of a list:

```scimon
downloads {
    for i in {1..3} {
        https://example.com/vol-${i}.pdf as "vol-${i}.pdf"
    }
}
```

This produces `vol-1.pdf`, `vol-2.pdf` and `vol-3.pdf`.

## Combining with variables and functions

The loop runs before variables and functions, so the body can use global
`@var`s and call `@fn`s:

```scimon
@var gist "https://gist.githubusercontent.com/Kremilly"

@fn doc(slug, name) {
    ${gist}/${slug} as "${name}"
}

downloads {
    for n in ["intro", "guide"] {
        @doc("abc/${n}.md", "${n}.pdf")
    }
}
```

## Notes

- Loops may be nested, and a `for` can sit at the top level or inside a block
  (e.g. `downloads { ... }`).
- `for` definitions are expanded away before the list runs, so they don't appear
  in the served source or `parse.json`.
- The loop variable only substitutes inside its own body.
