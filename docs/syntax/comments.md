# Comments

Scimon lets you annotate a list or temporarily disable parts of it with
comments. They are stripped before the list is interpreted, so they never affect
the result.

## Line comments

Use `//` to comment until the end of the line. It works on its own line or after
a statement:

```scimon
// where the downloaded files are saved
path "downloads/"   // trailing comment
```

## Block comments

Use `/* ... */` for a comment that can span multiple lines:

```scimon
/*
   This list downloads a couple of papers
   and renders them to PDF.
*/
path "downloads/"
```

A block comment can also appear inline:

```scimon
path /* output folder */ "downloads/"
```

## Rules

- A `//` or `/*` is only treated as a comment when it is at the **start of a
  line** or **preceded by whitespace**. This means URLs are never mistaken for a
  comment — `https://example.com` stays intact because the `//` comes right after
  the `:`.
- Comment markers inside **quoted strings** are kept as-is, so
  `open "https://site/page#section"` or an [`ai`](./ai-block.md) prompt like
  `"explain // in C++"` work as expected.
- The Markdown body of a [`readme { }`](./readme-block.md) block is preserved
  verbatim, so `//`, `#` and `/* */` written there are kept (they are part of
  your content, not Scimon comments).

## Example

```scimon
path "downloads/"   // comments work next to directives

/*
   The arxiv paper is fetched and renamed,
   while the second URL is skipped for now.
*/
downloads {
    https://arxiv.org/pdf/2203.08877 as "paper.pdf"
    https://arxiv.org/pdf/2405.01513 !ignore
}
```
