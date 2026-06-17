<div align='center'>
    <img src="https://i.imgur.com/jAE5DWn.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
    <img src='https://i.imgur.com/RRPMQ2j.png' />
</p>

<p align='center'>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
    <img src="https://img.shields.io/badge/built%20with-Rust-orange.svg" alt="Built with Rust">
    <img src="https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforms">
    <a href="https://docs.scimon.dev"><img src="https://img.shields.io/badge/docs-scimon.dev-success.svg" alt="Documentation"></a>
</p>

## What is Scimon?

**Scimon** is a fast, Rust-powered command-line tool for building document
collections from a single, declarative list. You describe *what* you want in the
Scimon language (`.mon`) and the interpreter does the rest — downloading papers,
rendering Markdown and LaTeX to PDF, capturing AI conversations, generating QR
codes, and more.

What began as a batch PDF downloader has grown into a small, friendly
language: the syntax is intuitive and quick to write, with a clear separation
between **variables** (single-line, e.g. `path "..."`) and **blocks**
(multi-line, e.g. `downloads { ... }`), all processed top to bottom.

## Features

- 📥 **Batch downloads** — list URLs and fetch them all, with per-line renaming (`as "name.pdf"`) and skipping (`!ignore`).
- 🌐 **Smart providers** — Arxiv, Sci-Hub, Wikipedia/Wikisource, GitHub/GitLab and more are handled automatically.
- 💬 **AI conversations to PDF** — paste a ChatGPT or Gemini *share link* and Scimon scrapes, cleans, and prints it (images inlined).
- 📐 **Built-in LaTeX compiler** — turn `.tex` files into PDF with no TeX distribution installed (theorems, bibliography, acronyms, TikZ, pgfplots, and more).
- 📝 **Markdown rendering** — render Markdown to styled PDF with MathJax and Mermaid support.
- 🔢 **Math to image** — render formulas straight to PNG.
- 🔳 **QR codes & covers** — generate QR codes and extract document covers.
- 🗜️ **Compression & scripts** — zip output folders and run Python/JavaScript/TypeScript steps (with a secure-by-default runner).

## Requirements

- [Rust &amp; Cargo](https://www.rust-lang.org/tools/install) (to build from source).
- A **Chromium/Chrome** install — used to render HTML/Markdown/LaTeX to PDF.
- [pdfium binaries](https://github.com/bblanchon/pdfium-binaries) — only if you use the `covers` feature.

## Installation

Build from source:

```bash
git clone https://github.com/YeDawa/Scimon.git
cd Scimon
cargo build --release
```

The binary is produced at `target/release/scimon`. See the
[build guide](https://docs.scimon.dev/build) for details.

## Quick start

Create a file named `scimon.mon`:

```scimon
path "downloads/"

downloads {
    https://arxiv.org/pdf/2203.08877 as "arxiv_paper.pdf"
    https://chatgpt.com/share/67c3f647-0bac-8005-abbb-012c3c1dafcc as "chat.pdf"
}
```

Run it:

```bash
scimon run scimon.mon
```

## A fuller example

```scimon
@name "Scimon"
@version "1.0.0"
@description "A simple and powerful tool for downloading files, generating QR codes, compressing folders, and more."
@author "Kremilly"
@license "MIT"
@privacy "Public"
@homepage "https://kremilly.com"

path "downloads/"

open "https://github.com/kremilly"

compress "folder.zip"

covers "downloads/covers/"
qrcode "downloads/qrcodes/"

math "2 + 2" > downloads/math.png
math "2 + 3" > downloads/math1.png

print "Hello, World!"

readme "https://gist.githubusercontent.com/Kremilly/5fd360d994bb0fe108b648d0e4c9e92f/raw/1ede0877f2bd023e77674eb89f4a0eb7d8f7e7da/readme-example.md"

downloads {
    https://arxiv.org/pdf/2203.08877 as "arxiv_paper.pdf"
    https://chatgpt.com/share/67c3f647-0bac-8005-abbb-012c3c1dafcc as "chatgpt_conversation.pdf"
    https://arxiv.org/pdf/2405.01513 !ignore
    https://www.sci-hub.se/10.1626/JCS.66.427
    https://raw.githubusercontent.com/facebook/react/main/README.md
    https://cs.uwaterloo.ca/~jimmylin/publications/Busch_etal_ICDE2012.pdf
    https://raw.githubusercontent.com/h4cknlearn/architecture101/main/README.md !ignore
    https://pt.wikisource.org/wiki/Manifesto_da_Guerrilha_do_Livre_Acesso !ignore
}

commands {
    https://gist.githubusercontent.com/Kremilly/e0e0db11e43269da179adab610f38bb1/raw/6820be26a936a54bac713d03deb49edf804d0b6b/index.py
}
```

> [!NOTE]
> Save the file as `scimon.mon`, then run `scimon run scimon.mon`.

## Documentation

Full documentation is available at **[docs.scimon.dev](https://docs.scimon.dev)**.

- [How to build](https://docs.scimon.dev/build)
- [Basic usage](https://docs.scimon.dev/basic-usage)
- [Commands](https://docs.scimon.dev/commands)
- [Scrape](https://docs.scimon.dev/scrape)
- [Providers](https://docs.scimon.dev/providers)
- [LaTeX Compiler](https://docs.scimon.dev/compile)
- [Syntax](https://docs.scimon.dev/syntax/what-is)
  - [Metadata](https://docs.scimon.dev/syntax/metadata)
  - [Downloads Block](https://docs.scimon.dev/syntax/download-block)
  - [Readme Block](https://docs.scimon.dev/syntax/readme-block)
  - [Commands Block](https://docs.scimon.dev/syntax/commands-block)
  - [Compress folder](https://docs.scimon.dev/syntax/compress)
  - [Open links](https://docs.scimon.dev/syntax/open-links)
  - [Style](https://docs.scimon.dev/syntax/style)
  - [Print](https://docs.scimon.dev/syntax/prints)
  - [Covers](https://docs.scimon.dev/syntax/covers)
  - [QR Code](https://docs.scimon.dev/syntax/qrcode)
  - [Math](https://docs.scimon.dev/syntax/math)
- [Markdown render](https://docs.scimon.dev/syntax/markdown-render)
- [Configs](https://docs.scimon.dev/configs/scimon.yml-file)
  - [Scimon.yml file](https://docs.scimon.dev/configs/scimon.yml-file)
  - [.env file](https://docs.scimon.dev/configs/env-file)

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

Licensed under the [MIT License](LICENSE).
