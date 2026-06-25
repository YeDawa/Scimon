# Packages

A Scimon **package** is a single distributable file — a **`.scpkg`** bundle — that
carries a list and everything needed to share or publish it. It is a
gzip-compressed tar archive (like a Rust `.crate`) holding the manifest, the
license and the `.mon` lists.

## What's inside

A `.scpkg` contains **only**:

| File            | Description                                                   |
| --------------- | ------------------------------------------------------------- |
| `package.yml` | The package [metadata](./syntax/metadata.md) manifest.          |
| `LICENSE`     | The license file sitting next to the list, when present.      |
| `*.mon`       | The entry list plus every list it pulls in through`import`. |

Generated output (the folder declared with [`path`](./syntax/what-is.md)) and
other referenced assets are **not** included — a package ships source lists, not
build artifacts. Internally the bundle also records which list is the entry, so it
always runs the right one.

## `init`

Scaffold a new package in the current directory:

```shell
scimon init
```

It creates three files (leaving any that already exist untouched):

| File           | Description                                                              |
| -------------- | ------------------------------------------------------------------------ |
| `scimon.yml` | A package descriptor template (`name`, `description`, `author`…). |
| `main.mon`   | The entry list, ready to edit.                                           |
| `.entry`     | Records the entry list (`main.mon`) so `pack` knows what to ship.    |

## `pack`

Build a bundle from an entry list:

```shell
scimon pack scimon.mon   # explicit entry
scimon pack              # use the entry recorded in .entry
```

With no argument, `pack` reads the project's `.entry` (created by `init`) to find
the entry list. Scimon reads the `package.yml` next to the list, gathers the entry
and its imported `.mon` lists (followed transitively), adds the license, and writes
`<name>-<version>.scpkg`. The file name is **slugified** (lowercase, with spaces
and other characters turned into hyphens); `name` and `version` come from
`package.yml`. Without a `version` the file is just `<name>.scpkg`, and without a
`name` it falls back to the list's file name.

```
my-list/
├── package.yml      # name: "demo", version: "1.0.0"
├── LICENSE
├── main.mon         # import "lib.mon"
└── lib.mon
```

```shell
scimon pack main.mon
# → demo.scpkg   (package.yml, LICENSE, main.mon, lib.mon)
```

## `info`

Inspect a bundle's metadata and contents **without extracting it**:

```shell
scimon info demo-1.0.0.scpkg
```

It reads the manifest and file list straight from the archive and prints the
metadata, the entry list and the packed files:

```
PACKAGE INFO
  Name:        demo
  Version:     1.0.0
  Description: A collection of papers.
  Author:      YeDawa
  License:     MIT
  Homepage:    https://scimon.dev
  Entry:       main.mon
  Files:       4
    - main.mon
    - package.yml
    - LICENSE
    - lib.mon
```

## Running a bundle directly

`run` also accepts a `.scpkg`: it extracts the bundle and immediately runs the
entry list, just like running a plain `.mon` file.

```shell
scimon run demo.scpkg
```

## Notes

- The **entry** is the list you pass to `pack`; it is the one executed on
  `run`.
- Imports are followed **transitively** — a list imported by an imported list is
  packed too.
- Remote (`http(s)`) and [Monlib](https://monlib.net) imports are resolved at run
  time, so they are not packed.
- `init` writes the descriptor to `scimon.yml`, but the metadata read when packing
  comes from `package.yml` — see [Metadata](./syntax/metadata.md).
- See [Import](./syntax/import.md) for how lists pull in one another.
