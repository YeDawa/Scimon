# .env file

## Open .env file

**To open .env file, simply use run:**

```shell
scimon options open-env
```

### .env file locations in Operations Systems:

| System  | Location                                                           |
| ------- | ------------------------------------------------------------------ |
| Linux   | `home/<YOUR_USERNAME>/.config/scimon/.env`                       |
| MacOS   | `/Users/<YOUR_USERNAME>/Library/Application Support/scimon/.env` |
| Windows | `C:\Users\<YOUR_USERNAME>\AppData\Roaming\scimon\.env`           |

### Environments of system

| Name               | Description                                                   |
| ------------------ | ------------------------------------------------------------- |
| MONLIB_API_KEY     | Your API key for access Scimon (*`Under in development`*) |
| OPENROUTER_API_KEY | Your [OpenRouter](https://openrouter.ai) API key, used by the [`ai` block](../syntax/ai-block.md) |
