# Basic Usage

You can download files using a local or remote list with `scimon`. Here are the instructions for both methods:

### Downloading Files

To download files specified in a local list, use the following command:

```shell
scimon run scimon.mon
```

### Downloading Files with a Remote List

To download files specified in a remote list, use the following command:

```shell
scimon run https://example.com/scimon.mon
```

### Useful Flags for Download List

There are several flags available to customize the download process. Here are some commonly used ones:

### Download Without Skipping Any Files

Use the `--no-ignore` flag to download all files without skipping any:

```shell
scimon run scimon.mon --no-ignore
```

### Don't Open Links

Use the `--no-open-link` flag to prevent the `open` variable from opening URLs in your browser:

```shell
scimon run scimon.mon --no-open-link
```

### Skip README File Rendering

Use the `--no-readme` flag to skip rendering README files during the download process:

```shell
scimon run scimon.mon --no-readme
```

### Disable Secure Mode

Use the `--no-secure` flag to disable secure mode for the `commands {}` runner:

```shell
scimon run scimon.mon --no-secure
```

By using these flags, you can control how `scimon` handles different parts of the download list, ensuring a customized download process according to your needs. See the full list on the [Commands](commands.md#global-flags) page.
