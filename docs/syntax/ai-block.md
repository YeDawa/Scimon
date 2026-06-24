# AI Block

> This feature is `Experimental`

The `ai { ... }` block generates Markdown files using AI through
[OpenRouter](https://openrouter.ai). You describe what you want in a prompt and
Scimon writes the resulting Markdown straight into your output folder.

## Requirements

You need an OpenRouter API key set as the `OPENROUTER_API_KEY` environment
variable in your Scimon [`.env` file](../configs/env-file.md):

```shell
scimon options write-env
```

```ini
OPENROUTER_API_KEY="sk-or-..."
```

If the key is missing, the block reports a friendly error instead of stopping
the run.

## Usage

Each line inside the block is a prompt followed by the output file:

```scimon
"<prompt>" as "<filename>.md"
```

- The output extension decides the format:
  - `.md` (or no extension) saves the raw generated Markdown.
  - `.pdf` renders the generated Markdown to a styled PDF using the generic
    template (same pipeline as [Markdown render](./markdown-render.md)). The
    [`style`](./style.md) variable is honored for the PDF's CSS, falling back to
    the default stylesheet.
  - `.epub` packages the generated Markdown as an EPUB. The book title comes from
    the output file name and the author is set to `Scimon`.
- The `.md` extension is added automatically if you omit it.
- Files are written inside the folder defined by the [`path`](./what-is.md)
  variable, so they are also picked up by the [`server`](./server.md) command.
- Add `!ignore` to skip an entry without removing it.

### Example

```scimon
path "downloads/"

ai {
    "Write a short article about the Rust programming language" as "rust.md"
    "Explain quantum computing for beginners" as "quantum.pdf"
    "Write a beginner's guide to Rust" as "guide.epub"
    "Draft release notes for version 2.0" as "drafts/release.md" !ignore
}
```

Here `rust.md` is saved as Markdown, `quantum.pdf` is rendered to a PDF, and
`guide.epub` is packaged as an EPUB.

## Choosing a model

By default the block uses the `openai/gpt-4o-mini` model. You can override the
model per entry with `with "provider/model"`, using any model id available on
OpenRouter:

```scimon
ai {
    "Summarize the history of the internet" as "internet.md" with "anthropic/claude-3.5-sonnet"
    "Write a haiku about Rust" as "haiku.md" with "meta-llama/llama-3.1-8b-instruct"
}
```

## Notes

- The model is asked to reply with the Markdown body only (no surrounding code
  fence), so the output is ready to use as-is.
- Combine it with the [`readme`](./readme-block.md) or
  [`markdown render`](./markdown-render.md) features to turn the generated
  Markdown into styled PDF.
- The block runs after downloads but before [`merge`](./merge.md),
  [`covers`](./covers.md) and [`compress`](./compress.md), so generated PDFs are
  merged, get a cover extracted and are included in the archive. The full order
  is: `downloads → ai → merge → covers → compress → qrcode → math → convert`.
