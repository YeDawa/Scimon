# Security Policy

## Reporting a Vulnerability

We take the security of Scimon seriously. If you discover a security
vulnerability, **please do not open a public issue.**

Instead, report it privately through GitHub's
[**Report a vulnerability**](https://github.com/YeDawa/Scimon/security/advisories/new)
feature (the *Security* tab → *Advisories*). This keeps the details private until
a fix is available.

When reporting, please include as much of the following as you can:

- A description of the vulnerability and its impact.
- Steps to reproduce it (the command and any `.mon` / `.tex` / script input).
- The affected version (`scimon --version`) and your operating system.
- Any proof-of-concept or suggested fix, if you have one.

We'll acknowledge your report as soon as possible, keep you updated on progress,
and credit you in the release notes once a fix ships (unless you'd prefer to stay
anonymous).

## Supported Versions

Scimon is under active development. Security fixes are applied to the latest
release on the `main` branch.

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| older   | :x:                |

## Security considerations when using Scimon

Scimon can fetch and process remote content, so keep the following in mind:

- **The `commands` block runs code.** A `commands { ... }` block executes
  Python/JavaScript/TypeScript scripts — including scripts fetched from a URL.
  Only run `.mon` lists you trust.
- **Secure mode is on by default.** Before executing a script, Scimon inspects
  it: scripts with unusually high entropy (a sign of obfuscation) or that trip
  the built-in security rules are blocked. The `--no-secure` flag disables these
  checks — only use it for scripts you have reviewed and trust.
- **Downloads and rendering reach the network.** Provider links, `readme`
  sources, images, and remote `.tex`/`.mon` files are fetched from the internet.
  Treat untrusted lists as you would any untrusted input.
- **Respect the source.** Make sure you have permission to download or scrape the
  content you point Scimon at, and comply with each provider's terms of use.

Thank you for helping keep Scimon and its users safe.
