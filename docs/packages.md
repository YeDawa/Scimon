# Packages

A Scimon **package** is a single distributable file — a **`.scpkg`** bundle — that
carries a list and everything needed to share or publish it. It is a
gzip-compressed tar archive (like a Rust `.crate`) holding the manifest, the
license and the `.mon` lists.

## What's inside

A `.scpkg` contains **only**:

| File          | Description                                                       |
| ------------- | ---------------------------------------------------------------- |
| `package.yml` | The package [metadata](./syntax/metadata.md) manifest.           |
| `LICENSE`     | The license file sitting next to the list, when present.         |
| `*.mon`       | The entry list plus every list it pulls in through `import`.     |

Generated output (the folder declared with [`path`](./syntax/what-is.md)) and
other referenced assets are **not** included — a package ships source lists, not
build artifacts. Internally the bundle also records which list is the entry, so it
always runs the right one.

## `pack`

Build a bundle from an entry list:

```shell
scimon pack scimon.mon
```

Scimon reads the `package.yml` next to the list, gathers the entry and its
imported `.mon` lists (followed transitively), adds the license, and writes
`<name>-<version>.scpkg`. The file name is always **lowercase**; `name` and
`version` come from `package.yml`. Without a `version` the file is just
`<name>.scpkg`, and without a `name` it falls back to the list's file name.

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
- See [Metadata](./syntax/metadata.md) for the `package.yml` fields and
  [Import](./syntax/import.md) for how lists pull in one another.
