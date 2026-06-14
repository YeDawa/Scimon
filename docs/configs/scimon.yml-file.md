# Scimon.yml file

This configuration file is utilized to set up the tool. Please utilize this default version if any alterations you make result in issues.

```yaml
general:
  default_text_editor: 'notepad' # String (default: 'notepad')
  urlfilter_open: false # Boolean (valid values: 'true' or 'false'; default: 'false')
  qrcode_size: 512 # Integer — pixel size of generated QR codes
  level_compress: 6 # Integer (0-9) — compression level used by the `compress` block

ui:
  show_header: true # Boolean (valid values: 'true' or 'false'; default: 'true')

render_markdown:
  output_path: '{app_path}' # String (default: '{app_path}')
  overwrite: true # Boolean (valid values: 'true' or 'false'; default: 'true')
  minify_html: true # Boolean (valid values: 'true' or 'false'; default: 'true')
```

## Open `scimon.yml` file

To open the `scimon.yml` file, run the following command:

```shell
scimon options open-settings
```

## Download `scimon.yml` file

To download the `scimon.yml` file, run the following command:

```shell
scimon options download-settings
```

## `scimon.yml` file locations in Operations Systems:

Save this file at the following location:

| System  | Location                                                                 |
| ------- | ------------------------------------------------------------------------ |
| Linux   | `home/<YOUR_USERNAME>/.config/scimon/scimon.yml`                       |
| MacOS   | `/Users/<YOUR_USERNAME>/Library/Application Support/scimon/scimon.yml` |
| Windows | `C:\Users\<YOUR_USERNAME>\AppData\Roaming\scimon\scimon.yml`           |
