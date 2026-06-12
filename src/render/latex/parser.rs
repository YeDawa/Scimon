use chrono::Local;
use std::collections::HashMap;

use crate::render::latex::{
    packages,
    tikz::Tikz,
    lexer::Lexer,
    bibtex::BibStyle,
    macros::MacroDef,
    pgfplots::Pgfplots,

    tex_ast::{
        AcrCaps,
        AcrForm,
        CiteKind,
        LatexNode,
        TableCell,
    },
};

// ---------------------------------------------------------------------------
// Greek letters, operators and other math symbols supported as \commands
// ---------------------------------------------------------------------------
pub fn math_symbol(cmd: &str) -> Option<&'static str> {
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
        "varnothing"  => Some("∅"),
        "infin"      => Some("∞"),
        "therefore"  => Some("∴"),
        "because"    => Some("∵"),
        "perp"       => Some("⊥"),
        "parallel"   => Some("∥"),
        "angle"      => Some("∠"),
        "triangle"   => Some("△"),

        // Variant Greek
        "varphi"     => Some("φ"),
        "varepsilon" => Some("ε"),
        "vartheta"   => Some("ϑ"),
        "varrho"     => Some("ϱ"),
        "varsigma"   => Some("ς"),
        "varpi"      => Some("ϖ"),
        "varkappa"   => Some("ϰ"),

        // Special math letters
        "imath"      => Some("ı"),
        "jmath"      => Some("ȷ"),
        "ell"        => Some("ℓ"),
        "hbar"       => Some("ℏ"),
        "wp"         => Some("℘"),
        "mho"        => Some("℧"),
        "Re"         => Some("ℜ"),
        "Im"         => Some("ℑ"),
        "aleph"      => Some("ℵ"),
        "beth"       => Some("ℶ"),
        "gimel"      => Some("ℷ"),
        "daleth"     => Some("ℸ"),

        // More delimiters
        "vert"       => Some("|"),
        "Vert"       => Some("‖"),
        "mid"        => Some("∣"),
        "nmid"       => Some("∤"),

        // Set operations
        "setminus"       => Some("∖"),
        "smallsetminus"  => Some("∖"),
        "complement"     => Some("∁"),
        "sqcup"          => Some("⊔"),
        "sqcap"          => Some("⊓"),
        "uplus"          => Some("⊎"),
        "amalg"          => Some("⨿"),

        // More relations
        "prec"       => Some("≺"),
        "succ"       => Some("≻"),
        "preceq"     => Some("⪯"),
        "succeq"     => Some("⪰"),
        "ll"         => Some("≪"),
        "gg"         => Some("≫"),
        "lll"        => Some("⋘"),
        "ggg"        => Some("⋙"),
        "asymp"      => Some("≍"),
        "bowtie"     => Some("⋈"),
        "smile"      => Some("⌣"),
        "frown"      => Some("⌢"),

        // Logic / proof
        "vdash"      => Some("⊢"),
        "dashv"      => Some("⊣"),
        "models"     => Some("⊨"),
        "vDash"      => Some("⊨"),
        "Vdash"      => Some("⊩"),
        "top"        => Some("⊤"),
        "bot"        => Some("⊥"),

        // Square order
        "sqsubset"   => Some("⊏"),
        "sqsupset"   => Some("⊐"),
        "sqsubseteq" => Some("⊑"),
        "sqsupseteq" => Some("⊒"),

        // Triangle relations
        "lhd"        => Some("⊲"),
        "rhd"        => Some("⊳"),
        "unlhd"      => Some("⊴"),
        "unrhd"      => Some("⊵"),

        // More circle ops
        "ominus"     => Some("⊖"),
        "oslash"     => Some("⊘"),
        "odot"       => Some("⊙"),
        "circledast" => Some("⊛"),
        "boxplus"    => Some("⊞"),
        "boxminus"   => Some("⊟"),
        "boxtimes"   => Some("⊠"),
        "boxdot"     => Some("⊡"),

        // Misc symbols
        "dagger"     => Some("†"),
        "ddagger"    => Some("‡"),
        "bullet"     => Some("•"),
        "star"       => Some("⋆"),
        "ast"        => Some("∗"),
        "sharp"      => Some("♯"),
        "flat"       => Some("♭"),
        "natural"    => Some("♮"),
        "checkmark"  => Some("✓"),
        "maltese"    => Some("✠"),
        "clubsuit"   => Some("♣"),
        "diamondsuit"=> Some("♦"),
        "heartsuit"  => Some("♥"),
        "spadesuit"  => Some("♠"),
        "surd"       => Some("√"),
        "backslash"  => Some("\\"),

        // Harpoons
        "rightharpoonup"   => Some("⇀"),
        "leftharpoonup"    => Some("↼"),
        "rightharpoondown" => Some("⇁"),
        "leftharpoondown"  => Some("↽"),
        "rightleftharpoons"=> Some("⇌"),
        "leftrightharpoons"=> Some("⇋"),

        // Diagonal arrows
        "nearrow"    => Some("↗"),
        "searrow"    => Some("↘"),
        "nwarrow"    => Some("↖"),
        "swarrow"    => Some("↙"),
        "leadsto"    => Some("⇝"),

        // More double arrows
        "Uparrow"    => Some("⇑"),
        "Downarrow"  => Some("⇓"),
        "Updownarrow"=> Some("⇕"),
        "updownarrow"=> Some("↕"),
        "hookleftarrow"  => Some("↩"),
        "hookrightarrow" => Some("↪"),
        "looparrowleft"  => Some("↫"),
        "looparrowright" => Some("↬"),
        "twoheadrightarrow" => Some("↠"),
        "twoheadleftarrow"  => Some("↞"),
        "rightarrowtail"    => Some("↣"),
        "leftarrowtail"     => Some("↢"),
        "multimap"   => Some("⊸"),
        "lightning"  => Some("↯"),

        // Long arrows
        "longrightarrow"      => Some("⟶"),
        "longleftarrow"       => Some("⟵"),
        "longleftrightarrow"  => Some("⟷"),
        "Longrightarrow"      => Some("⟹"),
        "Longleftarrow"       => Some("⟸"),
        "Longleftrightarrow"  => Some("⟺"),

        // Not-equal variants
        "nless"      => Some("≮"),
        "ngtr"       => Some("≯"),
        "nleq"       => Some("≰"),
        "ngeq"       => Some("≱"),
        "nsubseteq"  => Some("⊄"),
        "nsupseteq"  => Some("⊅"),
        "nprec"      => Some("⊀"),
        "nsucc"      => Some("⊁"),

        // Dots
        "dots"       => Some("…"),
        "dotsc"      => Some("…"),
        "dotsb"      => Some("⋯"),
        "dotsm"      => Some("⋯"),
        "dotsi"      => Some("…"),

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
    /// Lexical layer — owns the source text and cursor; every low-level
    /// read goes through it so escapes and nesting are handled in one place
    pub lexer: Lexer,
    pub in_document: bool,

    pub current_chapter: usize,
    pub current_section: usize,
    pub current_subsection: usize,
    pub current_table: usize,
    pub current_equation: usize,

    /// User-defined macros, compiled at definition time into structural
    /// pieces (see `macros::MacroDef`); expansion is a single splice.
    pub macros: HashMap<String, MacroDef>,

    /// Guards against runaway recursion (\def\x{\x})
    expansion_depth: usize,

    /// Current \theoremstyle, applied to subsequent \newtheorem definitions
    theorem_style: String,
}

impl Parser {

    pub fn new(input: &str) -> Self {
        Parser {
            lexer: Lexer::new(input),
            in_document: false,
            current_chapter: 0,
            current_section: 0,
            current_subsection: 0,
            current_table: 0,
            current_equation: 0,
            macros: HashMap::new(),
            expansion_depth: 0,
            theorem_style: String::from("plain"),
        }
    }

    // -----------------------------------------------------------------------
    // Character-level helpers — thin delegations into the lexical layer
    // -----------------------------------------------------------------------

    pub fn next_char(&mut self) -> Option<char> {
        self.lexer.next_char()
    }

    pub fn peek(&self) -> Option<char> {
        self.lexer.peek()
    }

    /// Look `n` chars ahead from the current position (0 = same as peek).
    pub fn peek_ahead(&self, n: usize) -> Option<char> {
        self.lexer.peek_ahead(n)
    }

    /// Read a TeX dimension token (number + optional unit) from current position.
    /// Returns the raw string, e.g. "1.5em", "-2pt", "0".
    fn read_dimension(&mut self) -> String {
        self.skip_whitespace();
        let mut s = String::new();
        // optional sign
        if matches!(self.peek(), Some('+') | Some('-')) {
            if let Some(c) = self.next_char() { s.push(c); }
        }
        self.skip_whitespace();
        // digits and decimal point
        while matches!(self.peek(), Some('0'..='9') | Some('.')) {
            if let Some(c) = self.next_char() { s.push(c); }
        }
        self.skip_whitespace();
        // unit (up to 2 letters)
        let mut unit = String::new();
        while unit.len() < 4 && matches!(self.peek(), Some('a'..='z') | Some('A'..='Z')) {
            if let Some(c) = self.next_char() { unit.push(c); }
        }
        if !unit.is_empty() { s.push_str(&unit); }
        s
    }

    /// Capitalise first character of a string.
    fn capitalise(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None    => String::new(),
            Some(f) => f.to_uppercase().to_string() + c.as_str(),
        }
    }

    /// Parse enumitem `[label=\alph*, label=(\roman*), ...]` option string
    /// and return the equivalent CSS `list-style-type` value.
    fn enumitem_label_style(opt: &str) -> String {
        // Extract `label=<value>` from option string
        let lower = opt.to_lowercase();
        let label_val = if let Some(pos) = lower.find("label=") {
            opt[pos + 6..].trim().trim_end_matches(',').trim()
        } else {
            return String::new();
        };

        // Map common patterns
        if label_val.contains("\\alph") || label_val.contains("\\alph*") {
            "lower-alpha".to_string()
        } else if label_val.contains("\\Alph") || label_val.contains("\\Alph*") {
            "upper-alpha".to_string()
        } else if label_val.contains("\\roman") {
            "lower-roman".to_string()
        } else if label_val.contains("\\Roman") {
            "upper-roman".to_string()
        } else if label_val.contains("\\arabic") {
            "decimal".to_string()
        } else if label_val.starts_with('(') {
            // e.g. (\arabic*) — use decimal in parens via CSS counter
            "decimal".to_string()
        } else {
            String::new()
        }
    }

    /// Render a LaTeX tabbing environment body to HTML.
    /// \= sets tab stops, \> advances, \kill discards a line, \\ ends line.
    fn render_tabbing(body: &str) -> String {
        let mut html = String::from("<div class=\"latex-tabbing\">");
        let mut tab_stops: Vec<usize> = Vec::new();

        for raw_line in body.split("\\\\") {
            let line = raw_line.trim();
            if line.is_empty() { continue; }

            // \kill — discard this line, keep tab stops
            if line.ends_with("\\kill") {
                let content = &line[..line.len() - 5];
                // measure tab stop positions from \= markers
                tab_stops.clear();
                let mut col: usize = 0;
                for segment in content.split("\\=") {
                    col += segment.len();
                    tab_stops.push(col);
                }
                continue;
            }

            // split by \> (advance to next tab stop)
            let segments: Vec<&str> = line.split("\\>").collect();
            html.push_str("<span class=\"tabbing-line\">");
            for (i, seg) in segments.iter().enumerate() {
                // strip \= (set tab stop marker) from segment
                let seg_clean = seg.replace("\\=", "");
                html.push_str(&seg_clean);
                if i + 1 < segments.len() {
                    // pad to next tab stop
                    let target = tab_stops.get(i).copied().unwrap_or((i + 1) * 8);
                    let current = seg_clean.len();
                    let pad = if target > current { target - current } else { 1 };
                    for _ in 0..pad {
                        html.push_str("&nbsp;");
                    }
                }
            }
            html.push_str("</span><br>");
        }

        html.push_str("</div>");
        html
    }

    /// Read an alphabetic command word (letters only) from current position.
    fn read_command_word(&mut self) -> String {
        self.lexer.command_word()
    }

    /// Skip whitespace characters without consuming them permanently.
    fn skip_whitespace(&mut self) {
        self.lexer.skip_whitespace()
    }

    // -----------------------------------------------------------------------
    // Text / content helpers
    // -----------------------------------------------------------------------

    /// Collect plain text until a special character.
    pub fn parse_text(&mut self) -> String {
        self.lexer.text_run()
    }

    /// Collect everything inside the next `{…}`, respecting nesting.
    pub fn parse_braces_content(&mut self) -> String {
        self.lexer.brace_group()
    }

    /// Parse the next argument: `{…}` (multiple chars) or a single char.
    pub fn parse_argument(&mut self) -> Vec<LatexNode> {
        self.skip_whitespace();
        if self.peek() == Some('{') {
            let raw = self.parse_braces_content();
            let mut sub = Parser::new(&raw);
            sub.macros = self.macros.clone();
            sub.parse(true, &mut HashMap::new())
        } else {
            vec![LatexNode::Text(self.next_char().unwrap_or(' ').to_string())]
        }
    }

    /// Consume an optional `[…]` argument, returning its contents.
    /// Nested brackets and braced groups are kept whole by the lexer.
    pub(crate) fn parse_optional_arg(&mut self) -> Option<String> {
        self.lexer.optional_group()
    }

    /// "key=value, key={braced, value}, flag" option lists used by
    /// \newacronym, \DeclareAcronym, \printacronyms and friends.
    /// Flags without '=' are ignored; commas inside braces are preserved.
    fn key_value_list(raw: &str) -> HashMap<String, String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut depth = 0usize;

        for c in raw.chars() {
            match c {
                '{' | '[' => { depth += 1; current.push(c); }
                '}' | ']' => { depth = depth.saturating_sub(1); current.push(c); }
                ',' if depth == 0 => parts.push(std::mem::take(&mut current)),
                _ => current.push(c),
            }
        }
        parts.push(current);

        let mut map = HashMap::new();
        for part in parts {
            if let Some((key, value)) = part.split_once('=') {
                let value = value.trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .trim();
                map.insert(key.trim().to_lowercase(), value.to_string());
            }
        }
        map
    }

    /// Read everything up to the matching `\end{env_name}`, consuming the
    /// tag. Nested same-name environments are counted by the lexer.
    pub(crate) fn read_until_end(&mut self, env_name: &str) -> String {
        self.lexer.until_env_end(env_name)
    }

    /// Consume `\OPEN ... \CLOSE` math delimiters and return the raw inner LaTeX.
    /// Expects `pos` to be positioned BEFORE the leading `\`; advances past `\CLOSE`.
    fn consume_math_block(&mut self, open: char, close: char) -> String {
        self.lexer.pos += 2; // skip `\` + open char
        let mut content = String::new();
        while self.lexer.pos < self.lexer.input.len() {
            if self.peek() == Some('\\') && self.peek_ahead(1) == Some(close) {
                self.lexer.pos += 2;
                break;
            }
            if let Some(c) = self.next_char() { content.push(c); }
        }
        let _ = open; // used only as documentation / symmetry
        content
    }

    /// Consume `$...$` (inline) or `$$...$$` (display) and return the appropriate node.
    fn consume_dollar_math(&mut self) -> LatexNode {
        self.lexer.pos += 1; // skip first `$`
        let display = self.peek() == Some('$');
        if display { self.lexer.pos += 1; }

        let mut content = String::new();
        while let Some(c) = self.peek() {
            if c == '$' {
                self.lexer.pos += 1;
                if display && self.peek() == Some('$') { self.lexer.pos += 1; }
                break;
            }
            content.push(c);
            self.lexer.pos += c.len_utf8();
        }

        if display { LatexNode::RawMathDisplay(content) } else { LatexNode::RawMathInline(content) }
    }

    // -----------------------------------------------------------------------
    // Main parse loop
    // -----------------------------------------------------------------------

    pub fn parse(&mut self, force_active: bool, labels: &mut HashMap<String, String>) -> Vec<LatexNode> {
        let mut nodes: Vec<LatexNode> = Vec::new();
        if force_active { self.in_document = true; }

        while self.lexer.pos < self.lexer.input.len() {
            let current = match self.peek() { Some(c) => c, None => break };

            // ----------------------------------------------------------------
            // Comments  % ... \n
            // ----------------------------------------------------------------
            if current == '%' {
                while self.peek().map_or(false, |c| c != '\n') {
                    self.next_char();
                }
                continue;
            }

            // ----------------------------------------------------------------
            // Non-breaking space  ~
            // ----------------------------------------------------------------
            if current == '~' && self.in_document {
                self.lexer.pos += 1;
                nodes.push(LatexNode::Text("\u{00A0}".to_string())); // &nbsp;
                continue;
            }

            // ----------------------------------------------------------------
            // Display math  \[ ... \]
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('[')
            {
                nodes.push(LatexNode::RawMathDisplay(self.consume_math_block('[', ']')));
                continue;
            }

            // ----------------------------------------------------------------
            // Inline math  \( ... \)
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('(')
            {
                nodes.push(LatexNode::RawMathInline(self.consume_math_block('(', ')')));
                continue;
            }

            // ----------------------------------------------------------------
            // Double backslash  \\  → line break (outside verbatim)
            // ----------------------------------------------------------------
            if current == '\\' && self.in_document
                && self.peek_ahead(1) == Some('\\')
            {
                self.lexer.pos += 2;
                nodes.push(LatexNode::LineBreak);
                continue;
            }

            // ----------------------------------------------------------------
            // Command  \name
            // ----------------------------------------------------------------
            if current == '\\' {
                self.lexer.pos += 1;
                let mut command = String::new();

                // Special single-char commands like \{ \} \_ etc.
                if let Some(nc) = self.peek() {
                    if !nc.is_alphabetic() {
                        self.lexer.pos += nc.len_utf8(); // consume nc

                        // Accent commands: read the base letter from {x} or bare x
                        if matches!(nc, '\'' | '`' | '"' | '^' | '~' | '=' | '.') && self.in_document {
                            let base = if self.peek() == Some('{') {
                                self.parse_braces_content()
                            } else if self.peek().map_or(false, |c| c.is_alphabetic()) {
                                self.next_char().map(|c| c.to_string()).unwrap_or_default()
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

                while let Some(c) = self.peek() {
                    if c.is_alphabetic() { command.push(c); self.lexer.pos += c.len_utf8(); }
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
                    "pagestyle" | "geometry" | "hypersetup" => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                    }

                    // \documentclass[twocolumn]{article} — class options
                    "documentclass" => {
                        let options = self.parse_optional_arg().unwrap_or_default();
                        self.parse_braces_content();
                        if options.split(',').any(|option| option.trim() == "twocolumn") {
                            nodes.push(LatexNode::DocumentColumns(2));
                        }
                    }

                    // \twocolumn[preface] / \onecolumn — switch the document
                    "twocolumn" => {
                        nodes.push(LatexNode::NewPage);
                        nodes.push(LatexNode::DocumentColumns(2));
                        if let Some(preface) = self.parse_optional_arg() {
                            nodes.push(LatexNode::Text(
                                "<div style=\"column-span: all; margin-bottom: 8px;\">".to_string()
                            ));
                            nodes.extend(Parser::new(preface.trim()).parse(true, labels));
                            nodes.push(LatexNode::Text("</div>".to_string()));
                        }
                    }
                    "onecolumn" => {
                        nodes.push(LatexNode::NewPage);
                        nodes.push(LatexNode::DocumentColumns(1));
                    }

                    // \usepackage[style=...]{biblatex} selects the citation style
                    "usepackage" => {
                        let options = self.parse_optional_arg().unwrap_or_default();
                        let package = self.parse_braces_content();

                        if package.contains("biblatex") {
                            for option in options.split(',') {
                                let mut parts = option.splitn(2, '=');
                                let name = parts.next().unwrap_or("").trim();
                                let value = parts.next().unwrap_or("").trim();

                                if matches!(name, "style" | "citestyle" | "bibstyle") {
                                    if let Some(style) = BibStyle::from_name(value) {
                                        nodes.push(LatexNode::BibStyleSet(style));
                                    }
                                }
                            }
                        }
                    }

                    // \setcounter{name}{value}
                    "setcounter" | "stepcounter" | "refstepcounter" | "addtocounter" => {
                        self.parse_braces_content(); // counter name
                        if command == "setcounter" || command == "addtocounter" {
                            self.parse_braces_content(); // value
                        }
                        // counter mutation not tracked in HTML rendering
                    }

                    // \definecolor{name}{model}{spec}
                    "definecolor" | "providecolor" | "colorlet" => {
                        let name  = self.parse_braces_content();
                        let model = self.parse_braces_content();
                        let spec  = self.parse_braces_content();
                        let css   = Self::latex_color_model(&model, &spec);
                        nodes.push(LatexNode::DefineColor { name, css });
                    }

                    // \DeclareMathOperator{\cmd}{name}
                    "DeclareMathOperator" => {
                        let raw_name = self.parse_braces_content();
                        let display  = self.parse_braces_content();
                        let op_name  = raw_name.trim_start_matches('\\').to_string();
                        if !op_name.is_empty() {
                            self.macros.insert(op_name, MacroDef::compile(
                                0, None, &format!("\\operatorname{{{}}}", display),
                            ));
                        }
                    }

                    // \theoremstyle{plain|definition|remark} — applies to
                    // every \newtheorem that follows
                    "theoremstyle" => {
                        self.theorem_style = self.parse_braces_content();
                    }

                    // \newtheorem{env}{Label}[parent] — numbered within parent
                    // \newtheorem{env}[shared]{Label} — shares shared's counter
                    // \newtheorem*{env}{Label}        — unnumbered
                    "newtheorem" => {
                        let env    = self.parse_braces_content();
                        let shared = self.parse_optional_arg();
                        let title  = self.parse_braces_content();
                        let mut parent = self.parse_optional_arg().unwrap_or_default();

                        let counter = if starred {
                            String::new()
                        } else if let Some(sibling) = shared {
                            // sharing a counter also inherits its numbering parent
                            if let Some(def) = labels.get(&format!("thm@{}", sibling)) {
                                parent = def.splitn(4, '|').nth(2).unwrap_or("").to_string();
                            }
                            sibling
                        } else {
                            env.clone()
                        };

                        labels.insert(
                            format!("thm@{}", env),
                            format!("{}|{}|{}|{}", title, counter, parent, self.theorem_style),
                        );
                    }

                    // \newenvironment{name}[n][default]{begin-code}{end-code}
                    "newenvironment" | "renewenvironment" => {
                        let env_name  = self.parse_braces_content();
                        let n_args    = self.parse_optional_arg()
                            .and_then(|s| s.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                        let default   = if n_args > 0 { self.parse_optional_arg() } else { None };
                        let begin_code = self.parse_braces_content();
                        let end_code   = self.parse_braces_content();
                        // Store as two macros: env@begin@name and env@end@name
                        self.macros.insert(
                            format!("env@begin@{}", env_name),
                            MacroDef::compile(n_args, default, &begin_code),
                        );
                        self.macros.insert(
                            format!("env@end@{}", env_name),
                            MacroDef::compile(0, None, &end_code),
                        );
                    }

                    // \newcolumntype{X}[n]{spec}
                    "newcolumntype" => {
                        self.parse_braces_content();
                        self.parse_optional_arg();
                        self.parse_braces_content();
                    }

                    // \captionsetup, \floatname, \floatsetup — consume silently
                    "captionsetup" | "floatname" | "floatsetup" => {
                        self.parse_optional_arg();
                        self.parse_braces_content();
                        self.parse_optional_arg();
                    }

                    // \setlength{\param}{value} — emit a CSS custom-property
                    // block; preamble values matter too (\columnsep et al.)
                    "setlength" | "addtolength" => {
                        let param = self.parse_braces_content(); // e.g. \parskip
                        let raw   = self.parse_braces_content(); // e.g. 6pt or \fill
                        let value = Self::length_to_css(&raw);
                        nodes.push(LatexNode::SetLength { param, value });
                    }

                    // \newcommand{\name}[n][default]{body} — with [default],
                    // #1 becomes optional
                    "newcommand" | "providecommand" | "renewcommand" => {
                        let raw_name = self.parse_braces_content(); // e.g. \myCmd
                        let name     = raw_name.trim_start_matches('\\').to_string();
                        let params: usize = self.parse_optional_arg()
                            .and_then(|s| s.trim().parse().ok())
                            .unwrap_or(0);
                        let default = if params > 0 { self.parse_optional_arg() } else { None };
                        let body = self.parse_braces_content();

                        // \providecommand only defines when not yet defined
                        let keep_existing = command == "providecommand"
                            && self.macros.contains_key(&name);
                        if !name.is_empty() && !keep_existing {
                            self.macros.insert(name, MacroDef::compile(params, default, &body));
                        }
                    }

                    // \def\name#1#2{body} — parameter text scanned for slots;
                    // delimiter characters between them are ignored
                    "def" => {
                        self.skip_whitespace();
                        if self.peek() == Some('\\') {
                            self.lexer.pos += 1;
                            let name = self.read_command_word();

                            let mut params = 0usize;
                            while let Some(c) = self.peek() {
                                if c == '{' { break; }
                                self.lexer.pos += c.len_utf8();
                                if c == '#' {
                                    if let Some(digit) = self.peek().and_then(|d| d.to_digit(10)) {
                                        self.lexer.pos += 1;
                                        params = params.max(digit as usize);
                                    }
                                }
                            }

                            let body = self.parse_braces_content();
                            if !name.is_empty() {
                                self.macros.insert(name, MacroDef::compile(params, None, &body));
                            }
                        }
                    }

                    "let" => {
                        // \let\new=\existing — snapshot the current meaning
                        self.skip_whitespace();
                        if self.peek() == Some('\\') {
                            self.lexer.pos += 1;
                            let new_name = self.read_command_word();
                            self.skip_whitespace();
                            if self.peek() == Some('=') { self.lexer.pos += 1; }
                            self.skip_whitespace();
                            if self.peek() == Some('\\') {
                                self.lexer.pos += 1;
                                let existing = self.read_command_word();
                                let def = self.macros.get(&existing).cloned()
                                    .unwrap_or_else(|| MacroDef::compile(
                                        0, None, &format!("\\{}", existing),
                                    ));
                                self.macros.insert(new_name, def);
                            }
                        }
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
                        // [htbp] placement, theorem notes, etc. — forwarded so
                        // environments that care about it can use it
                        let opt = self.parse_optional_arg();
                        nodes.extend(self.parse_environment(&env, opt, labels));
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

                    "part" if self.in_document => {
                        self.parse_optional_arg();
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!("<div class=\"latex-part\"><span class=\"part-title\">{}</span></div>", title)));
                        } else {
                            nodes.push(LatexNode::Part(title));
                        }
                    }

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
                        let title = self.parse_braces_content();
                        if starred {
                            nodes.push(LatexNode::Text(format!(
                                "<h6 class=\"section-star\">{}</h6>", title
                            )));
                        } else {
                            nodes.push(LatexNode::Paragraph(title));
                        }
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
                    "sout" | "st" | "strikethrough" | "xout" if self.in_document => {
                        nodes.push(LatexNode::Strikethrough(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "uline" | "uuline" if self.in_document => {
                        nodes.push(LatexNode::Underline(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "uwave" if self.in_document => {
                        let inner = Parser::new(&self.parse_braces_content()).parse(true, labels);
                        nodes.push(LatexNode::Text("<span class=\"uwave\">".to_string()));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }
                    "dashuline" if self.in_document => {
                        let inner = Parser::new(&self.parse_braces_content()).parse(true, labels);
                        nodes.push(LatexNode::Text("<span class=\"dashuline\">".to_string()));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }
                    "dotuline" if self.in_document => {
                        let inner = Parser::new(&self.parse_braces_content()).parse(true, labels);
                        nodes.push(LatexNode::Text("<span class=\"dotuline\">".to_string()));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }
                    "textsuper" | "textsuperscript" if self.in_document => {
                        nodes.push(LatexNode::Superscript(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "textsub" | "textsubscript" if self.in_document => {
                        nodes.push(LatexNode::Subscript(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        ));
                    }
                    "textrm" | "mathrm" | "textnormal" | "textmd" | "textup" if self.in_document => {
                        nodes.extend(
                            Parser::new(&self.parse_braces_content()).parse(true, labels)
                        );
                    }

                    "textsf" | "mathsf" if self.in_document => {
                        let inner = Parser::new(&self.parse_braces_content()).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"font-family: sans-serif;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    "textsl" if self.in_document => {
                        let inner = Parser::new(&self.parse_braces_content()).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"font-style: oblique;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // Font declaration commands (no braces — apply to rest of group)
                    // --------------------------------------------------------
                    "itshape" | "slshape" | "bfseries" | "ttfamily"
                    | "sffamily" | "rmfamily" | "upshape" | "scshape"
                    | "normalfont"
                    if self.in_document => {
                        // Collect remaining content up to the enclosing } or end
                        let rest = self.collect_until_close_brace();
                        let inner = Parser::new(&rest).parse(true, labels);
                        nodes.push(LatexNode::FontDecl {
                            style: command.clone(),
                            nodes: inner,
                        });
                    }

                    // --------------------------------------------------------
                    // \color{name}  — declaration (scoped to enclosing group)
                    // --------------------------------------------------------
                    "color" if self.in_document => {
                        let color = self.parse_braces_content();
                        let rest  = self.collect_until_close_brace();
                        let inner = Parser::new(&rest).parse(true, labels);
                        nodes.push(LatexNode::ColorDecl { color, nodes: inner });
                    }

                    // --------------------------------------------------------
                    // \parbox[pos]{width}{content}
                    // --------------------------------------------------------
                    "parbox" if self.in_document => {
                        self.parse_optional_arg(); // [pos]
                        let w   = Self::conv_width(&self.parse_braces_content());
                        let raw = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Parbox { width: w, nodes: inner });
                    }

                    // --------------------------------------------------------
                    // \raisebox{lift}[h][d]{content}
                    // --------------------------------------------------------
                    "raisebox" if self.in_document => {
                        let lift = Self::conv_width(&self.parse_braces_content());
                        self.parse_optional_arg(); // [height]
                        self.parse_optional_arg(); // [depth]
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Raisebox { lift, nodes: inner });
                    }

                    // --------------------------------------------------------
                    // \scalebox \rotatebox \resizebox — consume, render content
                    // --------------------------------------------------------
                    "scalebox" if self.in_document => {
                        self.parse_braces_content(); // scale
                        self.parse_optional_arg();   // [yscale]
                        let raw = self.parse_braces_content();
                        nodes.extend(Parser::new(&raw).parse(true, labels));
                    }
                    "rotatebox" if self.in_document => {
                        self.parse_optional_arg();   // [origin]
                        self.parse_braces_content(); // angle
                        let raw = self.parse_braces_content();
                        nodes.extend(Parser::new(&raw).parse(true, labels));
                    }
                    "resizebox" if self.in_document => {
                        self.parse_braces_content(); // width
                        self.parse_braces_content(); // height
                        let raw = self.parse_braces_content();
                        nodes.extend(Parser::new(&raw).parse(true, labels));
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
                            let rest = self.lexer.input[self.lexer.pos..].to_string();
                            self.lexer.pos = self.lexer.input.len();
                            nodes.push(LatexNode::FontSize(
                                command.clone(),
                                Parser::new(&rest).parse(true, labels)
                            ));
                        }
                    }

                    // --------------------------------------------------------
                    // Math formatting
                    // --------------------------------------------------------
                    "math" | "ensuremath" if self.in_document =>
                        nodes.push(LatexNode::RawMathInline(self.parse_braces_content())),

                    // \nonumber / \notag — suppress equation number in align
                    "nonumber" | "notag" if self.in_document => {
                        // Inject \notag into the last RawMathDisplay node if present,
                        // otherwise emit nothing.
                        if let Some(LatexNode::RawMathDisplay(s)) = nodes.last_mut() {
                            s.push_str("\\notag");
                        }
                    }

                    // \dfrac \tfrac \cfrac → raw KaTeX
                    "dfrac" | "tfrac" | "cfrac" if self.in_document => {
                        let num = self.parse_braces_content();
                        let den = self.parse_braces_content();
                        nodes.push(LatexNode::RawMathInline(
                            format!("\\{}{{{}}}{{{}}}",  command, num, den)
                        ));
                    }

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

                    "overline" | "hat" | "bar"
                    | "vec" | "dot" | "ddot" | "tilde" | "widehat" | "widetilde"
                    | "overrightarrow" | "overleftarrow" | "breve" | "check"
                    | "acute" | "grave" | "mathring" | "underline" | "underbrace"
                    | "overbrace" if self.in_document => {
                        nodes.extend(self.parse_argument());
                    }

                    "left" | "right" if self.in_document => {
                        // consume the delimiter — single char OR a \command
                        self.skip_whitespace();
                        if self.peek() == Some('\\') {
                            self.lexer.pos += 1; // skip backslash
                            self.read_command_word(); // e.g. rangle, lfloor, vert …
                        } else {
                            self.next_char(); // (, ), [, ], |, . etc.
                        }
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

                    // --------------------------------------------------------
                    // \overbrace{expr}^{label}
                    // --------------------------------------------------------
                    "overbrace" if self.in_document => {
                        let content = self.parse_argument();
                        self.skip_whitespace();
                        let has_label = self.peek() == Some('^');
                        if has_label { self.next_char(); }
                        let label = if has_label { self.parse_argument() } else { vec![] };

                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-overbrace\">".to_string()
                        ));
                        if !label.is_empty() {
                            nodes.push(LatexNode::Text(
                                "<span class=\"overbrace-label\">".to_string()
                            ));
                            nodes.extend(label);
                            nodes.push(LatexNode::Text("</span>".to_string()));
                        }
                        nodes.push(LatexNode::Text(
                            "<span class=\"overbrace-content\">".to_string()
                        ));
                        nodes.extend(content);
                        nodes.push(LatexNode::Text("</span></span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \underbrace{expr}_{label}
                    // --------------------------------------------------------
                    "underbrace" if self.in_document => {
                        let content = self.parse_argument();
                        self.skip_whitespace();
                        let has_label = self.peek() == Some('_');
                        if has_label { self.next_char(); }
                        let label = if has_label { self.parse_argument() } else { vec![] };

                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-underbrace\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"underbrace-content\">".to_string()
                        ));
                        nodes.extend(content);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                        if !label.is_empty() {
                            nodes.push(LatexNode::Text(
                                "<span class=\"underbrace-label\">".to_string()
                            ));
                            nodes.extend(label);
                            nodes.push(LatexNode::Text("</span>".to_string()));
                        }
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \overset{top}{base}  /  \underset{bottom}{base}
                    // --------------------------------------------------------
                    "overset" if self.in_document => {
                        let top  = self.parse_argument();
                        let base = self.parse_argument();
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-stackrel\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"stackrel-top\">".to_string()
                        ));
                        nodes.extend(top);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                        nodes.push(LatexNode::Text(
                            "<span class=\"stackrel-base\">".to_string()
                        ));
                        nodes.extend(base);
                        nodes.push(LatexNode::Text("</span></span>".to_string()));
                    }

                    "underset" if self.in_document => {
                        let bottom = self.parse_argument();
                        let base   = self.parse_argument();
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-underbrace\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"underbrace-content\">".to_string()
                        ));
                        nodes.extend(base);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                        nodes.push(LatexNode::Text(
                            "<span class=\"underbrace-label\">".to_string()
                        ));
                        nodes.extend(bottom);
                        nodes.push(LatexNode::Text("</span></span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \stackrel{top}{base}
                    // --------------------------------------------------------
                    "stackrel" if self.in_document => {
                        let top  = self.parse_argument();
                        let base = self.parse_argument();
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-stackrel\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"stackrel-top\">".to_string()
                        ));
                        nodes.extend(top);
                        nodes.push(LatexNode::Text(
                            "</span><span class=\"stackrel-base\">".to_string()
                        ));
                        nodes.extend(base);
                        nodes.push(LatexNode::Text("</span></span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \binom{n}{k}
                    // --------------------------------------------------------
                    "binom" | "dbinom" | "tbinom" if self.in_document => {
                        let top = self.parse_argument();
                        let bot = self.parse_argument();
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-binom\">\
                             <span class=\"binom-paren\">(</span>\
                             <span class=\"binom-stack\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"binom-top\">".to_string()
                        ));
                        nodes.extend(top);
                        nodes.push(LatexNode::Text(
                            "</span><span class=\"binom-bot\">".to_string()
                        ));
                        nodes.extend(bot);
                        nodes.push(LatexNode::Text(
                            "</span></span>\
                             <span class=\"binom-paren\">)</span></span>".to_string()
                        ));
                    }

                    // --------------------------------------------------------
                    // {n \choose k}  — old-style binomial
                    // --------------------------------------------------------
                    "choose" if self.in_document => {
                        // nodes collected so far = numerator; rest of input = denominator
                        let num = std::mem::take(&mut nodes);
                        let den_raw = self.lexer.input[self.lexer.pos..].to_string();
                        self.lexer.pos = self.lexer.input.len();
                        let den = Parser::new(&den_raw).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-binom\">\
                             <span class=\"binom-paren\">(</span>\
                             <span class=\"binom-stack\">".to_string()
                        ));
                        nodes.push(LatexNode::Text(
                            "<span class=\"binom-top\">".to_string()
                        ));
                        nodes.extend(num);
                        nodes.push(LatexNode::Text(
                            "</span><span class=\"binom-bot\">".to_string()
                        ));
                        nodes.extend(den);
                        nodes.push(LatexNode::Text(
                            "</span></span>\
                             <span class=\"binom-paren\">)</span></span>".to_string()
                        ));
                    }

                    // --------------------------------------------------------
                    // \xrightarrow[below]{above}  /  \xleftarrow[below]{above}
                    // --------------------------------------------------------
                    "xrightarrow" | "xleftarrow"
                    | "xRightarrow" | "xLeftarrow"
                    | "xleftrightarrow" | "xLeftrightarrow"
                    if self.in_document => {
                        let below_raw = self.parse_optional_arg().unwrap_or_default();
                        let above_raw = self.parse_braces_content();
                        let above = Parser::new(&above_raw).parse(true, labels);
                        let below = if below_raw.is_empty() {
                            vec![]
                        } else {
                            Parser::new(&below_raw).parse(true, labels)
                        };
                        let arrow = match command.as_str() {
                            "xrightarrow"      => "⟶",
                            "xleftarrow"       => "⟵",
                            "xRightarrow"      => "⟹",
                            "xLeftarrow"       => "⟸",
                            "xleftrightarrow"  => "⟷",
                            "xLeftrightarrow"  => "⟺",
                            _                  => "→",
                        };
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-xarrow\">".to_string()
                        ));
                        if !above.is_empty() {
                            nodes.push(LatexNode::Text(
                                "<span class=\"xarrow-above\">".to_string()
                            ));
                            nodes.extend(above);
                            nodes.push(LatexNode::Text("</span>".to_string()));
                        }
                        nodes.push(LatexNode::Text(
                            format!("<span class=\"xarrow-sym\">{}</span>", arrow)
                        ));
                        if !below.is_empty() {
                            nodes.push(LatexNode::Text(
                                "<span class=\"xarrow-below\">".to_string()
                            ));
                            nodes.extend(below);
                            nodes.push(LatexNode::Text("</span>".to_string()));
                        }
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \substack{a \\ b \\ c}
                    // --------------------------------------------------------
                    "substack" if self.in_document => {
                        let raw = self.parse_braces_content();
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-substack\">".to_string()
                        ));
                        for line in raw.split(r"\\") {
                            let trimmed = line.trim();
                            if trimmed.is_empty() { continue; }
                            nodes.push(LatexNode::Text(
                                "<span class=\"substack-line\">".to_string()
                            ));
                            nodes.extend(Parser::new(trimmed).parse(true, labels));
                            nodes.push(LatexNode::Text("</span>".to_string()));
                        }
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \mathbb{R} — blackboard bold → Unicode
                    // --------------------------------------------------------
                    "mathbb" if self.in_document => {
                        let ch = self.parse_braces_content();
                        let sym = match ch.as_str() {
                            "N" => "ℕ", "Z" => "ℤ", "Q" => "ℚ", "R" => "ℝ",
                            "C" => "ℂ", "H" => "ℍ", "P" => "ℙ",
                            "A" => "𝔸", "B" => "𝔹", "D" => "𝔻", "E" => "𝔼",
                            "F" => "𝔽", "G" => "𝔾", "I" => "𝕀", "J" => "𝕁",
                            "K" => "𝕂", "L" => "𝕃", "M" => "𝕄", "O" => "𝕆",
                            "S" => "𝕊", "T" => "𝕋", "U" => "𝕌", "V" => "𝕍",
                            "W" => "𝕎", "X" => "𝕏", "Y" => "𝕐",
                            _ => &ch,
                        };
                        nodes.push(LatexNode::Text(sym.to_string()));
                    }

                    // --------------------------------------------------------
                    // \mathcal{L} — calligraphic → Unicode script letters
                    // --------------------------------------------------------
                    "mathcal" | "mathscr" if self.in_document => {
                        let ch = self.parse_braces_content();
                        let sym = match ch.as_str() {
                            "A" => "𝒜", "B" => "ℬ", "C" => "𝒞", "D" => "𝒟",
                            "E" => "ℰ", "F" => "ℱ", "G" => "𝒢", "H" => "ℋ",
                            "I" => "ℐ", "J" => "𝒥", "K" => "𝒦", "L" => "ℒ",
                            "M" => "ℳ", "N" => "𝒩", "O" => "𝒪", "P" => "𝒫",
                            "Q" => "𝒬", "R" => "ℛ", "S" => "𝒮", "T" => "𝒯",
                            "U" => "𝒰", "V" => "𝒱", "W" => "𝒲", "X" => "𝒳",
                            "Y" => "𝒴", "Z" => "𝒵",
                            _ => &ch,
                        };
                        nodes.push(LatexNode::Text(sym.to_string()));
                    }

                    // --------------------------------------------------------
                    // \mathfrak{g} — Fraktur letters
                    // --------------------------------------------------------
                    "mathfrak" if self.in_document => {
                        let ch = self.parse_braces_content();
                        let sym = match ch.as_str() {
                            "a" => "𝔞", "b" => "𝔟", "c" => "𝔠", "d" => "𝔡",
                            "e" => "𝔢", "f" => "𝔣", "g" => "𝔤", "h" => "𝔥",
                            "i" => "𝔦", "j" => "𝔧", "k" => "𝔨", "l" => "𝔩",
                            "m" => "𝔪", "n" => "𝔫", "o" => "𝔬", "p" => "𝔭",
                            "q" => "𝔮", "r" => "𝔯", "s" => "𝔰", "t" => "𝔱",
                            "u" => "𝔲", "v" => "𝔳", "w" => "𝔴", "x" => "𝔵",
                            "y" => "𝔶", "z" => "𝔷",
                            "A" => "𝔄", "B" => "𝔅", "C" => "ℭ", "D" => "𝔇",
                            "E" => "𝔈", "F" => "𝔉", "G" => "𝔊", "H" => "ℌ",
                            "I" => "ℑ", "J" => "𝔍", "K" => "𝔎", "L" => "𝔏",
                            "M" => "𝔐", "N" => "𝔑", "O" => "𝔒", "P" => "𝔓",
                            "Q" => "𝔔", "R" => "ℜ", "S" => "𝔖", "T" => "𝔗",
                            "U" => "𝔘", "V" => "𝔙", "W" => "𝔚", "X" => "𝔛",
                            "Y" => "𝔜", "Z" => "ℨ",
                            _ => &ch,
                        };
                        nodes.push(LatexNode::Text(sym.to_string()));
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

                    "columnbreak" | "newcolumn" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<div style=\"break-after:column;\"></div>".to_string()
                        )),

                    "clearpage" if self.in_document =>
                        nodes.push(LatexNode::NewPage),

                    // continue on the next odd (right-hand) page
                    "cleardoublepage" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<div style=\"break-after: right; page-break-after: right;\"></div>".to_string()
                        )),

                    // \enlargethispage{1cm} — no HTML equivalent, consume
                    "enlargethispage" => { self.parse_braces_content(); }

                    "noindent" | "indent" | "centering" | "raggedright"
                    | "raggedleft" | "smallskip" | "medskip" | "bigskip" => {}

                    "newline" | "linebreak" if self.in_document => {
                        self.parse_optional_arg(); // \linebreak[n] optional penalty
                        nodes.push(LatexNode::LineBreak);
                    }
                    "nopagebreak" | "nolinebreak" if self.in_document => {
                        self.parse_optional_arg(); // optional penalty
                        // No visual output — hint only
                    }
                    "newpage" | "pagebreak" if self.in_document => {
                        self.parse_optional_arg(); // \pagebreak[n] priority
                        nodes.push(LatexNode::NewPage);
                    }

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

                    // \path{...} / \nolinkurl{...} — verbatim path without hyperlink
                    "path" | "nolinkurl" if self.in_document => {
                        let raw = self.parse_braces_content();
                        nodes.push(LatexNode::Text(
                            format!("<code class=\"latex-path\">{}</code>",
                                raw.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"))
                        ));
                    }

                    "href" if self.in_document => {
                        let link_url = self.parse_braces_content();
                        let link_text = self.parse_argument();
                        nodes.push(LatexNode::Href { url: link_url, text: link_text });
                    }

                    // \hypertarget{name}{text}
                    "hypertarget" if self.in_document => {
                        let name = self.parse_braces_content();
                        let raw  = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::HyperTarget { name, nodes: inner });
                    }

                    // \hyperlink{name}{text}
                    "hyperlink" if self.in_document => {
                        let name = self.parse_braces_content();
                        let raw  = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::HyperLink { name, nodes: inner });
                    }

                    // \phantomsection — invisible anchor
                    "phantomsection" if self.in_document =>
                        nodes.push(LatexNode::PhantomSection),

                    // --------------------------------------------------------
                    // Citations & bibliography
                    // --------------------------------------------------------
                    "cite" | "citep" | "citet" | "citealt" | "citealp"
                    | "citeauthor" | "citeyear"
                    // biblatex variants
                    | "parencite" | "textcite" | "autocite" | "Autocite"
                    | "footcite" | "footcitetext" | "smartcite"
                    | "citetitle" | "citeurl" | "citedate" | "citenum"
                    | "fullcite" | "volcite" | "Citet" | "Citep"
                    if self.in_document =>
                    {
                        let kind = match command.as_str() {
                            "citet" | "Citet" | "citealt" | "textcite" => CiteKind::Text,
                            "citeauthor"                               => CiteKind::Author,
                            "citeyear" | "citedate"                    => CiteKind::Year,
                            "citetitle"                                => CiteKind::Title,
                            "fullcite" | "volcite"                     => CiteKind::Full,
                            "footcite" | "footcitetext"                => CiteKind::Foot,
                            _                                          => CiteKind::Paren,
                        };

                        // one optional arg is a postnote; two are prenote + postnote
                        let unties = |note: String| note.replace('~', "\u{a0}");
                        let (prenote, postnote) = match (self.parse_optional_arg(), self.parse_optional_arg()) {
                            (Some(pre), Some(post)) => (Some(unties(pre)), Some(unties(post))),
                            (Some(post), None)      => (None, Some(unties(post))),
                            _                       => (None, None),
                        };

                        let raw = self.parse_braces_content();
                        let keys: Vec<String> = raw.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        nodes.push(LatexNode::Cite { keys, kind, prenote, postnote });
                    }

                    // --------------------------------------------------------
                    // Acronyms (acro / acronym / glossaries packages)
                    // Resolved at render time so definitions survive across
                    // environments and \ac gets first-use expansion.
                    // --------------------------------------------------------
                    // \newacronym[longplural=...]{label}{short}{long}
                    "newacronym" => {
                        let options = self.parse_optional_arg().unwrap_or_default();
                        let keys = Self::key_value_list(&options);
                        nodes.push(LatexNode::AcronymDef {
                            label: self.parse_braces_content(),
                            short: self.parse_braces_content(),
                            long:  self.parse_braces_content(),
                            short_plural: keys.get("shortplural").cloned(),
                            long_plural:  keys.get("longplural").cloned(),
                        });
                    }

                    // \newglossaryentry{label}{name=..., description=...}
                    // Plain glossary entries print their name; the empty long
                    // form keeps \gls from expanding them like an acronym.
                    "newglossaryentry" => {
                        let label = self.parse_braces_content();
                        let keys = Self::key_value_list(&self.parse_braces_content());
                        nodes.push(LatexNode::AcronymDef {
                            short: keys.get("name").cloned().unwrap_or_else(|| label.clone()),
                            long:  String::new(),
                            short_plural: keys.get("plural").cloned(),
                            long_plural:  None,
                            label,
                        });
                    }

                    // acro: \DeclareAcronym{label}{short=..., long=...}
                    "DeclareAcronym" => {
                        let label = self.parse_braces_content();
                        let keys = Self::key_value_list(&self.parse_braces_content());
                        let short = keys.get("short").cloned().unwrap_or_else(|| label.clone());
                        let long  = keys.get("long").cloned().unwrap_or_default();

                        // *-plural keys are suffixes; *-plural-form replaces the word
                        let short_plural = keys.get("short-plural-form").cloned()
                            .or_else(|| keys.get("short-plural").map(|sfx| format!("{}{}", short, sfx)));
                        let long_plural = keys.get("long-plural-form").cloned()
                            .or_else(|| keys.get("long-plural").map(|sfx| format!("{}{}", long, sfx)));

                        nodes.push(LatexNode::AcronymDef {
                            label, short, long, short_plural, long_plural,
                        });
                    }

                    // acronym package: \acro{label}[short]{long}
                    "acro" | "acrodef" => {
                        let label = self.parse_braces_content();
                        let short = self.parse_optional_arg().unwrap_or_else(|| label.clone());
                        nodes.push(LatexNode::AcronymDef {
                            long: self.parse_braces_content(),
                            short_plural: None,
                            long_plural:  None,
                            label, short,
                        });
                    }

                    // \ac, \acs, \acl, \acf, \gls and variants
                    "ac"  | "Ac"  | "acp"  | "Acp"
                    | "acs" | "Acs" | "acsp"
                    | "acl" | "Acl" | "aclp"
                    | "acf" | "Acf" | "acfp"
                    | "acr" | "acrshort" | "acrlong" | "acrfull"
                    | "gls" | "Gls" | "GLS" | "glspl" | "Glspl" | "GLSpl"
                    | "glsshort" | "glslong" | "glsfull"
                    | "glsentrytext" | "glsentryshort" | "glsentrylong" | "glsentryfull"
                    if self.in_document => {
                        self.parse_optional_arg();
                        let label = self.parse_braces_content();
                        let lower = command.to_lowercase();

                        let form = if lower.contains("full")
                            || matches!(lower.as_str(), "acf" | "acfp")
                        {
                            AcrForm::Full
                        } else if lower.contains("long")
                            || matches!(lower.as_str(), "acl" | "aclp")
                        {
                            AcrForm::Long
                        } else if lower.contains("short") || lower.contains("text")
                            || matches!(lower.as_str(), "acs" | "acsp")
                        {
                            AcrForm::Short
                        } else {
                            AcrForm::Auto
                        };

                        let plural = lower.ends_with("pl")
                            || matches!(lower.as_str(), "acp" | "acsp" | "acfp" | "aclp");

                        let caps = if command.starts_with("GLS") {
                            AcrCaps::All
                        } else if command.chars().next().is_some_and(char::is_uppercase) {
                            AcrCaps::First
                        } else {
                            AcrCaps::No
                        };

                        nodes.push(LatexNode::Acronym { label, form, plural, caps });
                    }

                    "acresetall" | "glsresetall" => nodes.push(LatexNode::AcronymReset),

                    // \printacronyms[name=...] / \printglossary[title=...]
                    "printglossaries" | "printglossary" | "printacronyms" => {
                        let options = self.parse_optional_arg().unwrap_or_default();
                        let keys = Self::key_value_list(&options);
                        let title = keys.get("title").or_else(|| keys.get("name")).cloned();
                        nodes.push(LatexNode::PrintAcronyms { title });
                    }

                    "printindex" | "makeindex" | "makeglossaries" => {}
                    // \index{entry} — consume silently
                    "index" | "glossary" => { self.parse_braces_content(); }

                    "nocite" if self.in_document => {
                        let raw = self.parse_braces_content();
                        let keys: Vec<String> = raw.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        nodes.push(LatexNode::NoCite(keys));
                    }

                    "bibliography" if self.in_document =>
                        nodes.push(LatexNode::Bibliography {
                            file: self.parse_braces_content(),
                            title: None,
                        }),

                    // biblatex: \printbibliography[title=..., heading=..., etc.]
                    "printbibliography" if self.in_document => {
                        let options = self.parse_optional_arg().unwrap_or_default();
                        let title = options.split(',').find_map(|option| {
                            let mut parts = option.splitn(2, '=');
                            if parts.next()?.trim() != "title" {
                                return None;
                            }
                            let value = parts.next()?.trim()
                                .trim_matches(|c| c == '{' || c == '}')
                                .to_string();
                            Some(value)
                        });
                        nodes.push(LatexNode::Bibliography { file: String::new(), title });
                    }

                    // biblatex: \addbibresource{file.bib} — loads the database
                    "addbibresource" =>
                        nodes.push(LatexNode::BibResource(self.parse_braces_content())),

                    "bibliographystyle" => {
                        let name = self.parse_braces_content();
                        if let Some(style) = BibStyle::from_name(&name) {
                            nodes.push(LatexNode::BibStyleSet(style));
                        }
                    }

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
                        } else if label_name.starts_with("sec:")
                               || label_name.starts_with("sec.")
                               || label_name.starts_with("chap.")
                        {
                            self.current_section.to_string()
                        } else {
                            // Unknown prefix — use nearest enclosing counter.
                            // Prefer section over equation to avoid eq number
                            // leaking into structural refs.
                            if self.current_section > 0 {
                                self.current_section.to_string()
                            } else if self.current_equation > 0 {
                                self.current_equation.to_string()
                            } else {
                                self.current_chapter.to_string()
                            }
                        };

                        labels.insert(label_name.clone(), target_value);
                        nodes.push(LatexNode::Label(label_name));
                    }

                    "ref" | "eqref" | "autoref" | "cref" | "Cref"
                    | "vref" | "Vref" | "cpageref" | "labelcref" if self.in_document =>
                        nodes.push(LatexNode::Ref(self.parse_braces_content())),
                    "pageref" if self.in_document =>
                        nodes.push(LatexNode::PageRef(self.parse_braces_content())),
                    "nameref" if self.in_document =>
                        nodes.push(LatexNode::NameRef(self.parse_braces_content())),
                    "hyperref" if self.in_document => {
                        let label = self.parse_optional_arg().unwrap_or_default();
                        let raw   = self.parse_braces_content();
                        let text  = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::HyperRef { label, text });
                    }

                    // --------------------------------------------------------
                    // Counter display  \arabic{c}  \roman{c} …
                    // --------------------------------------------------------
                    "arabic" | "roman" | "Roman" | "alph" | "Alph" | "fnsymbol"
                    if self.in_document => {
                        let counter = self.parse_braces_content();
                        nodes.push(LatexNode::CounterValue {
                            style: command.clone(), counter
                        });
                    }

                    // \listoffigures / \listoftables — placeholder (no actual list built)
                    "listoffigures" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<p class=\"list-placeholder\">[List of Figures]</p>".to_string()
                        )),
                    "listoftables" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<p class=\"list-placeholder\">[List of Tables]</p>".to_string()
                        )),

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
                        let explicit = self.parse_optional_arg()
                            .and_then(|s| s.trim().parse::<usize>().ok());
                        nodes.push(LatexNode::FootnoteMark(explicit));
                    }

                    "footnotetext" if self.in_document => {
                        let explicit = self.parse_optional_arg()
                            .and_then(|s| s.trim().parse::<usize>().ok());
                        let raw = self.parse_braces_content();
                        let content = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::FootnoteText { num: explicit, content });
                    }

                    // --------------------------------------------------------
                    // Images
                    // --------------------------------------------------------
                    "includegraphics" if self.in_document => {
                        self.parse_optional_arg(); // [width=…]
                        nodes.push(LatexNode::Image(self.parse_braces_content()));
                    }
                    "caption" if self.in_document => {
                        self.parse_optional_arg(); // [short title] for \listoffigures
                        let raw = self.parse_braces_content();
                        let content = Parser::new(&raw).parse(true, labels);
                        if starred {
                            nodes.push(LatexNode::CaptionStar(content));
                        } else {
                            nodes.push(LatexNode::Caption(content));
                        }
                    }

                    // --------------------------------------------------------
                    // \addcontentsline{toc}{level}{title}
                    // --------------------------------------------------------
                    "addcontentsline" if self.in_document => {
                        let list  = self.parse_braces_content(); // "toc", "lof", "lot"
                        let level = self.parse_braces_content(); // "section", "chapter", …
                        let title = self.parse_braces_content();
                        if list == "toc" {
                            nodes.push(LatexNode::AddContentsLine(level, title));
                        }
                    }

                    // --------------------------------------------------------
                    // titlesec: \titleformat — consume all args, render nothing
                    // --------------------------------------------------------
                    "titleformat" => {
                        self.parse_braces_content(); // {command}
                        self.parse_optional_arg();   // [shape]
                        self.parse_braces_content(); // {format}
                        self.parse_braces_content(); // {label}
                        self.parse_braces_content(); // {sep}
                        self.parse_braces_content(); // {before-code}
                        self.parse_optional_arg();   // [after-code]
                    }

                    "titlespacing" | "titlespacing*" => {
                        self.parse_braces_content(); // {command}
                        self.parse_braces_content(); // {left}
                        self.parse_braces_content(); // {before-sep}
                        self.parse_braces_content(); // {after-sep}
                        self.parse_optional_arg();   // [right]
                    }

                    // --------------------------------------------------------
                    // fancyhdr
                    // --------------------------------------------------------

                    // \fancyhf[pos]{content}  — sets header AND footer at pos;
                    // \fancyhf{}              — clears everything
                    "fancyhf" => {
                        let pos = self.parse_optional_arg().unwrap_or_default();
                        let content = self.parse_braces_content();
                        if content.trim().is_empty() && pos.trim().is_empty() {
                            nodes.push(LatexNode::FancyClear);
                        } else {
                            let inner = Parser::new(&content).parse(true, labels);
                            let p = Self::normalize_fancy_pos(&pos);
                            nodes.push(LatexNode::FancyHeader { pos: p.clone(), nodes: inner.clone() });
                            nodes.push(LatexNode::FancyFooter { pos: p,         nodes: inner });
                        }
                    }

                    "fancyhead" => {
                        let pos     = self.parse_optional_arg().unwrap_or_else(|| "C".to_string());
                        let content = self.parse_braces_content();
                        let inner   = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::FancyHeader {
                            pos:   Self::normalize_fancy_pos(&pos),
                            nodes: inner,
                        });
                    }

                    "fancyfoot" => {
                        let pos     = self.parse_optional_arg().unwrap_or_else(|| "C".to_string());
                        let content = self.parse_braces_content();
                        let inner   = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::FancyFooter {
                            pos:   Self::normalize_fancy_pos(&pos),
                            nodes: inner,
                        });
                    }

                    // Shorthand slot commands
                    "lhead" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyHeader {
                            pos: "L".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }
                    "chead" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyHeader {
                            pos: "C".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }
                    "rhead" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyHeader {
                            pos: "R".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }
                    "lfoot" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyFooter {
                            pos: "L".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }
                    "cfoot" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyFooter {
                            pos: "C".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }
                    "rfoot" => {
                        self.parse_optional_arg();
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::FancyFooter {
                            pos: "R".to_string(),
                            nodes: Parser::new(&content).parse(true, labels),
                        });
                    }

                    "fancypagestyle" => {
                        self.parse_braces_content(); // {style name}
                        self.parse_braces_content(); // {definition body}
                    }

                    "renewpagestyle" | "newpagestyle" => {
                        self.parse_braces_content();
                        self.parse_braces_content();
                    }

                    "thispagestyle" => { self.parse_braces_content(); }
                    "headrulewidth" | "footrulewidth" => { self.parse_braces_content(); }

                    // \thepage — current page number
                    "thepage" if self.in_document =>
                        nodes.push(LatexNode::ThePage),

                    // --------------------------------------------------------
                    // algorithmicx / algpseudocode commands
                    // --------------------------------------------------------
                    "State" | "Statex" if self.in_document => {
                        let content = self.parse_argument();
                        nodes.push(LatexNode::Text("<li class=\"alg-state\">".to_string()));
                        nodes.extend(content);
                        nodes.push(LatexNode::Text("</li>".to_string()));
                    }

                    "If" | "ElsIf" if self.in_document => {
                        let cond = self.parse_argument();
                        let kw = if command == "If" { "if" } else { "else if" };
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-if\"><strong>{}</strong> ", kw
                        )));
                        nodes.extend(cond);
                        nodes.push(LatexNode::Text(" <strong>then</strong></li>".to_string()));
                    }

                    "Else" | "EndIf" | "EndFor" | "EndWhile"
                    | "EndProcedure" | "EndFunction" | "EndLoop"
                    if self.in_document => {
                        let kw = command.to_lowercase();
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-end\"><strong>{}</strong></li>", kw
                        )));
                    }

                    "For" | "ForAll" | "While" | "Loop" if self.in_document => {
                        let cond = self.parse_argument();
                        let kw = command.to_lowercase();
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-loop\"><strong>{}</strong> ", kw
                        )));
                        nodes.extend(cond);
                        nodes.push(LatexNode::Text(" <strong>do</strong></li>".to_string()));
                    }

                    "Procedure" | "Function" if self.in_document => {
                        let name = self.parse_braces_content();
                        let args = self.parse_braces_content();
                        let kw   = command.to_lowercase();
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-proc\"><strong>{}</strong> <em>{}</em>({})</li>",
                            kw, name, args
                        )));
                    }

                    "Return" | "Require" | "Ensure" | "Print" | "Output"
                    if self.in_document => {
                        let content = self.parse_argument();
                        let kw = command.to_lowercase();
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-state\"><strong>{}</strong> ", kw
                        )));
                        nodes.extend(content);
                        nodes.push(LatexNode::Text("</li>".to_string()));
                    }

                    "Comment" if self.in_document => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Text(format!(
                            "<li class=\"alg-comment\"><em>▷ {}</em></li>", content
                        )));
                    }

                    // --------------------------------------------------------
                    // Ignored structural commands
                    // --------------------------------------------------------
                    "hline" | "cline" | "toprule" | "midrule" | "bottomrule"
                    | "appendix" | "frontmatter"
                    | "mainmatter" | "backmatter" | "sloppy" | "frenchspacing"
                    | "nonfrenchspacing" | "protect"
                    | "makeatletter" | "makeatother"
                    | "unskip" | "ignorespaces" | "relax" | "empty" => {}

                    "input" | "include" | "includeonly" => {
                        self.parse_braces_content(); // ignore file name
                    }

                    // --------------------------------------------------------
                    // Package config — consume silently
                    // --------------------------------------------------------
                    "lstset" | "lstdefinestyle" | "tcbset" | "tcbuselibrary"
                    | "pgfplotsset" | "usetikzlibrary" => {
                        self.parse_braces_content();
                    }

                    // \tikzset{name/.style={...}, ...} — named TikZ styles,
                    // stored in `labels` so every tikzpicture can expand them
                    "tikzset" => {
                        let content = self.parse_braces_content();
                        for (key, value) in Self::key_value_list(&content) {
                            if let Some(name) = key.strip_suffix("/.style") {
                                labels.insert(format!("tikzstyle@{}", name.trim()), value);
                            }
                        }
                    }

                    // \tikzstyle{name}=[options] (deprecated but common)
                    "tikzstyle" => {
                        let name = self.parse_braces_content();
                        self.skip_whitespace();
                        if self.peek() == Some('=') { self.next_char(); }
                        if let Some(options) = self.parse_optional_arg() {
                            labels.insert(format!("tikzstyle@{}", name.trim()), options);
                        }
                    }

                    "lstinputlisting" => {
                        self.parse_optional_arg();
                        self.parse_braces_content(); // filename
                    }

                    "fcolorbox" if self.in_document => {
                        let frame_color = self.parse_braces_content();
                        let bg_color    = self.parse_braces_content();
                        let content     = self.parse_braces_content();
                        let inner = Parser::new(&content).parse(true, labels);
                        nodes.push(LatexNode::Text(format!(
                            "<span style=\"border: 2px solid {}; background-color: {}; padding: 2px 6px;\">",
                            frame_color, bg_color
                        )));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    "shadowbox" | "doublebox" | "ovalbox" if self.in_document => {
                        let content = self.parse_braces_content();
                        let inner = Parser::new(&content).parse(true, labels);
                        let style = match command.as_str() {
                            "doublebox" => "border: 3px double currentColor; padding: 2px 6px;",
                            "ovalbox"   => "border: 1px solid currentColor; border-radius: 8px; padding: 2px 6px;",
                            _           => "border: 1px solid currentColor; box-shadow: 2px 2px 4px rgba(0,0,0,0.4); padding: 2px 6px;",
                        };
                        nodes.push(LatexNode::Text(format!("<span style=\"{}\">", style)));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
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
                    // Horizontal / vertical fill
                    // --------------------------------------------------------
                    "hfill" | "hfil" if self.in_document =>
                        nodes.push(LatexNode::HSpace("auto".to_string())),

                    "dotfill" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<span class=\"latex-dotfill\"></span>".to_string()
                        )),

                    "fill" if self.in_document =>
                        nodes.push(LatexNode::HSpace("auto".to_string())),

                    // \stretch{n} — proportional fill; render as flexible spacer
                    "stretch" if self.in_document => {
                        let _factor = self.parse_braces_content();
                        nodes.push(LatexNode::HSpace("auto".to_string()));
                    }

                    "vfill" | "vfil" if self.in_document =>
                        nodes.push(LatexNode::VSpace("auto".to_string())),

                    // --------------------------------------------------------
                    // TeX primitive spacing: \kern, \mkern
                    // --------------------------------------------------------
                    "kern" | "hskip" if self.in_document => {
                        // read a dimension token (e.g. 1em, 5pt, -2mm)
                        self.skip_whitespace();
                        let dim = self.read_dimension();
                        let css = Self::conv_width(&dim);
                        nodes.push(LatexNode::HSpace(css));
                    }
                    "mkern" | "mskip" if self.in_document => {
                        // math units (mu) — consume and map to a small space
                        self.skip_whitespace();
                        let _mu = self.read_dimension();
                        nodes.push(LatexNode::HSpace("0.18em".to_string()));
                    }
                    "vskip" if self.in_document => {
                        self.skip_whitespace();
                        let dim = self.read_dimension();
                        let css = Self::conv_width(&dim);
                        nodes.push(LatexNode::VSpace(css));
                    }

                    // --------------------------------------------------------
                    // TeX box primitives: \hbox, \vbox, \vtop, \vcenter
                    // --------------------------------------------------------
                    "hbox" | "vbox" | "vtop" | "vcenter" if self.in_document => {
                        // optional [height spec], then {content}
                        self.parse_optional_arg();
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.extend(inner);
                    }

                    // --------------------------------------------------------
                    // Line-break / hyphenation hints (no visual output)
                    // --------------------------------------------------------
                    "penalty" | "widowpenalty" | "clubpenalty"
                    | "interlinepenalty" if self.in_document => {
                        // consume optional number
                        self.skip_whitespace();
                        self.read_dimension(); // reads the number
                    }
                    "allowbreak" if self.in_document => {}
                    "nobreak"    if self.in_document => {}
                    "discretionary" if self.in_document => {
                        self.parse_braces_content(); // pre-break
                        self.parse_braces_content(); // post-break
                        let raw = self.parse_braces_content(); // no-break
                        nodes.extend(Parser::new(&raw).parse(true, labels));
                    }
                    "slash" if self.in_document =>
                        nodes.push(LatexNode::Text("/".to_string())),

                    // --------------------------------------------------------
                    // \not — negate next symbol
                    // --------------------------------------------------------
                    "not" if self.in_document => {
                        self.skip_whitespace();
                        // read the next command or char and emit negated form
                        if self.peek() == Some('\\') {
                            self.lexer.pos += 1;
                            let sym = self.read_command_word();
                            let negated = match sym.as_str() {
                                "in"        => "∉",
                                "ni"        => "∌",
                                "subset"    => "⊄",
                                "supset"    => "⊅",
                                "subseteq"  => "⊄",
                                "supseteq"  => "⊅",
                                "sim"       => "≁",
                                "approx"    => "≉",
                                "equiv"     => "≢",
                                "prec"      => "⊀",
                                "succ"      => "⊁",
                                "preceq"    => "⋠",
                                "succeq"    => "⋡",
                                "vdash"     => "⊬",
                                "models"    => "⊭",
                                "parallel"  => "∦",
                                "perp"      => "⊬",
                                "leq"       => "≰",
                                "geq"       => "≱",
                                _           => "≠",
                            };
                            nodes.push(LatexNode::Text(negated.to_string()));
                        } else if let Some(c) = self.next_char() {
                            let negated = match c {
                                '=' => "≠",
                                '<' => "≮",
                                '>' => "≯",
                                _   => { nodes.push(LatexNode::Text(c.to_string())); continue; }
                            };
                            nodes.push(LatexNode::Text(negated.to_string()));
                        }
                    }

                    // --------------------------------------------------------
                    // \pmod{m} / \bmod outside math
                    // --------------------------------------------------------
                    "pmod" if self.in_document => {
                        let m = self.parse_braces_content();
                        nodes.push(LatexNode::Text(format!(" (mod {})", m)));
                    }
                    "bmod" if self.in_document =>
                        nodes.push(LatexNode::Text(" mod ".to_string())),

                    // --------------------------------------------------------
                    // Line spacing
                    // --------------------------------------------------------
                    "linespread" => {
                        let raw: f64 = self.parse_braces_content()
                            .trim().parse().unwrap_or(1.0);
                        if self.in_document {
                            nodes.push(LatexNode::LineSpread(raw * 1.2));
                        }
                    }
                    "onehalfspacing" if self.in_document =>
                        nodes.push(LatexNode::LineSpread(1.5)),
                    "doublespacing" if self.in_document =>
                        nodes.push(LatexNode::LineSpread(2.0)),
                    "singlespacing" if self.in_document =>
                        nodes.push(LatexNode::LineSpread(1.2)),

                    // --------------------------------------------------------
                    // \qed / \qedhere — proof end marker ∎
                    // --------------------------------------------------------
                    "qed" | "qedhere" if self.in_document =>
                        nodes.push(LatexNode::Text(
                            "<span class=\"qed\">∎</span>".to_string()
                        )),

                    // --------------------------------------------------------
                    // \smash{content} — render content with zero height
                    // --------------------------------------------------------
                    "smash" if self.in_document => {
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"display:inline-block;height:0;overflow:visible;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \llap{content} / \rlap{content} — overlap boxes
                    // --------------------------------------------------------
                    "llap" if self.in_document => {
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"display:inline-block;width:0;overflow:visible;direction:rtl;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }
                    "rlap" if self.in_document => {
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<span style=\"display:inline-block;width:0;overflow:visible;\">".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</span>".to_string()));
                    }

                    // --------------------------------------------------------
                    // \keywords{...} — abstract keywords
                    // --------------------------------------------------------
                    "keywords" if self.in_document => {
                        let raw   = self.parse_braces_content();
                        let inner = Parser::new(&raw).parse(true, labels);
                        nodes.push(LatexNode::Text(
                            "<div class=\"latex-keywords\"><strong>Keywords:</strong> ".to_string()
                        ));
                        nodes.extend(inner);
                        nodes.push(LatexNode::Text("</div>".to_string()));
                    }

                    // --------------------------------------------------------
                    // Phantom boxes
                    // --------------------------------------------------------
                    "phantom" if self.in_document => {
                        let raw = self.parse_braces_content();
                        nodes.push(LatexNode::Phantom(
                            Parser::new(&raw).parse(true, labels)
                        ));
                    }

                    "hphantom" if self.in_document => {
                        let raw = self.parse_braces_content();
                        nodes.push(LatexNode::HPhantom(
                            Parser::new(&raw).parse(true, labels)
                        ));
                    }

                    "vphantom" if self.in_document => {
                        let raw = self.parse_braces_content();
                        nodes.push(LatexNode::VPhantom(
                            Parser::new(&raw).parse(true, labels)
                        ));
                    }

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
                    // \multicolumn / \multirow outside tabular — render content
                    // --------------------------------------------------------
                    "multicolumn" if self.in_document => {
                        self.parse_braces_content(); // {N}
                        self.parse_braces_content(); // {spec}
                        let raw = self.parse_braces_content(); // {content}
                        nodes.extend(Parser::new(&raw).parse(true, labels));
                    }

                    "multirow" if self.in_document => {
                        self.parse_braces_content(); // {N}
                        self.parse_optional_arg();   // [vpos]
                        self.parse_braces_content(); // {width}
                        self.parse_optional_arg();   // [fixup]
                        let raw = self.parse_braces_content(); // {content}
                        nodes.extend(Parser::new(&raw).parse(true, labels));
                    }

                    // --------------------------------------------------------
                    // User-defined macro expansion  (\newcommand / \def / \let)
                    // --------------------------------------------------------
                    _ if self.macros.contains_key(command.as_str()) => {
                        let def = self.macros[command.as_str()].clone();

                        // \def\x{\x} must not hang or blow the stack — each
                        // nesting level is a full parse() frame, so the cap
                        // stays low (real documents rarely nest beyond ~5)
                        if self.expansion_depth < 16 {
                            let mut args: Vec<String> = Vec::new();
                            if let Some(default) = &def.default {
                                args.push(self.parse_optional_arg()
                                    .unwrap_or_else(|| default.clone()));
                            }
                            while args.len() < def.params {
                                args.push(self.lexer.macro_argument());
                            }

                            // Re-parse the spliced body
                            let expanded = def.expand(&args);
                            let mut sub = Parser::new(&expanded);
                            sub.in_document     = self.in_document;
                            sub.macros          = self.macros.clone();
                            sub.expansion_depth = self.expansion_depth + 1;
                            sub.current_chapter   = self.current_chapter;
                            sub.current_section   = self.current_section;
                            sub.current_subsection= self.current_subsection;
                            sub.current_equation  = self.current_equation;
                            sub.current_table     = self.current_table;
                            let expanded_nodes = sub.parse(true, labels);

                            // definitions made inside the expansion persist
                            self.macros = sub.macros;
                            nodes.extend(expanded_nodes);
                        }
                    }

                    // --------------------------------------------------------
                    // Package modules (siunitx, tcolorbox, ...) — consulted
                    // after user macros so \renewcommand takes precedence
                    // --------------------------------------------------------
                    _ if packages::is_package_command(command.as_str()) && self.in_document => {
                        let handled = packages::command(command.as_str(), starred, self, labels);
                        nodes.extend(handled);
                    }

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
            // Stray & in body text — alignment contexts never reach here
            // (math goes raw to MathJax, tabular consumes & itself), so the
            // tolerant rendering is a literal ampersand
            // ----------------------------------------------------------------
            if current == '&' && self.in_document {
                self.lexer.pos += 1;
                nodes.push(LatexNode::Text("&amp;".to_string()));
                continue;
            }

            // ----------------------------------------------------------------
            // Superscript  ^
            // ----------------------------------------------------------------
            if current == '^' && self.in_document {
                self.lexer.pos += 1;
                nodes.push(LatexNode::Superscript(self.parse_argument()));
                continue;
            }

            // ----------------------------------------------------------------
            // Subscript  _
            // ----------------------------------------------------------------
            if current == '_' && self.in_document {
                self.lexer.pos += 1;
                nodes.push(LatexNode::Subscript(self.parse_argument()));
                continue;
            }

            // ----------------------------------------------------------------
            // Inline math  $...$  or  $$...$$
            // ----------------------------------------------------------------
            if current == '$' && self.in_document {
                nodes.push(self.consume_dollar_math());
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
                self.lexer.pos += 1;
                continue;
            }

            // ----------------------------------------------------------------
            // LaTeX typographic quotes:
            //   ``  →  " (U+201C)     ''  →  " (U+201D)
            //   `   →  ' (U+2018)     '   in text → ' (U+2019)
            // These are just source characters, not commands.
            // ----------------------------------------------------------------
            if current == '`' && self.in_document {
                self.lexer.pos += 1;
                if self.peek() == Some('`') {
                    self.lexer.pos += 1;
                    nodes.push(LatexNode::Text("\u{201C}".to_string())); // "
                } else {
                    nodes.push(LatexNode::Text("\u{2018}".to_string())); // '
                }
                continue;
            }

            if current == '\'' && self.in_document {
                self.lexer.pos += 1;
                if self.peek() == Some('\'') {
                    self.lexer.pos += 1;
                    nodes.push(LatexNode::Text("\u{201D}".to_string())); // "
                } else {
                    nodes.push(LatexNode::Text("\u{2019}".to_string())); // '
                }
                continue;
            }

            // Em-dash  ---  and en-dash  --  (hyphens in text)
            if current == '-' && self.in_document {
                self.lexer.pos += 1;
                if self.peek() == Some('-') {
                    self.lexer.pos += 1;
                    if self.peek() == Some('-') {
                        self.lexer.pos += 1;
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

                self.lexer.pos += 1;
            } else if self.in_document && !text.trim().is_empty() {
                nodes.push(LatexNode::Text(text));
            } else if self.in_document {
                nodes.push(LatexNode::Text(" ".to_string()));
            }
        }

        nodes
    }

    fn parse_environment(&mut self, env: &str, opt: Option<String>, labels: &mut HashMap<String, String>) -> Vec<LatexNode> {
        let mut nodes: Vec<LatexNode> = Vec::new();

        // Environments declared with \newtheorem take precedence over
        // built-ins. The registry lives in `labels` (threaded through every
        // sub-parser); numbering happens at render time.
        if let Some(def) = labels.get(&format!("thm@{}", env)).cloned() {
            let mut fields = def.splitn(4, '|');
            let title   = fields.next().unwrap_or("").to_string();
            let counter = fields.next().unwrap_or("").to_string();
            let parent  = fields.next().unwrap_or("").to_string();
            let style   = fields.next().unwrap_or("plain").to_string();

            let raw  = self.read_until_end(env);
            let body = Parser::new(raw.trim()).parse(true, labels);

            nodes.push(LatexNode::Theorem { title, counter, parent, style, note: opt, body });
            return nodes;
        }

        // Package modules own their environments (tcolorbox, ...)
        if self.in_document && packages::is_package_environment(env) {
            nodes.extend(packages::environment(env, opt, self, labels));
            return nodes;
        }

        if env == "document" {
            self.in_document = true;

        } else if env == "abstract" && self.in_document {
            let raw = self.read_until_end("abstract");
            nodes.push(LatexNode::Abstract(
                Parser::new(raw.trim()).parse(true, labels)
            ));

        } else if (env == "IEEEkeywords" || env == "keywords") && self.in_document {
            let raw = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text(
                "<div class=\"latex-keywords\"><strong>Keywords:</strong> ".to_string()
            ));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "tabbing" && self.in_document {
            let raw = self.read_until_end("tabbing");
            nodes.push(LatexNode::Text(Self::render_tabbing(&raw)));

        } else if env == "verse" && self.in_document {
            let raw = self.read_until_end("verse");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div class=\"latex-verse\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "titlepage" && self.in_document {
            let raw = self.read_until_end("titlepage");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div class=\"latex-titlepage\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if (env == "landscape" || env == "pdflscape") && self.in_document {
            let raw = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div class=\"latex-landscape\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "spacing" && self.in_document {
            let factor: f64 = self.parse_braces_content().trim().parse().unwrap_or(1.2);
            let raw   = self.read_until_end("spacing");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text(format!("<div style=\"line-height:{}\">", factor)));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if matches!(env, "sloppypar" | "comment") && self.in_document {
            let raw = self.read_until_end(env);
            if env != "comment" {
                nodes.extend(Parser::new(raw.trim()).parse(true, labels));
            }

        } else if env == "samepage" && self.in_document {
            // keep the whole block on one page
            let raw = self.read_until_end("samepage");
            nodes.push(LatexNode::Text(
                "<div style=\"break-inside: avoid; page-break-inside: avoid;\">".to_string()
            ));
            nodes.extend(Parser::new(raw.trim()).parse(true, labels));
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "thebibliography" && self.in_document {
            self.parse_braces_content(); // {widest-label} — ignored
            let raw = self.read_until_end("thebibliography");
            nodes.push(LatexNode::TheBibliography(
                Self::parse_thebibliography(&raw, labels)
            ));

        } else if matches!(env, "lstlisting" | "verbatim" | "Verbatim") && self.in_document {
            self.parse_optional_arg();
            let raw = self.read_until_end(env);
            nodes.push(LatexNode::CodeBlock(raw.trim_matches('\n').to_string()));

        } else if env == "minted" && self.in_document {
            self.parse_braces_content(); // language arg
            let raw = self.read_until_end("minted");
            nodes.push(LatexNode::CodeBlock(raw.trim_matches('\n').to_string()));

        } else if env == "itemize" && self.in_document {
            let block = self.read_until_end("itemize");
            nodes.push(LatexNode::Itemize(Self::split_items(&block, labels)));

        } else if env == "enumerate" && self.in_document {
            let opt   = self.parse_optional_arg();
            let block = self.read_until_end("enumerate");
            let items = Self::split_items(&block, labels);
            if let Some(opt_str) = opt {
                let style = Self::enumitem_label_style(&opt_str);
                if !style.is_empty() {
                    nodes.push(LatexNode::EnumerateLabeled { style, items });
                } else {
                    nodes.push(LatexNode::Enumerate(items));
                }
            } else {
                nodes.push(LatexNode::Enumerate(items));
            }

        } else if env == "description" && self.in_document {
            let block = self.read_until_end("description");
            nodes.push(LatexNode::Description(Self::split_description_items(&block, labels)));

        } else if env == "mermaid" && self.in_document {
            let raw = self.read_until_end("mermaid");
            nodes.push(LatexNode::Mermaid(raw));

        } else if (env == "tabular" || env == "tabular*") && self.in_document {
            if env == "tabular*" { self.parse_braces_content(); } // overall width
            let colspec     = self.parse_braces_content();
            let table_block = self.read_until_end(env);
            nodes.push(LatexNode::Table(Self::parse_tabular(&table_block, &colspec, labels)));

        } else if (env == "tabularx" || env == "tabulary") && self.in_document {
            self.parse_braces_content(); // overall width
            let colspec     = self.parse_braces_content();
            let table_block = self.read_until_end(env);
            nodes.push(LatexNode::Table(Self::parse_tabular(&table_block, &colspec, labels)));

        } else if (env == "table" || env == "table*") && self.in_document {
            let raw = self.read_until_end(env);
            self.current_table += 1;
            let registered = Self::extract_and_register_labels(
                &raw, &self.current_table.to_string(), "tab:", labels,
            );
            let mut sub = Parser::new(raw.trim());
            sub.current_table      = self.current_table;
            sub.current_section    = self.current_section;
            sub.current_chapter    = self.current_chapter;
            sub.current_subsection = self.current_subsection;
            sub.current_equation   = self.current_equation;

            // anchors go at the TOP of the float (hypcap-style) but INSIDE
            // its unbreakable wrapper, so they travel with it when the page
            // fragmenter pushes the float — links land with the table visible
            let mut children = sub.parse(true, labels);
            for name in registered.into_iter().rev() {
                children.insert(0, LatexNode::Label(name));
            }

            // table* spans every column in two-column layouts
            if env == "table*" {
                nodes.push(LatexNode::Text("<div style=\"column-span: all;\">".to_string()));
                nodes.push(LatexNode::TableFloat(children));
                nodes.push(LatexNode::Text("</div>".to_string()));
            } else {
                nodes.push(LatexNode::TableFloat(children));
            }

        } else if (env == "figure" || env == "figure*") && self.in_document {
            let raw = self.read_until_end(env);
            let registered = Self::extract_and_register_labels(
                &raw, &self.current_section.to_string(), "fig:", labels,
            );
            let mut sub = Parser::new(raw.trim());
            sub.current_section    = self.current_section;
            sub.current_chapter    = self.current_chapter;
            sub.current_equation   = self.current_equation;
            sub.current_table      = self.current_table;
            sub.current_subsection = self.current_subsection;

            let mut children = sub.parse(true, labels);
            for name in registered.into_iter().rev() {
                children.insert(0, LatexNode::Label(name));
            }

            // figure* spans every column in two-column layouts
            if env == "figure*" {
                nodes.push(LatexNode::Text("<div style=\"column-span: all;\">".to_string()));
                nodes.push(LatexNode::FigureFloat(children));
                nodes.push(LatexNode::Text("</div>".to_string()));
            } else {
                nodes.push(LatexNode::FigureFloat(children));
            }

        } else if (env == "equation" || env == "equation*") && self.in_document {
            let raw = self.read_until_end(env);
            self.current_equation += 1;
            // the math body goes to MathJax, so anchors for its labels must
            // be emitted here for \pageref to find them in the DOM
            for name in Self::extract_and_register_labels(&raw, &self.current_equation.to_string(), "", labels) {
                nodes.push(LatexNode::Label(name));
            }
            nodes.push(LatexNode::EquationBlock(
                vec![LatexNode::RawMathDisplay(raw.trim().to_string())]
            ));

        } else if matches!(env,
            "array" | "cases" | "dcases" | "rcases" |
            "split" | "aligned" | "alignedat" | "gathered"
        ) && self.in_document {
            if env == "array" || env == "alignedat" { self.parse_braces_content(); }
            let raw   = self.read_until_end(env);
            let latex = format!("\\begin{{{}}}{}\\end{{{}}}", env, raw.trim(), env);
            nodes.push(LatexNode::RawMathDisplay(latex));

        } else if env == "subequations" && self.in_document {
            let raw = self.read_until_end("subequations");
            nodes.extend(Parser::new(raw.trim()).parse(true, labels));

        } else if matches!(env,
            "align"  | "align*" | "eqnarray" | "eqnarray*" |
            "multline" | "multline*" | "gather" | "gather*" |
            "flalign" | "flalign*"
        ) && self.in_document {
            let raw = self.read_until_end(env);
            self.current_equation += 1;
            for name in Self::extract_and_register_labels(&raw, &self.current_equation.to_string(), "", labels) {
                nodes.push(LatexNode::Label(name));
            }
            let latex = format!("\\begin{{{}}}{}\\end{{{}}}", env, raw.trim(), env);
            nodes.push(LatexNode::AlignBlock(vec![LatexNode::RawMathDisplay(latex)]));

        } else if matches!(env,
            "pmatrix" | "bmatrix" | "Bmatrix" | "vmatrix" | "Vmatrix" | "matrix" | "smallmatrix"
        ) && self.in_document {
            // delegate to MathJax — contextual cell alignment belongs to the
            // math engine, not to hand-rolled HTML
            let raw = self.read_until_end(env);
            nodes.push(LatexNode::RawMathInline(
                format!("\\begin{{{}}}{}\\end{{{}}}", env, raw.trim(), env)
            ));

        } else if env == "center" && self.in_document {
            let raw   = self.read_until_end("center");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div style=\"text-align:center;\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "quote" || env == "quotation" {
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<blockquote class=\"latex-quote\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</blockquote>".to_string()));

        } else if matches!(env,
            "theorem" | "lemma" | "corollary" | "proposition" | "proof" |
            "definition" | "remark" | "example" | "conjecture" | "claim" |
            "exercise" | "solution" | "question" | "answer" | "notation" |
            "observation" | "assumption" | "fact" | "problem" | "note"
        ) {
            let raw  = self.read_until_end(env);
            let body = Parser::new(raw.trim()).parse(true, labels);

            let style = match env {
                "proof" => "proof",
                "definition" | "example" | "exercise" | "problem"
                | "question" | "solution" | "answer" => "definition",
                "remark" | "note" | "notation" | "observation" | "claim" => "remark",
                _ => "plain",
            };

            nodes.push(LatexNode::Theorem {
                title:   Self::capitalise(env),
                counter: if env == "proof" { String::new() } else { env.to_string() },
                parent:  String::new(),
                style:   style.to_string(),
                note:    opt,
                body,
            });

        } else if env == "minipage" && self.in_document {
            self.parse_optional_arg();
            let width = Self::conv_width(&self.parse_braces_content());
            let raw   = self.read_until_end("minipage");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text(format!(
                "<div class=\"latex-minipage\" style=\"width: {};\">", width
            )));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if (env == "multicols" || env == "multicols*") && self.in_document {
            let count: u32 = self.parse_braces_content().trim().parse().unwrap_or(2);

            // \begin{multicols}{2}[preface][skip] — preface spans the columns
            let preface = self.parse_optional_arg()
                .map(|raw| Parser::new(raw.trim()).parse(true, labels))
                .unwrap_or_default();
            self.parse_optional_arg(); // optional vertical skip — ignored

            let raw  = self.read_until_end(env);
            let body = Parser::new(raw.trim()).parse(true, labels);

            nodes.push(LatexNode::MultiCols {
                count,
                preface,
                body,
                balanced: env == "multicols",
            });

        } else if matches!(env, "framed" | "shaded" | "shaded*" | "oframed" | "mdframed")
               && self.in_document
        {
            self.parse_optional_arg();
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            let cls   = if env.starts_with("shaded") { "latex-shaded" } else { "latex-framed" };
            nodes.push(LatexNode::Text(format!("<div class=\"{}\">", cls)));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if (env == "wrapfigure" || env == "wraptable") && self.in_document {
            self.parse_optional_arg();
            let pos_raw   = self.parse_braces_content();
            let width_raw = self.parse_braces_content();
            let width     = Self::conv_width(&width_raw);
            let float_dir = match pos_raw.trim() { "l" | "i" | "L" | "I" => "left", _ => "right" };
            let margin    = if float_dir == "left" { "0 1.5em 1em 0" } else { "0 0 1em 1.5em" };
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text(format!(
                "<div class=\"latex-wrapfigure\" style=\"float:{float}; width:{width}; margin:{margin};\">",
                float = float_dir, width = width, margin = margin,
            )));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("<div style=\"clear:both;\"></div></div>".to_string()));

        } else if (env == "subfigure" || env == "subfloat") && self.in_document {
            self.parse_optional_arg();
            let width = Self::conv_width(&self.parse_braces_content());
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text(format!(
                "<figure class=\"latex-subfigure\" style=\"width:{width};\">", width = width,
            )));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</figure>".to_string()));

        } else if matches!(env, "longtable" | "longtabu" | "xltabular") && self.in_document {
            self.parse_optional_arg();
            if env == "xltabular" { self.parse_braces_content(); }
            let colspec = self.parse_braces_content();
            let raw     = self.read_until_end(env);
            nodes.push(LatexNode::Table(Self::parse_tabular(&raw, &colspec, labels)));

        } else if env == "flushright" && self.in_document {
            let raw   = self.read_until_end("flushright");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div style=\"text-align:right;\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "flushleft" && self.in_document {
            let raw   = self.read_until_end("flushleft");
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div style=\"text-align:left;\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if matches!(env, "tikzpicture" | "pgfpicture" | "circuitikz" | "forest" | "tikzcd" | "scope")
               && self.in_document
        {
            let raw = self.read_until_end(env);

            // pgfplots: every axis environment becomes an inline SVG chart;
            // plain pictures go through the TikZ drawing renderer
            let axes = Pgfplots::parse(&raw);
            if !axes.is_empty() {
                for axis in axes {
                    nodes.push(LatexNode::PgfPlot(axis));
                }
            } else {
                // named styles from \tikzstyle/\tikzset across the document
                let styles: HashMap<String, String> = labels.iter()
                    .filter_map(|(key, value)| {
                        let name = key.strip_prefix("tikzstyle@")?;
                        Some((name.to_string(), value.clone()))
                    })
                    .collect();

                if let Some(picture) = Tikz::parse(&raw, opt.as_deref(), &styles) {
                    nodes.push(LatexNode::Tikz(picture));
                } else {
                    nodes.push(LatexNode::Text(
                        format!("<div class=\"latex-tikz-placeholder\">[{} diagram]</div>", env)
                    ));
                }
            }

        } else if matches!(env, "algorithm" | "algorithm2e" | "algorithm*") && self.in_document {
            self.parse_optional_arg();
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div class=\"latex-algorithm\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if matches!(env, "algorithmic" | "algorithmicx" | "algpseudocode") && self.in_document {
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<ol class=\"latex-algorithmic\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</ol>".to_string()));

        } else if matches!(env, "appendices" | "subappendices") && self.in_document {
            let raw   = self.read_until_end(env);
            let inner = Parser::new(raw.trim()).parse(true, labels);
            nodes.push(LatexNode::Text("<div class=\"latex-appendices\">".to_string()));
            nodes.extend(inner);
            nodes.push(LatexNode::Text("</div>".to_string()));

        } else if env == "filecontents" || env == "filecontents*" {
            self.parse_braces_content();
            self.read_until_end(env);

        } else if matches!(env, "adjustbox" | "varwidth") && self.in_document {
            self.parse_optional_arg();
            if env == "varwidth" { self.parse_braces_content(); }
            let raw = self.read_until_end(env);
            nodes.extend(Parser::new(raw.trim()).parse(true, labels));

        } else if self.in_document && self.macros.contains_key(&format!("env@begin@{}", env)) {
            let begin_def = self.macros[&format!("env@begin@{}", env)].clone();
            let end_code = self.macros.get(&format!("env@end@{}", env))
                .map(|def| def.expand(&[]))
                .unwrap_or_default();

            // \begin{env}[opt]{a}{b} — the begin handler already consumed
            // the [opt] group; remaining mandatory arguments follow
            let mut args: Vec<String> = Vec::new();
            if let Some(default) = &begin_def.default {
                args.push(opt.clone().unwrap_or_else(|| default.clone()));
            }
            while args.len() < begin_def.params {
                args.push(self.lexer.macro_argument());
            }

            let body = self.read_until_end(env);
            let full = format!("{}{}{}", begin_def.expand(&args), body, end_code);

            let mut sub = Parser::new(full.trim());
            sub.in_document = true;
            sub.macros = self.macros.clone();
            nodes.extend(sub.parse(true, labels));

        } else {
            self.read_until_end(env);
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

    // -----------------------------------------------------------------------
    // Inline bibliography parser
    // -----------------------------------------------------------------------

    /// Parse the body of \begin{thebibliography}...\end{thebibliography}.
    /// Returns Vec<(key, content_nodes)>.
    fn parse_thebibliography(
        body: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<(String, Vec<LatexNode>)> {
        let mut items: Vec<(String, Vec<LatexNode>)> = Vec::new();
        let mut rest = body;

        while let Some(pos) = rest.find("\\bibitem") {
            rest = &rest[pos + 8..]; // skip \bibitem

            // optional label in [ ] (ignored — used for display label in LaTeX)
            if rest.starts_with('[') {
                if let Some(end) = rest.find(']') {
                    rest = &rest[end + 1..];
                }
            }

            // {key}
            let key = if rest.starts_with('{') {
                if let Some(end) = rest.find('}') {
                    let k = rest[1..end].trim().to_string();
                    rest = &rest[end + 1..];
                    k
                } else { continue; }
            } else { continue; };

            // Content runs until the next \bibitem or end of body
            let content_end = rest.find("\\bibitem").unwrap_or(rest.len());
            let content_raw = rest[..content_end].trim();
            rest = &rest[content_end..];

            let content = Parser::new(content_raw).parse(true, labels);
            items.push((key, content));
        }

        items
    }

    // -----------------------------------------------------------------------
    // Brace / colspec / cell-meta helpers
    // -----------------------------------------------------------------------

    /// Extract the content of the first `{...}` from `s`, return (inner, rest).
    fn take_brace(s: &str) -> Option<(String, String)> {
        let s = s.trim_start();
        if !s.starts_with('{') { return None; }
        let inner = &s[1..];
        let mut depth = 1usize;
        let mut end   = 0usize;
        for (i, c) in inner.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 { end = i; break; }
                }
                _ => {}
            }
        }
        if depth != 0 { return None; }
        Some((inner[..end].to_string(), inner[end + 1..].to_string()))
    }

    /// Parse a LaTeX column-spec string into `Vec<(text-align, width)>`.
    fn parse_colspec(spec: &str) -> Vec<(String, Option<String>)> {
        let mut cols: Vec<(String, Option<String>)> = Vec::new();
        let bytes = spec.as_bytes();
        let mut i   = 0usize;

        while i < bytes.len() {
            match bytes[i] {
                b'l' | b's' => { cols.push(("left".into(),   None)); i += 1; }
                b'c'        => { cols.push(("center".into(), None)); i += 1; }
                b'r'        => { cols.push(("right".into(),  None)); i += 1; }
                // tabularx auto-width columns
                b'X' | b'Y' | b'Z' => {
                    cols.push(("left".into(), None)); i += 1;
                }
                // fixed-width cells: p{w}  m{w}  b{w}
                b'p' | b'm' | b'b' => {
                    i += 1;
                    if i < bytes.len() && bytes[i] == b'{' {
                        if let Some((w, rest)) = Self::take_brace(&spec[i..]) {
                            cols.push(("left".into(), Some(Self::conv_width(&w))));
                            i = spec.len() - rest.len();
                            continue;
                        }
                    }
                    cols.push(("left".into(), None));
                }
                // vertical rule, spaces → skip
                b'|' | b' ' | b'\t' | b'\n' => { i += 1; }
                // decorators: @{...}  >{...}  <{...}  !{...}
                b'@' | b'>' | b'<' | b'!' => {
                    i += 1;
                    if i < bytes.len() && bytes[i] == b'{' {
                        if let Some((_, rest)) = Self::take_brace(&spec[i..]) {
                            i = spec.len() - rest.len();
                            continue;
                        }
                    }
                }
                // *{n}{spec} — repeat
                b'*' => {
                    i += 1;
                    if let Some((n_str, rest)) = Self::take_brace(&spec[i..]) {
                        let n: usize = n_str.trim().parse().unwrap_or(1);
                        let rest = rest.trim_start();
                        if let Some((sub, rest2)) = Self::take_brace(rest) {
                            let sub_cols = Self::parse_colspec(&sub);
                            for _ in 0..n { cols.extend(sub_cols.clone()); }
                            i = spec.len() - rest2.len();
                            continue;
                        }
                    }
                    i += 1;
                }
                _ => { i += 1; }
            }
        }
        cols
    }

    /// Try to extract `\multicolumn{N}{spec}{content}` from the start of `cell`.
    /// Returns `(colspan, align_css, content_str)` if it matches.
    fn extract_multicolumn(cell: &str) -> Option<(usize, String, String)> {
        let s = cell.trim();
        if !s.starts_with("\\multicolumn") { return None; }
        let rest = s["\\multicolumn".len()..].trim_start();
        let (n_str, rest)     = Self::take_brace(rest)?;
        let n: usize          = n_str.trim().parse().ok()?;
        let rest              = rest.trim_start();
        let (spec, rest)      = Self::take_brace(&rest)?;
        let rest              = rest.trim_start();
        let (content, _)      = Self::take_brace(&rest)?;
        let align             = Self::colspec_align(&spec);
        Some((n, align, content))
    }

    /// Try to extract `\multirow{N}[vpos]{width}[fixup]{content}` from `cell`.
    /// Returns `(rowspan, content_str)` if it matches.
    fn extract_multirow(cell: &str) -> Option<(usize, String)> {
        let s = cell.trim();
        if !s.starts_with("\\multirow") { return None; }
        let rest           = s["\\multirow".len()..].trim_start();
        let (n_str, rest)  = Self::take_brace(rest)?;
        let n: usize       = n_str.trim().parse().ok()?;
        // optional [vpos]
        let rest = if rest.trim_start().starts_with('[') {
            let after = rest.trim_start();
            if let Some(end) = after.find(']') { &after[end+1..] } else { &rest }
        } else { &rest };
        let rest           = rest.trim_start();
        let (_w, rest)     = Self::take_brace(rest)?;   // width arg (ignore)
        // optional fixup arg
        let rest = if rest.trim_start().starts_with('[') {
            let after = rest.trim_start();
            if let Some(end) = after.find(']') { &after[end+1..] } else { &rest }
        } else { &rest };
        let rest           = rest.trim_start();
        let (content, _)   = Self::take_brace(rest)?;
        Some((n, content))
    }

    /// Map a single-column spec char/string to a CSS text-align value.
    fn colspec_align(spec: &str) -> String {
        for c in spec.trim().chars() {
            match c {
                'l' | 'p' | 's' => return "left".to_string(),
                'c'             => return "center".to_string(),
                'r'             => return "right".to_string(),
                _ => {}
            }
        }
        String::new()
    }

    /// Strip `\cline{...}` and `\cmidrule[...]{...}` patterns from a row string.
    fn strip_partial_rules(s: &str) -> (String, bool) {
        let mut result = String::new();
        let mut has_hline = false;
        let bytes = s.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            // Check for \hline / \toprule / \midrule / \bottomrule
            for rule in &["\\hline", "\\toprule", "\\midrule", "\\bottomrule"] {
                if s[i..].starts_with(rule) {
                    has_hline = true;
                    i += rule.len();
                    // continue outer while
                }
            }
            // Check for \cline / \cmidrule
            if s[i..].starts_with("\\cline") || s[i..].starts_with("\\cmidrule") {
                let cmd_len = if s[i..].starts_with("\\cline") { 6 } else { 9 };
                i += cmd_len;
                // optional [l|r] or [trim] arg
                if i < bytes.len() && bytes[i] == b'[' {
                    if let Some(end) = s[i..].find(']') { i += end + 1; }
                }
                // required {1-3} arg
                if i < bytes.len() && bytes[i] == b'{' {
                    if let Some((_, rest)) = Self::take_brace(&s[i..]) {
                        i = s.len() - rest.len();
                    }
                }
                continue;
            }
            if i < bytes.len() {
                result.push(bytes[i] as char);
                i += 1;
            }
        }
        (result, has_hline)
    }

    // -----------------------------------------------------------------------
    // Main table parser
    // -----------------------------------------------------------------------

    fn parse_tabular(
        table_block: &str,
        colspec: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<Vec<TableCell>> {
        let col_specs = Self::parse_colspec(colspec);
        let mut rows: Vec<Vec<TableCell>> = Vec::new();
        let mut pending_hline = false;

        for row_str in table_block.split(r"\\") {
            let (clean, has_hline) = Self::strip_partial_rules(row_str);
            let clean = clean.trim().to_string();
            if clean.is_empty() {
                if has_hline { pending_hline = true; }
                continue;
            }

            let row_hline = pending_hline || has_hline;
            pending_hline = false;

            let mut cells: Vec<TableCell> = Vec::new();
            let mut col_idx = 0usize;
            let is_first_row = rows.is_empty();

            for cell_str in clean.split('&') {
                let cell_str = cell_str.trim();

                // \multicolumn{N}{spec}{content}
                if let Some((span, align, content)) = Self::extract_multicolumn(cell_str) {
                    let nodes = Parser::new(&content).parse(true, labels);
                    cells.push(TableCell {
                        content: nodes,
                        colspan: span,
                        rowspan: 1,
                        align,
                        width: None,
                        hline: false,
                    });
                    col_idx += span;
                    continue;
                }

                // \multirow may wrap other content too; extract it first
                let (rowspan, inner_str) = Self::extract_multirow(cell_str)
                    .unwrap_or((1, cell_str.to_string()));

                // Apply column spec styling
                let (align, width) = col_specs.get(col_idx)
                    .cloned()
                    .unwrap_or_default();

                let nodes = Parser::new(&inner_str).parse(true, labels);

                // First row of a table that has only header-looking cells → use th
                cells.push(TableCell {
                    content: nodes,
                    colspan: 1,
                    rowspan,
                    align,
                    width,
                    hline: false,
                });
                col_idx += 1;
            }

            if !cells.is_empty() {
                // Apply hline to ALL cells in the row (not just the first)
                if row_hline {
                    for cell in &mut cells {
                        cell.hline = true;
                    }
                }
                // Promote first row cells to <th> by wrapping in Bold if
                // the table starts with \hline (booktabs / standard tables)
                if is_first_row && row_hline {
                    for cell in &mut cells {
                        let old = std::mem::take(&mut cell.content);
                        cell.content = vec![LatexNode::Bold(old)];
                    }
                }
                rows.push(cells);
            }
        }

        rows
    }

    // -----------------------------------------------------------------------
    // Width / color helpers used by the new environments
    // -----------------------------------------------------------------------

    /// Convert a LaTeX length (possibly \fill, \stretch, calc-style) to a CSS value.
    /// Used by \setlength — produces a value suitable for a CSS custom property.
    fn length_to_css(raw: &str) -> String {
        let s = raw.trim();
        match s {
            "\\fill" | "\\hfill" | "\\vfill" | "\\hfil" | "\\vfil" => "auto".to_string(),
            _ if s.starts_with("\\stretch") => "auto".to_string(),
            _ => Self::conv_width(s),
        }
    }

    /// Convert a LaTeX width expression to a CSS value.
    fn conv_width(raw: &str) -> String {
        let s = raw.trim();

        // Fraction of \textwidth / \linewidth / \columnwidth
        for keyword in &["\\textwidth", "\\linewidth", "\\columnwidth", "\\hsize"] {
            if let Some(idx) = s.find(keyword) {
                let factor = s[..idx].trim();
                if factor.is_empty() || factor == "1" || factor == "1.0" {
                    return "100%".to_string();
                }
                if let Ok(f) = factor.parse::<f64>() {
                    return format!("{:.1}%", f * 100.0);
                }
                return "100%".to_string();
            }
        }

        // Already CSS-compatible units
        if s.ends_with('%')
            || s.ends_with("px") || s.ends_with("em") || s.ends_with("rem")
            || s.ends_with("vw") || s.ends_with("vh")
        {
            return s.to_string();
        }

        // LaTeX absolute units → keep as-is (browsers understand cm, mm, in, pt)
        if s.ends_with("cm") || s.ends_with("mm") || s.ends_with("in")
            || s.ends_with("pt") || s.ends_with("ex")
        {
            return s.to_string();
        }

        // Fallback
        if s.is_empty() { "100%".to_string() } else { s.to_string() }
    }

    /// Convert a LaTeX color expression (name or `color!pct!base`) to CSS hex.
    pub(crate) fn latex_color(raw: &str) -> String {
        let parts: Vec<&str> = raw.split('!').collect();
        let name = parts[0].trim().to_lowercase();

        let base: (u8, u8, u8) = match name.as_str() {
            "white"            => (255, 255, 255),
            "black"            => (  0,   0,   0),
            "red"              => (231,  76,  60),
            "blue"             => ( 41, 128, 185),
            "green"            => ( 39, 174,  96),
            "yellow"           => (241, 196,  15),
            "orange"           => (230, 126,  34),
            "cyan"             => ( 26, 188, 156),
            "magenta" | "purple" => (142,  68, 173),
            "gray"  | "grey"   => (149, 165, 166),
            "brown"            => (160,  82,  45),
            "teal"             => (  0, 128, 128),
            "violet"           => (148,   0, 211),
            "pink"             => (231, 103, 159),
            "lime"             => (178, 223,  69),
            "olive"            => (128, 128,   0),
            "navy"             => (  0,   0, 128),
            // Pass through if it looks like a CSS value already
            _ => return raw.to_string(),
        };

        let pct: f32 = if parts.len() >= 2 {
            parts[1].trim().parse().unwrap_or(100.0)
        } else {
            100.0
        };

        let mix: (u8, u8, u8) = if parts.len() >= 3 {
            match parts[2].trim().to_lowercase().as_str() {
                "black" => (0, 0, 0),
                _       => (255, 255, 255),
            }
        } else {
            (255, 255, 255)
        };

        let f = pct / 100.0;
        let r = (base.0 as f32 * f + mix.0 as f32 * (1.0 - f)) as u8;
        let g = (base.1 as f32 * f + mix.1 as f32 * (1.0 - f)) as u8;
        let b = (base.2 as f32 * f + mix.2 as f32 * (1.0 - f)) as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Collect everything from the current position until the matching `}` that
    /// closes the enclosing group (depth-0 close brace), without consuming it.
    /// Used by font-declaration commands like `\itshape`.
    fn collect_until_close_brace(&mut self) -> String {
        let mut result = String::new();
        let mut depth  = 0usize;
        while let Some(c) = self.peek() {
            match c {
                '{' => { depth += 1; result.push(c); self.lexer.pos += 1; }
                '}' => {
                    if depth == 0 {
                        // Leave the brace for the outer parser to consume
                        break;
                    }
                    depth -= 1;
                    result.push(c);
                    self.lexer.pos += 1;
                }
                _ => { result.push(c); self.lexer.pos += c.len_utf8(); }
            }
        }
        result
    }

    /// Convert a \definecolor model + spec to a CSS color value.
    fn latex_color_model(model: &str, spec: &str) -> String {
        match model.trim().to_lowercase().as_str() {
            "rgb" => {
                // spec is "r,g,b" where each is 0–1
                let parts: Vec<f32> = spec.split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                if parts.len() == 3 {
                    let r = (parts[0] * 255.0) as u8;
                    let g = (parts[1] * 255.0) as u8;
                    let b = (parts[2] * 255.0) as u8;
                    return format!("#{:02x}{:02x}{:02x}", r, g, b);
                }
                spec.to_string()
            }
            "rgb255" => {
                let parts: Vec<u8> = spec.split(',')
                    .filter_map(|s| s.trim().parse::<u8>().ok())
                    .collect();
                if parts.len() == 3 {
                    return format!("#{:02x}{:02x}{:02x}", parts[0], parts[1], parts[2]);
                }
                spec.to_string()
            }
            "HTML" | "html" => format!("#{}", spec.trim().trim_start_matches('#')),
            "gray" | "grey" => {
                let v: f32 = spec.trim().parse().unwrap_or(0.5);
                let g = (v * 255.0) as u8;
                format!("#{:02x}{:02x}{:02x}", g, g, g)
            }
            "cmyk" => {
                let p: Vec<f32> = spec.split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                if p.len() == 4 {
                    let r = ((1.0 - p[0]) * (1.0 - p[3]) * 255.0) as u8;
                    let g = ((1.0 - p[1]) * (1.0 - p[3]) * 255.0) as u8;
                    let b = ((1.0 - p[2]) * (1.0 - p[3]) * 255.0) as u8;
                    return format!("#{:02x}{:02x}{:02x}", r, g, b);
                }
                spec.to_string()
            }
            // named — treat spec as a named color
            _ => Self::latex_color(spec),
        }
    }

    /// Normalise a fancyhdr position argument like "LE,RO" → "L,R".
    /// Strips the even/odd suffix (E/O) so only the base slot (L/C/R) remains.
    fn normalize_fancy_pos(raw: &str) -> String {
        raw.split(',')
            .map(|part| {
                let p = part.trim().to_ascii_uppercase();
                // Keep only the leading L/C/R, drop any trailing O/E
                match p.chars().next() {
                    Some('L') => "L",
                    Some('C') => "C",
                    Some('R') => "R",
                    _         => "C",
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Returns the registered label names so callers can emit anchors for
    /// content whose body never reaches the HTML DOM (e.g. MathJax blocks).
    fn extract_and_register_labels(
        raw: &str,
        value: &str,
        prefix: &str,
        labels: &mut HashMap<String, String>,
    ) -> Vec<String> {
        let tag = "\\label{";
        let mut pos = 0;
        let mut registered = Vec::new();

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
                    registered.push(key.to_string());
                }
            } else {
                pos += 1;
            }
        }

        registered
    }

}