use std::collections::HashMap;

use chrono::Local;

use crate::render::latex::tex_ast::LatexNode;

// ---------------------------------------------------------------------------
// Greek letters, operators and other math symbols supported as \commands
// ---------------------------------------------------------------------------
fn math_symbol(cmd: &str) -> Option<&'static str> {
    match cmd {
        // Greek lowercase
        "alpha"   => Some("α"),
        "beta"    => Some("β"),
        "gamma"   => Some("γ"),
        "delta"   => Some("δ"),
        "epsilon" => Some("ε"),
        "zeta"    => Some("ζ"),
        "eta"     => Some("η"),
        "theta"   => Some("θ"),
        "iota"    => Some("ι"),
        "kappa"   => Some("κ"),
        "lambda"  => Some("λ"),
        "mu"      => Some("μ"),
        "nu"      => Some("ν"),
        "xi"      => Some("ξ"),
        "pi"      => Some("π"),
        "rho"     => Some("ρ"),
        "sigma"   => Some("σ"),
        "tau"     => Some("τ"),
        "upsilon" => Some("υ"),
        "phi"     => Some("φ"),
        "chi"     => Some("χ"),
        "psi"     => Some("ψ"),
        "omega"   => Some("ω"),
        // Greek uppercase
        "Gamma"   => Some("Γ"),
        "Delta"   => Some("Δ"),
        "Theta"   => Some("Θ"),
        "Lambda"  => Some("Λ"),
        "Xi"      => Some("Ξ"),
        "Pi"      => Some("Π"),
        "Sigma"   => Some("Σ"),
        "Upsilon" => Some("Υ"),
        "Phi"     => Some("Φ"),
        "Psi"     => Some("Ψ"),
        "Omega"   => Some("Ω"),
        // Calculus / analysis
        "int"     => Some("∫"),
        "iint"    => Some("∬"),
        "iiint"   => Some("∭"),
        "oint"    => Some("∮"),
        "sum"     => Some("∑"),
        "prod"    => Some("∏"),
        "partial" => Some("∂"),
        "nabla"   => Some("∇"),
        "infty"   => Some("∞"),
        "sqrt"    => None, // handled separately
        // Arrows
        "rightarrow"      => Some("→"),
        "leftarrow"       => Some("←"),
        "Rightarrow"      => Some("⇒"),
        "Leftarrow"       => Some("⇐"),
        "Leftrightarrow"  => Some("⇔"),
        "leftrightarrow"  => Some("↔"),
        "uparrow"         => Some("↑"),
        "downarrow"       => Some("↓"),
        "to"              => Some("→"),
        "mapsto"          => Some("↦"),
        // Relations
        "leq"    => Some("≤"),
        "geq"    => Some("≥"),
        "neq"    => Some("≠"),
        "approx" => Some("≈"),
        "equiv"  => Some("≡"),
        "sim"    => Some("∼"),
        "simeq"  => Some("≃"),
        "cong"   => Some("≅"),
        "propto" => Some("∝"),
        "subset" => Some("⊂"),
        "supset" => Some("⊃"),
        "subseteq" => Some("⊆"),
        "supseteq" => Some("⊇"),
        "in"     => Some("∈"),
        "notin"  => Some("∉"),
        "cup"    => Some("∪"),
        "cap"    => Some("∩"),
        // Operators
        "cdot"   => Some("·"),
        "times"  => Some("×"),
        "div"    => Some("÷"),
        "pm"     => Some("±"),
        "mp"     => Some("∓"),
        "circ"   => Some("∘"),
        "oplus"  => Some("⊕"),
        "otimes" => Some("⊗"),
        // Misc
        "ldots"  => Some("…"),
        "cdots"  => Some("⋯"),
        "vdots"  => Some("⋮"),
        "ddots"  => Some("⋱"),
        "forall" => Some("∀"),
        "exists" => Some("∃"),
        "neg"    => Some("¬"),
        "land"   => Some("∧"),
        "lor"    => Some("∨"),
        "lfloor" => Some("⌊"),
        "rfloor" => Some("⌋"),
        "lceil"  => Some("⌈"),
        "rceil"  => Some("⌉"),
        "langle" => Some("⟨"),
        "rangle" => Some("⟩"),
        "emptyset"   => Some("∅"),
        "infin"      => Some("∞"),
        "therefore"  => Some("∴"),
        "because"    => Some("∵"),
        "perp"       => Some("⊥"),
        "parallel"   => Some("∥"),
        "angle"      => Some("∠"),
        "triangle"   => Some("△"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Diacritic / accent helpers
// ---------------------------------------------------------------------------
fn accent_char(accent: char, base: char) -> String {
    match (accent, base) {
        // Acute (')
        ('\'', 'a') => "á", ('\'', 'e') => "é", ('\'', 'i') => "í",
        ('\'', 'o') => "ó", ('\'', 'u') => "ú", ('\'', 'y') => "ý",
        ('\'', 'A') => "Á", ('\'', 'E') => "É", ('\'', 'I') => "Í",
        ('\'', 'O') => "Ó", ('\'', 'U') => "Ú", ('\'', 'Y') => "Ý",
        ('\'', 'c') => "ć", ('\'', 'C') => "Ć",
        ('\'', 'n') => "ń", ('\'', 'N') => "Ń",
        ('\'', 's') => "ś", ('\'', 'S') => "Ś",
        ('\'', 'z') => "ź", ('\'', 'Z') => "Ź",
        // Grave (`)
        ('`', 'a') => "à", ('`', 'e') => "è", ('`', 'i') => "ì",
        ('`', 'o') => "ò", ('`', 'u') => "ù",
        ('`', 'A') => "À", ('`', 'E') => "È", ('`', 'I') => "Ì",
        ('`', 'O') => "Ò", ('`', 'U') => "Ù",
        // Umlaut / diaeresis (")
        ('"', 'a') => "ä", ('"', 'e') => "ë", ('"', 'i') => "ï",
        ('"', 'o') => "ö", ('"', 'u') => "ü", ('"', 'y') => "ÿ",
        ('"', 'A') => "Ä", ('"', 'E') => "Ë", ('"', 'I') => "Ï",
        ('"', 'O') => "Ö", ('"', 'U') => "Ü",
        // Circumflex (^)
        ('^', 'a') => "â", ('^', 'e') => "ê", ('^', 'i') => "î",
        ('^', 'o') => "ô", ('^', 'u') => "û", ('^', 'w') => "ŵ",
        ('^', 'A') => "Â", ('^', 'E') => "Ê", ('^', 'I') => "Î",
        ('^', 'O') => "Ô", ('^', 'U') => "Û", ('^', 'W') => "Ŵ",
        // Tilde (~)
        ('~', 'n') => "ñ", ('~', 'N') => "Ñ",
        ('~', 'a') => "ã", ('~', 'A') => "Ã",
        ('~', 'o') => "õ", ('~', 'O') => "Õ",
        // Macron (=)
        ('=', 'a') => "ā", ('=', 'e') => "ē", ('=', 'i') => "ī",
        ('=', 'o') => "ō", ('=', 'u') => "ū",
        ('=', 'A') => "Ā", ('=', 'E') => "Ē", ('=', 'I') => "Ī",
        ('=', 'O') => "Ō", ('=', 'U') => "Ū",
        // Dot above (.)
        ('.', 'z') => "ż", ('.', 'Z') => "Ż",
        ('.', 'c') => "ċ", ('.', 'C') => "Ċ",
        ('.', 'e') => "ė", ('.', 'E') => "Ė",
        ('.', 'g') => "ġ", ('.', 'G') => "Ġ",
        ('.', 'I') => "İ",
        // Fallback: return the base character unchanged
        (_, c) => return c.to_string(),
    }.to_string()
}

pub struct Parser {
    pub chars: Vec<char>,
    pub pos: usize,
    pub in_document: bool,

    pub current_chapter: usize,
    pub current_section: usize,
    pub current_subsection: usize,
    pub current_table: usize,
    pub current_equation: usize,
}

impl Parser {

    pub fn new(input: &str) -> Self {
        Parser {
            chars: input.chars().collect(),
            pos: 0,
            in_document: false,
            current_chapter: 0,
            current_section: 0,
            current_subsection: 0,
            current_table: 0,
            current_equation: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Character-level helpers
    // -----------------------------------------------------------------------

    pub fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    /// Skip whitespace characters without consuming them permanently.
    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.next_char();
        }
    }

    // -----------------------------------------------------------------------
    // Text / content helpers
    // -----------------------------------------------------------------------

    /// Collect plain text until a special character.
    pub fn parse_text(&mut self) -> String {
        let mut text = String::new();
        while let Some(&c) = self.chars.get(self.pos) {
            if matches!(c, '\\' | '{' | '}' | '^' | '_' | '%' | '$' | '~' | '&' | '`' | '\'' | '-') {
                break;
            }
            text.push(c);
            self.pos += 1;
        }
        text
    }

    /// Collect everything inside the next `{…}`, respecting nesting.
    pub fn parse_braces_content(&mut self) -> String {
        self.skip_whitespace();
        if self.peek() == Some('{') {
            self.next_char(); // consume '{'
            let mut content = String::new();
            let mut depth = 1usize;
            while depth > 0 {
                match self.next_char() {
                    Some('{') => { depth += 1; content.push('{'); }
                    Some('}') => {
                        depth -= 1;
                        if depth > 0 { content.push('}'); }
                    }
                    Some(c) => content.push(c),
                    None => break,
                }
            }
            content
        } else {
            String::new()
        }
    }

    /// Parse the next argument: `{…}` (multiple chars) or a single char.
    pub fn parse_argument(&mut self) -> Vec<LatexNode> {
        self.skip_whitespace();
        if self.peek() == Some('{') {
            Parser::new(&self.parse_braces_content()).parse(true, &mut HashMap::new())
        } else {
            vec![LatexNode::Text(self.next_char().unwrap_or(' ').to_string())]
        }
    }

    /// Consume an optional `[…]` argument, returning its contents.
    fn parse_optional_arg(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.peek() == Some('[') {
            self.next_char();
            let mut content = String::new();
            while let Some(c) = self.next_char() {
                if c == ']' { break; }
                content.push(c);
            }
            Some(content)
        } else {
            None
        }
    }

    /// Read everything up to `\end{env_name}`, consuming the tag.
    fn read_until_end(&mut self, env_name: &str) -> String {
        let end_tag = format!("\\end{{{}}}", env_name);
        let mut raw = String::new();
        while self.pos < self.chars.len() {
            let lookahead: String = self.chars[self.pos..].iter().take(end_tag.len()).collect();
            if lookahead == end_tag {
                self.pos += end_tag.len();
                break;
            }
            if let Some(c) = self.next_char() {
                raw.push(c);
            }
        }
        raw
    }

    // -----------------------------------------------------------------------
    // Main parse loop
    // -----------------------------------------------------------------------

    pub fn parse(&mut self, force_active: bool, labels: &mut HashMap<String, String>) -> Vec<LatexNode> {
        let mut nodes: Vec<LatexNode> = Vec::new();
        if force_active { self.in_document = true; }

        while self.pos < self.chars.len() {
            let current = self.chars[self.pos];

            // ----------------------------------------------------------------
            // Comments  % ... \n
            // ----------------------------------------------------------------
            if current == '%' {
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }

            // ----------------------------------------------------------------
            // Non-breaking space  ~
            // ----------------------------------------------------------------
            if current == '~' && self.in_document {
                self.pos += 1;
                nodes.push(LatexNode::Text("\u{00A0}".to_string())); // &nbsp;
                continue;
            }

            // ----------------------------------------------------------------
            // Display math  \[ ... \]
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('[')
            {
                self.pos += 2; // consume '\['
                let mut math_block = String::new();
                while self.pos < self.chars.len() {
                    if self.chars[self.pos] == '\\' && self.peek_ahead(1) == Some(']') {
                        self.pos += 2;
                        break;
                    }
                    if let Some(c) = self.next_char() { math_block.push(c); }
                }
                nodes.push(LatexNode::RawMathDisplay(math_block));
                continue;
            }

            // ----------------------------------------------------------------
            // Inline math  \( ... \)
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('(')
            {
                self.pos += 2;
                let mut math_block = String::new();
                while self.pos < self.chars.len() {
                    if self.chars[self.pos] == '\\' && self.peek_ahead(1) == Some(')') {
                        self.pos += 2;
                        break;
                    }
                    if let Some(c) = self.next_char() { math_block.push(c); }
                }
                nodes.push(LatexNode::RawMathInline(math_block));
                continue;
            }

            // ----------------------------------------------------------------
            // Double backslash  \\  → line break (outside verbatim)
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('\\')
            {
                self.pos += 2;
                nodes.push(LatexNode::LineBreak);
                continue;
            }

            // ----------------------------------------------------------------
            // Command  \name
            // ----------------------------------------------------------------
            if current == '\\' {
                self.pos += 1;
                let mut command = String::new();

                // Special single-char commands like \{ \} \_ etc.
                if let Some(&nc) = self.chars.get(self.pos) {
                    if !nc.is_alphabetic() {
                        self.pos += 1; // consume nc

                        // Accent commands: read the base letter from {x} or bare x
                        if matches!(nc, '\'' | '`' | '"' | '^' | '~' | '=' | '.') && self.in_document {
                            let base = if self.peek() == Some('{') {
                                self.parse_braces_content()
                            } else if self.peek().map_or(false, |c| c.is_alphabetic()) {
                                let c = self.next_char().unwrap();
                                c.to_string()
                            } else {
                                // Accent without a following letter — emit the accent itself
                                match nc {
                                    '\'' => "'",
                                    '`'  => "`",
                                    '"'  => "\"",
                                    '^'  => "^",
                                    '~'  => "~",
                                    '='  => "=",
                                    '.'  => ".",
                                    _    => "",
                                }.to_string()
                            };
                            if let Some(c) = base.chars().next() {
                                if base.len() == 1 {
                                    nodes.push(LatexNode::Text(accent_char(nc, c)));
                                } else {
                                    // multi-char base (shouldn't happen) — just pass through
                                    nodes.push(LatexNode::Text(base));
                                }
                            }
                            continue;
                        }

                        let sym = match nc {
                            '{' => "{",
                            '}' => "}",
                            '_' => "_",
                            '&' => "&amp;",
                            '#' => "#",
                            '$' => "$",
                            '%' => "%",
                            ',' => "\u{2009}", // thin space
                            ';' => "\u{2009}",
                            '!' => "",         // negative thin space – ignore
                            ' ' => " ",
                            '-' => "\u{00AD}", // soft hyphen
                            '|' => "∥",
                            ':' => "",
                            _ => "",
                        };
                        if self.in_document && !sym.is_empty() {
                            nodes.push(LatexNode::Text(sym.to_string()));
                        }
                        continue;
                    }
                }

                while let Some(&c) = self.chars.get(self.pos) {
                    if c.is_alphabetic() { command.push(c); self.pos += 1; }
                    else { break; }
                }

                // Consume optional starred variant marker (e.g. \section*)
                let starred = self.peek() == Some('*');
                if starred { self.next_char(); }

                // Skip trailing whitespace after a command (LaTeX semantics)
                if !command.is_empty() {
                    while self.peek() == Some(' ') { self.next_char(); }
                }

                match command.as_str() {

                    // --------------------------------------------------------
                    // Preamble / metadata (allowed outside \begin{document})
                    // --------------------------------------------------------
                    "documentclass" | "usepackage" | "pagestyle"
                    | "setlength" | "setcounter" | "renewcommand"
                    | "geometry" | "hypersetup" => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                    }

                    "newcommand" | "providecommand" => {
                        self.parse_braces_content(); // command name
                        self.parse_optional_arg();   // optional arg count
                        self.parse_braces_content(); // definition body
                    }

                    "newtheorem" | "theoremstyle" => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                        self.parse_optional_arg();
                        self.parse_braces_content();
                    }

                    "title" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Title(
                            Parser::new(&content).parse(true, labels)
                        ));
                    }
                    "author" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Author(
                            Parser::new(&content).parse(true, labels)
                        ));
                    }
                    "date" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Date(
                            Parser::new(&content).parse(true, labels)
                        ));
                    }

                    // --------------------------------------------------------
                    // Environments
                    // --------------------------------------------------------
                    "begin" => {
                        let env = self.parse_braces_content();

                        // Consume optional placement [htbp] etc.
                        self.parse_optional_arg();

                        if env == "document" {
                            self.in_document = true;

                        } else if env == "abstract" && self.in_document {
                            let raw = self.read_until_end("abstract");
                            nodes.push(LatexNode::Abstract(
                                Parser::new(raw.trim()).parse(true, labels)
                            ));

                        } else if (env == "lstlisting" || env == "verbatim" || env == "Verbatim") && self.in_document {
                            self.parse_optional_arg(); // lstlisting options
                            let raw = self.read_until_end(&env);
                            nodes.push(LatexNode::CodeBlock(raw.trim_matches('\n').to_string()));

                        } else if env == "minted" && self.in_document {
                            self.parse_braces_content(); // language arg
                            let raw = self.read_until_end("minted");
                            nodes.push(LatexNode::CodeBlock(raw.trim_matches('\n').to_string()));

                        } else if env == "itemize" && self.in_document {
                            let block = self.read_until_end("itemize");
                            let items = Self::split_items(&block, labels);
                            nodes.push(LatexNode::Itemize(items));

                        } else if env == "enumerate" && self.in_document {
                            let block = self.read_until_end("enumerate");
                            let items = Self::split_items(&block, labels);
                            nodes.push(LatexNode::Enumerate(items));

                        } else if env == "description" && self.in_document {
                            let block = self.read_until_end("description");
                            let items = Self::split_description_items(&block, labels);
                            nodes.push(LatexNode::Description(items));

                        } else if env == "mermaid" && self.in_document {
                            let raw = self.read_until_end("mermaid");
                            nodes.push(LatexNode::Mermaid(raw));

                        } else if env == "tabular" && self.in_document {
                            self.parse_braces_content(); // column spec
                            let table_block = self.read_until_end("tabular");
                            let rows = Self::parse_tabular(&table_block, labels);
                            nodes.push(LatexNode::Table(rows));

                        } else if env == "table" && self.in_document {
                            let raw = self.read_until_end("table");
                            self.current_table += 1;
                            // Pre-register \label{tab:…} with the current table number
                            Self::extract_and_register_labels(&raw, &self.current_table.to_string(), "tab:", labels);
                            let mut sub = Parser::new(raw.trim());
                            sub.current_table      = self.current_table;
                            sub.current_section    = self.current_section;
                            sub.current_chapter    = self.current_chapter;
                            sub.current_subsection = self.current_subsection;
                            sub.current_equation   = self.current_equation;
                            let children = sub.parse(true, labels);
                            nodes.push(LatexNode::TableFloat(children));

                        } else if env == "figure" && self.in_document {
                            let raw = self.read_until_end("figure");
                            Self::extract_and_register_labels(&raw, &self.current_section.to_string(), "fig:", labels);
                            let mut sub = Parser::new(raw.trim());
                            sub.current_section    = self.current_section;
                            sub.current_chapter    = self.current_chapter;
                            sub.current_equation   = self.current_equation;
                            sub.current_table      = self.current_table;
                            sub.current_subsection = self.current_subsection;
                            let children = sub.parse(true, labels);
                            nodes.push(LatexNode::FigureFloat(children));

                        } else if (env == "equation" || env == "equation*") && self.in_document {
                            let raw = self.read_until_end(&env);
                            self.current_equation += 1;
                            Self::extract_and_register_labels(&raw, &self.current_equation.to_string(), "", labels);
                            nodes.push(LatexNode::EquationBlock(
                                vec![LatexNode::RawMathDisplay(raw.trim().to_string())]
                            ));

                        } else if (env == "align"  || env == "align*"
                                || env == "eqnarray" || env == "eqnarray*"
                                || env == "multline" || env == "multline*"
                                || env == "gather"   || env == "gather*"
                                || env == "flalign"  || env == "flalign*") && self.in_document
                        {
                            let raw = self.read_until_end(&env);
                            self.current_equation += 1;
                            Self::extract_and_register_labels(&raw, &self.current_equation.to_string(), "", labels);
                            let latex = format!("\\begin{{{}}}{}\\end{{{}}}", env, raw.trim(), env);
                            nodes.push(LatexNode::AlignBlock(
                                vec![LatexNode::RawMathDisplay(latex)]
                            ));

                        } else if matches!(env.as_str(),
                            "pmatrix" | "bmatrix" | "Bmatrix" |
                            "vmatrix" | "Vmatrix" | "matrix"  |
                            "smallmatrix"
                        ) && self.in_document {
                            let raw = self.read_until_end(&env);
                            let (open, close) = match env.as_str() {
                                "pmatrix"            => ("(", ")"),
                                "bmatrix"            => ("[", "]"),
                                "Bmatrix"            => ("{", "}"),
                                "vmatrix"            => ("|", "|"),
                                "Vmatrix"            => ("‖", "‖"),
                                _                    => ("", ""),
                            };
                            let inner = Self::parse_matrix_body(&raw, labels);
                            nodes.push(LatexNode::Matrix { open, close, rows: inner });

                        } else if env == "center" && self.in_document {
                            let raw = self.read_until_end("center");
                            let inner = Parser::new(raw.trim()).parse(true, labels);
                            // Wrap in a centred div
                            nodes.push(LatexNode::Text("<div style=\"text-align:center;\">".to_string()));
                            nodes.extend(inner);
                            nodes.push(LatexNode::Text("</div>".to_string()));

                        } else if env == "quote" || env == "quotation" {
                            let raw = self.read_until_end(&env);
                            let inner = Parser::new(raw.trim()).parse(true, labels);
                            nodes.push(LatexNode::Text("<blockquote class=\"latex-quote\">".to_string()));
                            nodes.extend(inner);
                            nodes.push(LatexNode::Text("</blockquote>".to_string()));

                        } else if env == "theorem" || env == "lemma" || env == "corollary"
                                || env == "proposition" || env == "proof"
                                || env == "definition" || env == "remark" || env == "example"
                        {
                            let raw = self.read_until_end(&env);
                            let inner = Parser::new(raw.trim()).parse(true, labels);
                            let label = {
                                let mut s = env.clone();
                                if let Some(r) = s.get_mut(0..1) {
                                    r.make_ascii_uppercase();
                                }
                                s
                            };
                            nodes.push(LatexNode::Text(
                                format!("<div class=\"latex-theorem latex-{}\"><strong>{}.</strong> ", env, label)
                            ));
                            nodes.extend(inner);
                            nodes.push(LatexNode::Text("</div>".to_string()));

                        } else {
                            // Unknown environment – skip \end{env}
                            self.read_until_end(&env);
                        }
                    }

                    "end" => {
                        let env = self.parse_braces_content();
                        if env == "document" { self.in_document = false; }
                    }

                    // --------------------------------------------------------
                    // Document structure commands
                    // --------------------------------------------------------
                    "maketitle" if self.in_document => nodes.push(LatexNode::MakeTitle),
                    "tableofcontents" if self.in_document => nodes.push(LatexNode::TableOfContents),

                    "chapter" if self.in_document => {
                        self.parse_optional_arg(); // short title
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!("<h1>{}</h1>", title)));
                        } else {
                            self.current_chapter += 1;
                            self.current_section = 0;
                            nodes.push(LatexNode::Chapter(title));
                        }
                    }

                    "section" if self.in_document => {
                        self.parse_optional_arg();
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!("<h2 class=\"section-star\">{}</h2>", title)));
                        } else {
                            self.current_section += 1;
                            self.current_subsection = 0;
                            nodes.push(LatexNode::Section(title));
                        }
                    }

                    "subsection" if self.in_document => {
                        self.parse_optional_arg();
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!("<h3 class=\"section-star\">{}</h3>", title)));
                        } else {
                            self.current_subsection += 1;
                            nodes.push(LatexNode::Subsection(title));
                        }
                    }

                    "subsubsection" if self.in_document => {
                        self.parse_optional_arg();
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!("<h4 class=\"section-star\">{}</h4>", title)));
                        } else {
                            nodes.push(LatexNode::Subsubsection(title));
                        }
                    }

                    "paragraph" if self.in_document => {
                        self.parse_optional_arg();
                        nodes.push(LatexNode::Paragraph(self.parse_braces_content()));
                    }

                    "subparagraph" if self.in_document => {
                        self.parse_optional_arg();
                        nodes.push(LatexNode::Paragraph(self.parse_braces_content()));
                    }

                    // --------------------------------------------------------
                    // Inline formatting
                    // --------------------------------------------------------
                    "textbf" | "mathbf" if self.in_document => {
                        nodes.push(LatexNode::Bold(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "textit" | "mathit" | "emph" if self.in_document => {
                        nodes.push(LatexNode::Italic(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "underline" if self.in_document => {
                        nodes.push(LatexNode::Underline(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "texttt" | "mathtt" | "verb" if self.in_document => {
                        nodes.push(LatexNode::Monospace(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "textsc" if self.in_document => {
                        nodes.push(LatexNode::SmallCaps(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "sout" | "st" | "strikethrough" if self.in_document => {
                        nodes.push(LatexNode::Strikethrough(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "textrm" | "mathrm" | "textnormal" | "textmd" | "textup" if self.in_document => {
                        // Remove special formatting – render as plain group
                        nodes.extend(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        );
                    }

                    // Font size declarations (scoped to the current group by the caller;
                    // here they consume the *rest* of the current brace group if any,
                    // otherwise they apply to the rest of the current scope)
                    "tiny" | "scriptsize" | "footnotesize" | "small" | "normalsize"
                    | "large" | "Large" | "LARGE" | "huge" | "Huge" | "HUGE"
                    if self.in_document => {
                        // If followed by a brace group, scope the size to it.
                        self.skip_whitespace();
                        if self.peek() == Some('{') {
                            let content = self.parse_braces_content();
                            nodes.push(LatexNode::FontSize(
                                command.clone(),
                                Parser::new(&content).parse(true, labels)
                            ));
                        } else {
                            // Declaration form: consume rest of stream
                            let mut rest = String::new();
                            while self.pos < self.chars.len() {
                                rest.push(self.chars[self.pos]);
                                self.pos += 1;
                            }
                            nodes.push(LatexNode::FontSize(
                                command.clone(),
                                Parser::new(&rest).parse(true, labels)
                            ));
                        }
                    }

                    // --------------------------------------------------------
                    // Math formatting
                    // --------------------------------------------------------
                    "math" if self.in_document =>
                        nodes.push(LatexNode::MathInline(self.parse_argument())),

                    "frac" if self.in_document => {
                        // Captura o LaTeX bruto e passa para KaTeX
                        let num = self.parse_braces_content();
                        let den = self.parse_braces_content();
                        nodes.push(LatexNode::RawMathInline(format!("\\frac{{{}}}{{{}}}", num, den)));
                    }
                    "sqrt" if self.in_document => {
                        self.parse_optional_arg();
                        let arg = self.parse_braces_content();
                        nodes.push(LatexNode::RawMathInline(format!("\\sqrt{{{}}}", arg)));
                    }

                    "overline" | "hat" | "bar" if self.in_document => {
                        nodes.extend(self.parse_argument());
                    }

                    "left" | "right" if self.in_document => {
                        // \left( ... \right) — consume the delimiter, ignore it
                        self.next_char();
                    }

                    // --------------------------------------------------------
                    // Math spacing & alignment
                    // --------------------------------------------------------
                    "quad" if self.in_document =>
                        nodes.push(LatexNode::HSpace("1em".to_string())),

                    "qquad" if self.in_document =>
                        nodes.push(LatexNode::HSpace("2em".to_string())),

                    // \text{...} inside math — render as plain HTML text span
                    "text" | "textrm" | "mathrm" if self.in_document => {
                        let content = self.parse_braces_content();
                        
                        nodes.push(LatexNode::Text(
                            format!("<span class=\"math-text\">{}</span>", content)
                        ));
                    }

                    // Alignment column separator in align/eqnarray — emit a space
                    // (the & character is intercepted before reaching here only when
                    //  parse_text() stops at it; handle the command form here)
                    "intertext" if self.in_document => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Text(
                            format!("<div class=\"math-intertext\">{}</div>", content)
                        ));
                    }

                    // \tag{...} — custom equation tag
                    "tag" | "tag*" if self.in_document => {
                        let tag = self.parse_braces_content();
                        nodes.push(LatexNode::Text(
                            format!("<span class=\"eq-number\">({})</span>", tag)
                        ));
                    }

                    // \underbrace, \overbrace, \underset, \overset
                    "underbrace" | "overbrace" if self.in_document => {
                        let arg = self.parse_argument();
                        nodes.extend(arg);
                    }
                    
                    "underset" | "overset" if self.in_document => {
                        let _over = self.parse_argument();
                        let base = self.parse_argument();
                        nodes.extend(base);
                    }

                    // \limits, \nolimits — ignored (display hint only)
                    "limits" | "nolimits" | "displaystyle" | "textstyle"
                    | "scriptstyle" | "scriptscriptstyle" => {}

                    // \boldsymbol, \mathbf inside math
                    "boldsymbol" | "bm" if self.in_document => {
                        nodes.push(LatexNode::Bold(self.parse_argument()));
                    }

                    // \operatorname{...} — render as upright text
                    "operatorname" | "operatorname*" if self.in_document => {
                        let name = self.parse_braces_content();
                        nodes.push(LatexNode::Text(
                            format!("<span class=\"math-op\">{}</span>", name)
                        ));
                    }

                    // Named operators that are just symbols
                    "sin" | "cos" | "tan" | "cot" | "sec" | "csc"
                    | "arcsin" | "arccos" | "arctan"
                    | "sinh" | "cosh" | "tanh"
                    | "log" | "ln" | "lg" | "exp"
                    | "lim" | "limsup" | "liminf"
                    | "sup" | "inf" | "max" | "min"
                    | "det" | "deg" | "dim" | "ker"
                    | "gcd" | "lcm" | "arg" | "Re" | "Im"
                    if self.in_document => {
                        nodes.push(LatexNode::Text(
                            format!("<span class=\"math-op\">{}</span>", command)
                        ));
                    }

                    // --------------------------------------------------------
                    // Spacing
                    // --------------------------------------------------------
                    "vspace" | "vspace*" if self.in_document => {
                        nodes.push(LatexNode::VSpace(self.parse_braces_content()));
                    }
                    "hspace" | "hspace*" if self.in_document => {
                        nodes.push(LatexNode::HSpace(self.parse_braces_content()));
                    }

                    "noindent" | "indent" | "centering" | "raggedright"
                    | "raggedleft" | "clearpage" | "cleardoublepage"
                    | "smallskip" | "medskip" | "bigskip" => {}

                    "newline" if self.in_document => nodes.push(LatexNode::LineBreak),
                    "newpage" | "pagebreak" if self.in_document => nodes.push(LatexNode::NewPage),

                    "par" if self.in_document => nodes.push(LatexNode::Text("\n\n".to_string())),
                    "nobreakspace" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{00A0}".to_string())),

                    "rule" if self.in_document => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                        self.parse_braces_content();
                        nodes.push(LatexNode::HorizontalRule);
                    }

                    // --------------------------------------------------------
                    // Links
                    // --------------------------------------------------------
                    "url" if self.in_document => {
                        nodes.push(LatexNode::Url(self.parse_braces_content()));
                    }

                    "href" if self.in_document => {
                        let link_url = self.parse_braces_content();
                        let link_text = self.parse_argument();
                        nodes.push(LatexNode::Href { url: link_url, text: link_text });
                    }

                    // --------------------------------------------------------
                    // Citations & bibliography
                    // --------------------------------------------------------
                    "cite" | "citep" | "citet" | "citealt" | "citealp"
                    | "citeauthor" | "citeyear" if self.in_document =>
                    {
                        self.parse_optional_arg(); // optional note
                        let raw = self.parse_braces_content();
                        let keys: Vec<String> = raw.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        if keys.len() == 1 {
                            nodes.push(LatexNode::Cite(keys.into_iter().next().unwrap()));
                        } else {
                            nodes.push(LatexNode::CiteMultiple(keys));
                        }
                    }

                    "bibliography" if self.in_document =>
                        nodes.push(LatexNode::Bibliography(self.parse_braces_content())),

                    "bibliographystyle" => { self.parse_braces_content(); }

                    // --------------------------------------------------------
                    // Labels & cross-references
                    // --------------------------------------------------------
                    "label" if self.in_document => {
                        let label_name = self.parse_braces_content();
                        let target_value = if label_name.starts_with("tab:") {
                            self.current_table.to_string()
                        } else if label_name.starts_with("chap:") {
                            self.current_chapter.to_string()
                        } else if label_name.starts_with("subsec:") {
                            format!("{}.{}", self.current_section, self.current_subsection)
                        } else if label_name.starts_with("eq:") || label_name.starts_with("eqn:") {
                            self.current_equation.to_string()
                        } else if label_name.starts_with("fig:") {
                            self.current_section.to_string()
                        } else if label_name.starts_with("sec:") {
                            self.current_section.to_string()
                        } else {
                            if self.current_equation > 0 {
                                self.current_equation.to_string()
                            } else {
                                self.current_section.to_string()
                            }
                        };

                        labels.insert(label_name.clone(), target_value);
                        nodes.push(LatexNode::Label(label_name));
                    }

                    "ref" | "eqref" | "autoref" | "cref" if self.in_document =>
                        nodes.push(LatexNode::Ref(self.parse_braces_content())),
                    "pageref" if self.in_document =>
                        nodes.push(LatexNode::PageRef(self.parse_braces_content())),

                    // --------------------------------------------------------
                    // Footnotes
                    // --------------------------------------------------------
                    "footnote" if self.in_document => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Footnote(
                            Parser::new(&content).parse(true, labels)
                        ));
                    }

                    "footnotemark" if self.in_document => {
                        self.parse_optional_arg();
                    }

                    "footnotetext" if self.in_document => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                    }

                    // --------------------------------------------------------
                    // Images
                    // --------------------------------------------------------
                    "includegraphics" if self.in_document => {
                        self.parse_optional_arg(); // [width=…]
                        nodes.push(LatexNode::Image(self.parse_braces_content()));
                    }
                    "caption" if self.in_document =>
                        nodes.push(LatexNode::Caption(self.parse_braces_content())),

                    // --------------------------------------------------------
                    // Ignored structural commands
                    // --------------------------------------------------------
                    "hline" | "cline" | "toprule" | "midrule" | "bottomrule"
                    | "addcontentsline" | "appendix" | "frontmatter"
                    | "mainmatter" | "backmatter" | "sloppy" | "frenchspacing"
                    | "nonfrenchspacing" | "protect" => {}

                    "input" | "include" | "includeonly" => {
                        self.parse_braces_content(); // ignore file name
                    }

                    // --------------------------------------------------------
                    // Date
                    // --------------------------------------------------------
                    "today" if self.in_document => {
                        let date = Local::now().format("%B %d, %Y").to_string();
                        nodes.push(LatexNode::Text(date));
                    }

                    // --------------------------------------------------------
                    // Author helpers
                    // --------------------------------------------------------
                    "and" if self.in_document =>
                        nodes.push(LatexNode::Text(" and ".to_string())),

                    "thanks" if self.in_document => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Footnote(
                            Parser::new(&content).parse(true, labels)
                        ));
                    }

                    // --------------------------------------------------------
                    // Colors & boxes
                    // --------------------------------------------------------
                    "textcolor" if self.in_document => {
                        let color = self.parse_braces_content();
                        let content = self.parse_braces_content();
                        let inner = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::Text(format!("<span style=\"color: {};\">", color)));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    "colorbox" if self.in_document => {
                        let color = self.parse_braces_content();
                        let content = self.parse_braces_content();
                        let inner = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::Text(format!(
                            "<span style=\"background-color: {}; padding: 2px 5px; border-radius: 3px;\">", color
                        )));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    "fbox" | "framebox" if self.in_document => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        let inner = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"border: 1px solid currentColor; padding: 2px 6px;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    "mbox" | "makebox" if self.in_document => {
                        self.parse_optional_arg();
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.extend(Parser::new(&content).parse(true, labels));
                    }

                    // --------------------------------------------------------
                    // Horizontal fill
                    // --------------------------------------------------------
                    "hfill" | "hfil" | "dotfill" if self.in_document => {
                        nodes.push(LatexNode::Text(
                            "<span style=\"display:inline-block; flex:1; min-width:1em;\"></span>".to_string()
                        ));
                    }

                    "vfill" if self.in_document => nodes.push(LatexNode::VSpace("auto".to_string())),

                    // --------------------------------------------------------
                    // Special letters
                    // --------------------------------------------------------
                    "ss" if self.in_document => nodes.push(LatexNode::Text("ß".to_string())),
                    "ae" if self.in_document => nodes.push(LatexNode::Text("æ".to_string())),
                    "AE" if self.in_document => nodes.push(LatexNode::Text("Æ".to_string())),
                    "oe" if self.in_document => nodes.push(LatexNode::Text("œ".to_string())),
                    "OE" if self.in_document => nodes.push(LatexNode::Text("Œ".to_string())),
                    "aa" if self.in_document => nodes.push(LatexNode::Text("å".to_string())),
                    "AA" if self.in_document => nodes.push(LatexNode::Text("Å".to_string())),
                    "o"  if self.in_document => nodes.push(LatexNode::Text("ø".to_string())),
                    "O"  if self.in_document => nodes.push(LatexNode::Text("Ø".to_string())),
                    "l"  if self.in_document => nodes.push(LatexNode::Text("ł".to_string())),
                    "L"  if self.in_document => nodes.push(LatexNode::Text("Ł".to_string())),
                    "i"  if self.in_document => nodes.push(LatexNode::Text("ı".to_string())),
                    "j"  if self.in_document => nodes.push(LatexNode::Text("ȷ".to_string())),

                    // --------------------------------------------------------
                    // Named accent commands
                    // --------------------------------------------------------
                    "c" if self.in_document => {
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "c" => "ç", "C" => "Ç", "s" => "ş", "S" => "Ş",
                            "t" => "ţ", "T" => "Ţ", "n" => "ņ", "N" => "Ņ",
                            "k" => "ķ", "K" => "Ķ", "l" => "ļ", "L" => "Ļ",
                            "r" => "ŗ", "R" => "Ŗ", _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "v" if self.in_document => {
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "c" => "č", "C" => "Č", "s" => "š", "S" => "Š",
                            "z" => "ž", "Z" => "Ž", "e" => "ě", "E" => "Ě",
                            "n" => "ň", "N" => "Ň", "r" => "ř", "R" => "Ř",
                            "d" => "ď", "D" => "Ď", "t" => "ť", "T" => "Ť",
                            _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "u" if self.in_document => {
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "a" => "ă", "A" => "Ă", "e" => "ĕ", "E" => "Ĕ",
                            "g" => "ğ", "G" => "Ğ", "i" => "ĭ", "I" => "Ĭ",
                            "o" => "ŏ", "O" => "Ŏ", "u" => "ŭ", "U" => "Ŭ",
                            _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "H" if self.in_document => {
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "o" => "ő", "O" => "Ő", "u" => "ű", "U" => "Ű", _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "k" if self.in_document => {
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "a" => "ą", "A" => "Ą", "e" => "ę", "E" => "Ę",
                            "i" => "į", "I" => "Į", "u" => "ų", "U" => "Ų",
                            "o" => "ǫ", _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "d" if self.in_document => {
                        // dot below
                        let base = self.parse_braces_content();
                        let result = match base.as_str() {
                            "a" => "ạ", "A" => "Ạ", "e" => "ẹ", "E" => "Ẹ",
                            "i" => "ị", "I" => "Ị", "o" => "ọ", "O" => "Ọ",
                            "u" => "ụ", "U" => "Ụ", _ => &base,
                        };
                        nodes.push(LatexNode::Text(result.to_string()));
                    }

                    "b" if self.in_document => {
                        // bar under — just render base letter
                        let base = self.parse_braces_content();
                        nodes.push(LatexNode::Text(base));
                    }

                    // --------------------------------------------------------
                    // Quote typography helpers
                    // --------------------------------------------------------
                    "lq" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{2018}".to_string())), // '
                    "rq" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{2019}".to_string())), // '
                    "ldq" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{201C}".to_string())), // "
                    "rdq" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{201D}".to_string())), // "
                    "textquoteleft" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{2018}".to_string())),
                    "textquoteright" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{2019}".to_string())),
                    "textquotedblleft" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{201C}".to_string())),
                    "textquotedblright" if self.in_document =>
                        nodes.push(LatexNode::Text("\u{201D}".to_string())),
                    "guillemotleft" if self.in_document =>
                        nodes.push(LatexNode::Text("«".to_string())),
                    "guillemotright" if self.in_document =>
                        nodes.push(LatexNode::Text("»".to_string())),
                    "guilsinglleft" if self.in_document =>
                        nodes.push(LatexNode::Text("‹".to_string())),
                    "guilsinglright" if self.in_document =>
                        nodes.push(LatexNode::Text("›".to_string())),

                    // --------------------------------------------------------
                    // Text symbols
                    // --------------------------------------------------------
                    "textasciicircum" if self.in_document =>
                        nodes.push(LatexNode::Text("^".to_string())),
                    "textasciitilde" if self.in_document =>
                        nodes.push(LatexNode::Text("~".to_string())),
                    "textbackslash" if self.in_document =>
                        nodes.push(LatexNode::Text("\\".to_string())),
                    "textbar" if self.in_document =>
                        nodes.push(LatexNode::Text("|".to_string())),
                    "textless" if self.in_document =>
                        nodes.push(LatexNode::Text("&lt;".to_string())),
                    "textgreater" if self.in_document =>
                        nodes.push(LatexNode::Text("&gt;".to_string())),
                    "textbullet" if self.in_document =>
                        nodes.push(LatexNode::Text("•".to_string())),
                    "textdagger" | "dag" if self.in_document =>
                        nodes.push(LatexNode::Text("†".to_string())),
                    "textdaggerdbl" | "ddag" if self.in_document =>
                        nodes.push(LatexNode::Text("‡".to_string())),
                    "textsection" | "S" if self.in_document =>
                        nodes.push(LatexNode::Text("§".to_string())),
                    "textparagraph" | "P" if self.in_document =>
                        nodes.push(LatexNode::Text("¶".to_string())),
                    "copyright" if self.in_document =>
                        nodes.push(LatexNode::Text("©".to_string())),
                    "textregistered" if self.in_document =>
                        nodes.push(LatexNode::Text("®".to_string())),
                    "texttrademark" | "trademark" if self.in_document =>
                        nodes.push(LatexNode::Text("™".to_string())),
                    "pounds" if self.in_document =>
                        nodes.push(LatexNode::Text("£".to_string())),
                    "euro" if self.in_document =>
                        nodes.push(LatexNode::Text("€".to_string())),
                    "textyen" if self.in_document =>
                        nodes.push(LatexNode::Text("¥".to_string())),
                    "textdegree" if self.in_document =>
                        nodes.push(LatexNode::Text("°".to_string())),
                    "textellipsis" if self.in_document =>
                        nodes.push(LatexNode::Text("…".to_string())),
                    "textendash" | "endash" if self.in_document =>
                        nodes.push(LatexNode::Text("–".to_string())),
                    "textemdash" | "emdash" if self.in_document =>
                        nodes.push(LatexNode::Text("—".to_string())),

                    // --------------------------------------------------------
                    // List-of-figures / list-of-tables (placeholder)
                    // --------------------------------------------------------
                    "listoffigures" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<div class=\"toc\"><h2>List of Figures</h2><p><em>(auto-generated)</em></p></div>".to_string()
                        )),
                    "listoftables" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<div class=\"toc\"><h2>List of Tables</h2><p><em>(auto-generated)</em></p></div>".to_string()
                        )),

                    // --------------------------------------------------------
                    // Math symbols (catches all entries in math_symbol())
                    // --------------------------------------------------------
                    _ if self.in_document => {
                        if let Some(sym) = math_symbol(&command) {
                            nodes.push(LatexNode::Text(sym.to_string()));
                        }
                    }

                    _ => {} // preamble commands we don't recognise
                }

                continue;
            }

            // ----------------------------------------------------------------
            // Alignment column separator  &
            // In math (align/eqnarray) emit spacing; tabular handles & itself
            // in parse_tabular() before this branch is reached.
            // ----------------------------------------------------------------
            if current == '&' && self.in_document {
                self.pos += 1;
                nodes.push(LatexNode::HSpace("0.5em".to_string()));
                continue;
            }

            // ----------------------------------------------------------------
            // Superscript  ^
            // ----------------------------------------------------------------
            if current == '^' && self.in_document {
                self.pos += 1;
                nodes.push(LatexNode::Superscript(self.parse_argument()));
                continue;
            }

            // ----------------------------------------------------------------
            // Subscript  _
            // ----------------------------------------------------------------
            if current == '_' && self.in_document {
                self.pos += 1;
                nodes.push(LatexNode::Subscript(self.parse_argument()));
                continue;
            }

            // ----------------------------------------------------------------
            // Inline math  $...$  or  $$...$$
            // ----------------------------------------------------------------
            if current == '$' && self.in_document {
                self.pos += 1;
                let display = self.peek() == Some('$');
                if display { self.pos += 1; }

                let mut math_block = String::new();
                while let Some(&c) = self.chars.get(self.pos) {
                    if c == '$' {
                        self.pos += 1;
                        if display && self.peek() == Some('$') { self.pos += 1; }
                        break;
                    }
                    math_block.push(c);
                    self.pos += 1;
                }

                // Pass raw LaTeX to MathJax instead of rendering ourselves
                if display {
                    nodes.push(LatexNode::RawMathDisplay(math_block));
                } else {
                    nodes.push(LatexNode::RawMathInline(math_block));
                }
                continue;
            }

            // ----------------------------------------------------------------
            // Brace group  { ... }
            // ----------------------------------------------------------------
            if current == '{' && self.in_document {
                let content = self.parse_braces_content();
                nodes.extend(Parser::new(&content).parse(true, labels));
                continue;
            }

            if current == '}' && self.in_document {
                self.pos += 1;
                continue;
            }

            // ----------------------------------------------------------------
            // LaTeX typographic quotes:
            //   ``  →  " (U+201C)     ''  →  " (U+201D)
            //   `   →  ' (U+2018)     '   in text → ' (U+2019)
            // These are just source characters, not commands.
            // ----------------------------------------------------------------
            if current == '`' && self.in_document {
                self.pos += 1;
                if self.peek() == Some('`') {
                    self.pos += 1;
                    nodes.push(LatexNode::Text("\u{201C}".to_string())); // "
                } else {
                    nodes.push(LatexNode::Text("\u{2018}".to_string())); // '
                }
                continue;
            }

            if current == '\'' && self.in_document {
                self.pos += 1;
                if self.peek() == Some('\'') {
                    self.pos += 1;
                    nodes.push(LatexNode::Text("\u{201D}".to_string())); // "
                } else {
                    nodes.push(LatexNode::Text("\u{2019}".to_string())); // '
                }
                continue;
            }

            // Em-dash  ---  and en-dash  --  (hyphens in text)
            if current == '-' && self.in_document {
                self.pos += 1;
                if self.peek() == Some('-') {
                    self.pos += 1;
                    if self.peek() == Some('-') {
                        self.pos += 1;
                        nodes.push(LatexNode::Text("—".to_string())); // em-dash
                    } else {
                        nodes.push(LatexNode::Text("–".to_string())); // en-dash
                    }
                } else {
                    nodes.push(LatexNode::Text("-".to_string()));
                }
                continue;
            }

            // ----------------------------------------------------------------
            // Plain text
            // ----------------------------------------------------------------
            let text = self.parse_text();
            if text.is_empty() {
                if self.in_document {
                    nodes.push(LatexNode::Text(current.to_string()));
                }

                self.pos += 1;
            } else if self.in_document && !text.trim().is_empty() {
                nodes.push(LatexNode::Text(text));
            } else if self.in_document {
                nodes.push(LatexNode::Text(" ".to_string()));
            }
        }

        nodes
    }

    fn split_items(block: &str, labels: &mut HashMap<String, String>) -> Vec<Vec<LatexNode>> {
        let mut items = Vec::new();
        for item in block.split("\\item") {
            let body = if item.trim_start().starts_with('[') {
                let end = item.find(']').map(|i| i + 1).unwrap_or(0);

                &item[end..]
            } else {
                item
            };

            let trimmed = body.trim();
            if !trimmed.is_empty() {
                items.push(Parser::new(trimmed).parse(true, labels));
            }
        }
        items
    }

    fn split_description_items(
        block: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<(String, Vec<LatexNode>)> {
        let mut items = Vec::new();
        for item in block.split("\\item") {
            let trimmed = item.trim();
            if trimmed.is_empty() { continue; }

            if trimmed.starts_with('[') {
                let end = trimmed.find(']').unwrap_or(0);
                let term = trimmed[1..end].to_string();
                let body = trimmed[end + 1..].trim();
                items.push((term, Parser::new(body).parse(true, labels)));
            } else {
                items.push((String::new(), Parser::new(trimmed).parse(true, labels)));
            }
        }

        items
    }

    fn parse_tabular(
        table_block: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<Vec<Vec<LatexNode>>> {
        let mut rows = Vec::new();
        for row_str in table_block.split(r"\\") {
            let clean_row = row_str
                .replace("\\hline", "")
                .replace("\\toprule", "")
                .replace("\\midrule", "")
                .replace("\\bottomrule", "")
                .trim()
                .to_string();
            if clean_row.is_empty() { continue; }
            let mut cells = Vec::new();
            for cell_str in clean_row.split('&') {
                cells.push(Parser::new(cell_str.trim()).parse(true, labels));
            }
            rows.push(cells);
        }
        rows
    }

    fn parse_matrix_body(
        raw: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<Vec<Vec<LatexNode>>> {
        let mut rows = Vec::new();

        for row_str in raw.split(r"\\") {
            let trimmed = row_str.trim();
            if trimmed.is_empty() { continue; }

            let mut cells = Vec::new();
            for cell in trimmed.split('&') {
                cells.push(Parser::new(cell.trim()).parse(true, labels));
            }

            rows.push(cells);
        }

        rows
    }

    fn extract_and_register_labels(
        raw: &str,
        value: &str,
        prefix: &str,
        labels: &mut HashMap<String, String>,
    ) {
        let tag = "\\label{";
        let mut pos = 0;

        while pos + tag.len() <= raw.len() {
            if raw[pos..].starts_with(tag) {
                pos += tag.len();
                let start = pos;

                while pos < raw.len() && raw.as_bytes()[pos] != b'}' {
                    pos += 1;
                }

                let key = &raw[start..pos];
                if prefix.is_empty() || key.starts_with(prefix) {
                    labels.insert(key.to_string(), value.to_string());
                }
            } else {
                pos += 1;
            }
        }
    }

}