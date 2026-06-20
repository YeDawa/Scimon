# Markdown

The `compile` command renders Markdown (`.md` / `.markdown`) files to PDF using
the generic template, with support for MathJax math and Mermaid diagrams.

```shell
scimon compile notes.md
scimon compile notes.md -o build/notes.pdf
scimon compile https://example.com/readme.md
```

The PDF is written next to the input (reusing its name) unless you pass
`-o` / `--output`. A remote `.md` URL is also accepted, and its file name is used
when no output is given.

This is the same rendering used by the [`ai`](../syntax/ai-block.md) block's
`.pdf` output and by [Markdown render](../syntax/markdown-render.md).
