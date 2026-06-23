# Functions

Functions are reusable, parametrized templates. You define one with `fn` and a
parameter list, then call it as `name(args)` — each call is replaced by the
function body with `${param}` filled in. They build on
[variables](./variables.md): a function body can use both its parameters and
global `@var`s.

## Defining and calling

```scimon
fn arxiv(id, name) {
    https://arxiv.org/pdf/${id} as "${name}" !retry(3)
}

downloads {
    arxiv("2203.08877", "paper1.pdf")
    arxiv("2405.01513", "paper2.pdf")
}
```

After expansion this is exactly:

```scimon
downloads {
    https://arxiv.org/pdf/2203.08877 as "paper1.pdf" !retry(3)
    https://arxiv.org/pdf/2405.01513 as "paper2.pdf" !retry(3)
}
```

- **Definition** — `fn name(p1, p2, …) { ... }`. The name starts with a letter
  or `_`; the body can span multiple lines.
- **Call** — `name("a", "b")`. Arguments are comma-separated; surrounding double
  quotes are stripped.
- **Parameters** — referenced in the body with `${param}`, the same syntax as
  variables.

## Combining with variables

A function body and its arguments can reference global variables:

```scimon
@var gist "https://gist.githubusercontent.com/Kremilly"

fn doc(path, name) {
    ${gist}/${path} as "${name}"
}

downloads {
    doc("abc123/file.tex", "file.pdf")
}
```

Functions are expanded first, then variables — so the `${gist}` left behind by
the call is resolved afterwards.

## Notes

- `fn` definitions are removed after expansion, so they don't affect downloads,
  validation or the served source.
- Functions may call other functions; expansion repeats until stable (bounded,
  so a self-referential definition can't loop forever).
- A call to an undefined function (e.g. `missing(...)`) is left untouched,
  making typos easy to spot.
- A missing argument expands to an empty string.
