# LaTeX Compiler

Scimon ships with a built-in LaTeX engine that turns `.tex` documents into PDF
files — no TeX distribution (TeX Live, MiKTeX, etc.) required. The compiler
parses LaTeX into an internal AST, renders it to HTML, typesets the math with
[MathJax](https://www.mathjax.org/), and prints the result to PDF through the
same headless-browser pipeline used for Markdown rendering.

## Usage

Compile a local file:

```shell
scimon compile paper.tex
```

The PDF is written next to the input, reusing its name (`paper.pdf`).

### Custom output

Use `-o` / `--output` to choose the output file. The extension is normalized to
`.pdf` automatically:

```shell
scimon compile paper.tex -o build/final.pdf
```

### Remote files

You can compile a `.tex` file straight from a URL:

```shell
scimon compile https://example.com/paper.tex
```

When no `--output` is given, the file name is derived from the URL.

### Inside a downloads block

Any `.tex` URL listed in a `downloads { }` block is compiled to PDF
automatically, so you can mix LaTeX sources with regular downloads:

```scimon
downloads {
    https://example.com/paper.tex as "my-paper.pdf"
}
```

## Supported features

The engine focuses on document-oriented LaTeX. The most common commands and
environments are supported out of the box; the math itself is handled by MathJax,
so anything MathJax understands works inside math mode.

### Document structure

- `\documentclass[...]{...}` — class options such as `twocolumn` are honored.
- Sectioning: `\part`, `\chapter`, `\section`, `\subsection`, `\subsubsection`,
  `\paragraph` (with their starred variants).
- `\title`, `\author`, `\date`, `\maketitle`.
- `\begin{abstract} ... \end{abstract}`.
- `\tableofcontents` (built from the document's sections).
- `\addcontentsline`.

### Text formatting

- `\textbf`, `\textit`, `\emph`, `\underline`, `\texttt`, `\textsc`, `\sout`.
- Font sizing (`\tiny` … `\Huge`) and font declarations.
- Colors: `\definecolor`, `\color`, `\textcolor`, `\colorlet`.
- Spacing & breaks: `\vspace`, `\hspace`, `\bigskip`, `\\`, `\newpage`,
  `\noindent`, `\phantom`, `\hphantom`, `\vphantom`, `\parbox`, `\raisebox`,
  `\setlength`, `\linespread`.
- `\rule` / horizontal rules.

### Lists

- `\begin{itemize}`, `\begin{enumerate}` (including `[label=...]`),
  `\begin{description}`.

### Math

- Inline math `$...$` / `\(...\)` and display math `$$...$$` / `\[...\]`.
- `\begin{equation}`, `\begin{align}` (and starred forms).
- `\begin{cases}`, `\begin{matrix}`, `\begin{pmatrix}`, `\begin{bmatrix}`,
  `\begin{array}`.
- `\DeclareMathOperator`.

### Figures, tables and images

- `\includegraphics{...}` (remote images are downloaded and embedded).
- `\begin{figure}` / `\begin{table}` floats (with starred, full-width variants).
- `\begin{tabular}` / `\begin{tabular*}`.
- `\caption` / `\caption*`.

### Cross-references

- `\label`, `\ref`, `\pageref`, `\nameref`.
- `\hyperref`, `\hypertarget`, `\hyperlink`, `\phantomsection`.

Page references (`\pageref`) are resolved after layout, so they point at the real
printed page in the generated PDF.

### Footnotes

- `\footnote`, `\footnotemark`, `\footnotetext`.

### Theorem-like environments

Built-in environments — `theorem`, `lemma`, `proof`, `definition`, `corollary`,
`proposition`, `remark`, `example`, `claim` — are styled and counted
automatically. You can also declare your own:

```latex
\newtheorem{conjecture}{Conjecture}
```

### Bibliography & citations

- `\addbibresource{...}` and `\bibliographystyle{...}`.
- `\cite`, `\nocite` with author/year/textual styles.
- The manual `\begin{thebibliography} ... \end{thebibliography}` environment.

### Acronyms & glossaries

Compatible with the `acronym`, `acro` and `glossaries` command families:

- Definitions: `\newacronym`, `\DeclareAcronym`, `\acrodef`.
- Use: `\ac`, `\acs`, `\acl`, `\acf` (first use expands, later uses abbreviate).
- `\acresetall` / `\glsresetall` to reset usage.
- `\printacronyms` / `\printglossary` to print the list.

### Layout

- `\twocolumn` / `\onecolumn` and `\documentclass[twocolumn]`.
- `\begin{multicols}{n}[preface]` (and `multicols*` for unbalanced columns).
- Running heads/feet (`fancyhdr`): `\chead`, `\cfoot`, `\lhead`, `\rfoot`, … and
  `\thepage`.

### Links and code

- `\url`, `\href`.
- Verbatim / listing code blocks rendered with syntax highlighting.
- Mermaid diagrams.

### Diagrams & plots

- `\begin{tikzpicture}` — TikZ diagrams rendered to inline SVG.
- `\begin{axis} ... \end{axis}` (`pgfplots`) — plots rendered to inline SVG.

## Bundled packages

These packages are always available — you don't need `\usepackage`, though
leaving it in does no harm:

| Package      | Commands / environments                                              |
| ------------ | ------------------------------------------------------------------- |
| `siunitx`    | `\SI`, `\qty`, `\si`, `\unit`, `\num`, `\ang`                        |
| `mhchem`     | `\ce`, `\pu`                                                         |
| `physics`    | `\dv`, `\pdv`, `\fdv`, `\bra`, `\ket`, `\braket`, `\ip`, `\dyad`, `\ev`, `\mel`, `\comm`, `\acomm`, `\pb`, and more |
| `tcolorbox`  | `\begin{tcolorbox} ... \end{tcolorbox}`                             |

## Example

```latex
\documentclass{article}

\title{A Short Note}
\author{Ada Lovelace}
\date{\today}

\begin{document}
\maketitle

\begin{abstract}
A minimal example compiled directly by Scimon.
\end{abstract}

\section{Introduction}
The mass--energy relation is \( E = mc^2 \).

\begin{equation}
    \int_0^\infty e^{-x^2}\,dx = \frac{\sqrt{\pi}}{2}
\end{equation}

\begin{theorem}
Every bounded monotonic sequence converges.
\end{theorem}

Water is written \ce{H2O}, and a length is \SI{3}{\meter}.
\end{document}
```

```shell
scimon compile note.tex
```

> Markdown (`.md`) compilation is recognized by the `compile` command but is not
> implemented yet; use a `downloads` block with a `readme` to render Markdown.
