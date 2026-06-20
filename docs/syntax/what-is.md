# Syntax

## The Scimon language

Scimon (`.mon`) is the dedicated language behind the tool. It started as a way to batch-download files and has grown into a small, declarative format for building document collections: alongside downloads, a single list can render Markdown, run scripts, generate QR codes, extract covers, render math to images, compress folders, and more.

The key strength of the language is its user-friendly design. The syntax is intuitive, reducing the learning curve typically associated with programming languages. This makes it accessible to both beginners and experienced developers. A list is just a plain-text file made of **variables** (single-line, like `path "..."`) and **blocks** (multi-line, like `downloads { ... }`), processed top to bottom when you run `scimon run <file>`.

## Summary

- [Metadata](./metadata.md)
- [Downloads Block](./download-block.md)
- [Readme Block](./readme-block.md)
- [Commands Block](./commands-block.md)
- [AI Block](./ai-block.md)
- [Compress folder](./compress.md)
- [Copy folder](./copy.md)
- [Open links](./open-links.md)
- [Style](./style.md)
- [Print](./prints.md)
- [Covers](./covers.md)
- [QR Code](./qrcode.md)
- [Math](./math.md)
- [Server](./server.md)
- [Markdown render](./markdown-render.md)
