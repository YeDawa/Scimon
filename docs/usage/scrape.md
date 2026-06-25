# Scrape

The `scrape` command discovers downloadable documents on a web page and fetches
them for you — handy when you don't have a ready-made list of direct links.

## Usage

```shell
scimon scrape https://example.com
```

Replace `https://example.com` with the page you want to scan.

## How it works

1. Scimon sends the URL to its scrape service, which returns the list of
   documents found on the page.
2. Each non-encrypted item in the result is downloaded, reusing the normal
   download pipeline (so [providers](../providers.md) like ChatGPT/Gemini/Arxiv
   are handled automatically).
3. Files are saved to the Scimon scrape folder.

Items the service marks as `encrypted` are skipped, and if nothing is found a
message explains why.

## Flags

The [global flags](./commands.md#global-flags) apply here too — for example
`--no-ignore` to process every result.

## Notes

- Make sure you have permission to scrape and download the content of the target
  URL, and comply with its terms of use.
- Results depend on what the scrape service can detect on the page.
