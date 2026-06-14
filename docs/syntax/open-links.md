# Open links

## Open Variable

The `open` variable specifies a URL that Scimon will open in a web browser after processing the list file. This URL is typically used to provide additional information or resources related to the processed task or project.

### Opening URLs:

After Scimon completes processing the list file, it automatically opens the URL specified in the `open` variable in a web browser.

### Usage:

The `open` variable is useful for directing users to relevant websites, documentation, or resources associated with the tasks or projects being processed.

#### Example Usage:

```plaintext
open "https://example.com"
```

In this example:

- The `open` variable is assigned the URL `"https://example.com"`.
- After processing the list file, Scimon will open this URL in a web browser, allowing users to access the resource associated with the task or project.

By utilizing the `open` variable, you can seamlessly provide users with additional information and resources to enhance their understanding and engagement with the processed tasks or projects.

> Use the [`--no-open-link`](../commands.md#global-flags) flag to disable this behavior.
