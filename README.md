<div align='center'>
    <img src="https://i.imgur.com/jAE5DWn.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
    <img src='https://i.imgur.com/RRPMQ2j.png' />
</p>

## What is Scimon?

Scimon is a tool designed for batch downloading PDF files using its own dedicated language, Monset (.mon). Monset features a very simple and quick-to-write syntax, making it easy to use. The Scimon interpreter is both fast and secure, as it is written in Rust, leveraging the language's best practices.

Scimon is a language designed specifically for downloading files. It offers a streamlined syntax that makes the process of retrieving files from the internet straightforward and efficient. By focusing on simplicity, Scimon ensures that users can quickly grasp its fundamentals and start downloading files with minimal effort.

The key strength of Scimon lies in its user-friendly design. The syntax is intuitive, reducing the learning curve typically associated with programming languages. This makes it accessible to both beginners and experienced developers, allowing them to integrate file downloading capabilities into their projects seamlessly. Scimon abstracts the complexities involved in file transfers, providing a clear and concise way to handle downloads.

## Documentation

For more help and document, see our documentation:

- [How to build](https://docs.scimon.dev/build)
- [Basic usage](https://docs.scimon.dev/basic-usage)
- [Flags](https://docs.scimon.dev/flags)
- [Scrape](https://docs.scimon.dev/scrape)
- [Providers](https://docs.scimon.dev/providers)
- [Monset](https://docs.scimon.dev/monset/what-is)
  - [Downloads Block](https://docs.scimon.dev/monset/download-block)
  - [Readme Block](https://docs.scimon.dev/monset/readme-block)
  - [Commands Block](https://docs.scimon.dev/monset/commands-block)
  - [Compress folder](https://docs.scimon.dev/monset/compress)
  - [Open links](https://docs.scimon.dev/monset/open-links)
  - [Markdown render](https://docs.scimon.dev/monset/markdown-render)
  - [LaTeX render](https://docs.scimon.dev/monset/latex-render)
  - [Style](https://docs.scimon.dev/monset/style)
  - [Print](https://docs.scimon.dev/monset/prints)
  - [Covers](https://docs.scimon.dev/monset/covers)
  - [QR Code](https://docs.scimon.dev/monset/qrcode)
  - [Math](https://docs.scimon.dev/monset/math)
- [Configs](https://docs.scimon.dev/configs/index)
  - [Scimon.yml file](https://docs.scimon.dev/configs/scimon.yml-file)
  - [.env file](https://docs.scimon.dev/configs/env-file)
- [External Resources Usage](https://docs.scimon.dev/external-resources)

## Example of code and execute

```monset
@name "Scimon"
@version "1.0.0"
@description "Scimon is a simple and powerful tool for downloading files, generating QR codes, compressing folders, and more."
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

> [!note]
> Save as `scimon.mon`

Run the command:

```bash
scimon run scimon.mon
```
