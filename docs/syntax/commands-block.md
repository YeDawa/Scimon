# Commands Block

> This feature is `Experimental`

## Command Usage Documentation

### Purpose

The provided command `index.py` is used to perform a specific action or operation. In this case, it appears to be referencing a Python script named `index.py`.

### Usage

To use this command, follow the syntax:

```
commands {
    index.py
}
```

Replace `index.py` with the actual name of the Python script you want to execute.

### Example

Suppose you have a Python script named `my_script.py` and you want to execute it using this command. Your configuration file would look like this:

```
commands {
    my_script.py
}
```

### Supported languages

The script is dispatched by its extension:

| Extension(s)                     | Runs with  |
| -------------------------------- | ---------- |
| `.py`                            | `python` (`python3` on Linux/macOS) |
| `.js`, `.mjs`, `.cjs`, `.jsx`    | `node`     |
| `.ts`, `.tsx`                    | `tsc`      |

Make sure the matching runtime is installed and available on your `PATH`.

### Remote scripts

A line can also be a URL. Remote scripts are downloaded into the scripts folder
before running:

```plaintext
commands {
    https://example.com/setup.py
}
```

### Secure mode

By default the runner inspects each script before executing it: scripts with
unusually high entropy (a sign of obfuscation) or that trip the built-in
security rules are blocked. You can turn this off with the
[`--no-secure`](../commands.md#global-flags) flag — only do so for scripts you
trust.

### Scripts files locations in Operations Systems:

| System  | Location                                                               |
| ------- | ---------------------------------------------------------------------- |
| Linux   | `home/<YOUR_USERNAME>/.config/scimon/scripts/`                       |
| MacOS   | `/Users/<YOUR_USERNAME>/Library/Application Support/scimon/scripts/` |
| Windows | `C:\Users\<YOUR_USERNAME>\AppData\Roaming\scimon\scripts\`           |

### Notes

- Ensure the script file exists in the current directory, provide a full path, or use a URL.
- Make sure the corresponding runtime (Python, Node.js, or TypeScript) is installed and accessible from the command line.
