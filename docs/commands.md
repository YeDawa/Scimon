# Commands

Scimon is driven by a small set of subcommands. The general form is:

```shell
scimon <command> [arguments] [flags]
```

## `run`

Execute a Scimon list — runs the `downloads {}` block, the `commands {}` block,
and renders any `readme`. Accepts a local path or a remote URL.

```shell
scimon run scimon.mon
scimon run https://example.com/scimon.mon
```

See [Basic usage](./basic-usage.md) and the [language syntax](./syntax/what-is.md).

## `compile`

Compile a LaTeX (`.tex`) file into a PDF. Accepts a local path or a URL, with an
optional `-o` / `--output`.

```shell
scimon compile paper.tex
scimon compile paper.tex -o build/final.pdf
```

See the [LaTeX Compiler](./compile.md) page for the full feature list.

## `scrape`

Discover downloadable documents on a page and fetch them.

```shell
scimon scrape https://example.com
```

See [Scrape](./scrape.md).

## `pull`

Pull a Scimon list (and its referenced assets) from a remote location into your
workspace.

```shell
scimon pull my-list
```

## `push`

Publish a Scimon list. The list's metadata variables (`@name`, `@version`,
`@author`, …) describe the package being published. See
[Metadata](./syntax/metadata.md).

```shell
scimon push scimon.mon
```

## `options`

Manage configuration files (`scimon.yml` and `.env`).

| Option              | Action                                          |
| ------------------- | ----------------------------------------------- |
| `view-env`          | Print the current environment variables.        |
| `open-env`          | Open the `.env` file in your text editor.        |
| `open-settings`     | Open the `scimon.yml` file in your text editor.  |
| `write-env`         | Add a new environment variable interactively.    |
| `download-env`      | (Re)download the default `.env` file.            |
| `download-settings` | (Re)download the default `scimon.yml` file.      |

```shell
scimon options open-settings
scimon options view-env
```

See [Scimon.yml file](./configs/scimon.yml-file.md) and [.env file](./configs/env-file.md).

## `auth`

Authentication for Monlib.

```shell
scimon auth login
scimon auth logout
```

## `settings`

Sync your settings file with Monlib.

```shell
scimon settings pull
scimon settings push
```

## Global flags

These flags apply to every command:

| Flag             | Description                                            |
| ---------------- | ----------------------------------------------------- |
| `--no-ignore`    | Process every line, ignoring the `!ignore` directive.  |
| `--no-open-link` | Disable the `open` variable (don't open URLs).         |
| `--no-readme`    | Skip rendering `readme` blocks.                         |
| `--no-secure`    | Disable secure mode for the `commands {}` runner.      |

```shell
scimon run scimon.mon --no-ignore --no-readme
```
