# Providers

## What is a provider?

A *provider* is a source Scimon knows how to handle specially. When a URL in a
`downloads {}` block (or a `scrape` result) points at a known provider, Scimon
applies the right strategy automatically — fetching a paper, rendering a wiki
page, or turning an AI conversation into a PDF — instead of a plain download.

## Supported providers

| Provider        | Domain                | Notes                                            |
| --------------- | --------------------- | ------------------------------------------------ |
| Arxiv           | `arxiv.org`           | Research papers (direct PDF).                     |
| Sci-Hub         | `sci-hub.se`          | Research papers (Beta).                           |
| Wikipedia       | `wikipedia.org`       | Rendered to PDF.                                  |
| Wikisource      | `wikisource.org`      | Rendered to PDF.                                  |
| GitHub (raw)    | `raw.githubusercontent.com` | Raw files / README rendering.              |
| GitLab          | `gitlab.com`          | Repository resources.                            |
| Bitbucket       | `bitbucket.org`       | Repository resources.                            |
| Codeberg        | `codeberg.org`        | Repository resources.                            |
| ChatGPT         | `chatgpt.com`         | Shared conversation → PDF (Beta).                |
| Gemini          | `gemini.google.com`   | Shared conversation → PDF (Beta).                |
| NASA            | `nasa.gov`            | Information and resources.                        |
| SciELO          | `scielo.org`          | Latin-American scientific journals.              |

## AI conversations (ChatGPT & Gemini)

Paste a public **share link** into a downloads block and Scimon scrapes the
conversation, cleans it up, inlines any images, and prints it to PDF:

```scimon
path "downloads/"

downloads {
    https://chatgpt.com/share/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx as "chat.pdf"
    https://gemini.google.com/share/xxxxxxxxxxxx as "gemini.pdf"
}
```

How it works:

1. The shared page is loaded in a headless browser and given time to finish
   rendering its JavaScript content.
2. The answer is extracted from the page and stripped of UI chrome.
3. Remote images are downloaded and embedded as base64 so they always render.
4. The HTML is printed to PDF through the same engine used for Markdown and LaTeX.

If you don't pass `as "name.pdf"`, the file is named after the conversation
title.

> Both ChatGPT and Gemini support are **Beta**. They depend on the providers'
> current page structure, so a change on their side can require an update.

## Notes

These providers offer diverse content — from research papers and academic
articles to general knowledge and reference material. Always make sure you have
permission to download or scrape the content you point Scimon at, and respect
each provider's terms of use.
