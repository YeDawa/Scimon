# Packages

A Scimon **package** is a single distributable file — a **`.scpkg`** bundle — that
carries a list and everything needed to share or publish it. It is a
gzip-compressed tar archive (like a Rust `.crate`). The entry list is always
**`main.mon`**.

## What's inside

A `.scpkg` contains **only** the source:

| File          | Description                                                       |
| ------------- | ---------------------------------------------------------------- |
| `package.yml` | The package [metadata](./syntax/metadata.md) manifest.           |
| `LICENSE`     | The license file sitting next to the list, when present.         |
| `README`      | The README file sitting next to the list, when present.          |
| `main.mon`    | The entry list — always run first.                               |
| `*.mon`       | Every other list `main.mon` pulls in through `import`.           |

Generated output (the folder declared with [`path`](./syntax/what-is.md)) and
other referenced assets are **not** included — a package ships source lists, not
build artifacts.

## `init`

Scaffold a new package. `init` asks for the manifest fields and creates a new
folder named after the package:

```shell
scimon init
```

```
Package name: (my-package) demo
Description:  (A Scimon package.) A collection of papers
Version:      (0.1.0) 1.0.0
Author:       YeDawa
License:      (MIT)
Homepage:
Privacy:      (Public)
```

It writes four files into `demo/` (leaving any that already exist untouched):

| File          | Description                                                       |
| ------------- | ---------------------------------------------------------------- |
| `package.yml` | The manifest, filled from your answers.                          |
| `main.mon`    | The entry list, ready to edit.                                   |
| `README.md`   | `# <name>` plus the description.                                 |
| `LICENSE`     | The chosen license's full text, downloaded from the SPDX list with the year and author filled in. |

## `pack`

Bundle the current package — its `main.mon`, manifest, license, README and every
imported `.mon` list — into a `.scpkg`:

```shell
scimon pack
```

Scimon reads `main.mon` from the current directory, gathers the lists it imports
(followed transitively), and writes `<name>-<version>.scpkg`. The file name is
**slugified** (lowercase, with spaces and other characters turned into hyphens);
`name` and `version` come from `package.yml`. Without a `version` the file is just
`<name>.scpkg`.

```
demo/
├── package.yml      # name: "demo", version: "1.0.0"
├── LICENSE
├── README.md
├── main.mon         # import "lib.mon"
└── lib.mon
```

```shell
cd demo && scimon pack
# → demo-1.0.0.scpkg   (package.yml, LICENSE, README.md, main.mon, lib.mon)
```

## `info`

Inspect a bundle's metadata and contents **without extracting it**:

```shell
scimon info demo-1.0.0.scpkg
```

It reads the manifest and file list straight from the archive and prints the
metadata, the entry and the packed files:

```
PACKAGE INFO
  Name:        demo
  Version:     1.0.0
  Description: A collection of papers.
  Author:      YeDawa
  License:     MIT
  Homepage:    https://scimon.dev
  Entry:       main.mon
  Files:       5
    - main.mon
    - package.yml
    - LICENSE
    - README.md
    - lib.mon
```

## Running a bundle

`run` also accepts a `.scpkg`: it extracts the bundle into a folder named after it
and runs `main.mon` from there, just like running a plain list.

```shell
scimon run demo-1.0.0.scpkg
```

If the bundle has no `main.mon`, `run` reports an error.

## `pull`

Download a package from [Monlib](https://monlib.net), run it, and drop the
downloaded archive — leaving only the extracted package:

```shell
scimon pull demo            # latest version
scimon pull demo@1.0.0      # a specific version
```

`pull demo` fetches the latest non-yanked version; `pull demo@1.0.0` pins an exact
one. A private package can only be pulled by its owner.

## Notes

- The entry is **always `main.mon`** — `pack` packs it, and `run`/`pull` execute it.
- Imports are followed **transitively** — a list imported by an imported list is
  packed too.
- Remote (`http(s)`) and Monlib imports are resolved at run time, so they are not
  packed.
- See [Metadata](./syntax/metadata.md) for the `package.yml` fields and
  [Import](./syntax/import.md) for how lists pull in one another.
