pub struct Lexer {
    /// Source text; `pos` is a **byte** offset into this string.
    pub(crate) input: String,
    pub(crate) pos: usize,
}

impl Lexer {

    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
            pos: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Character-level reads
    // -----------------------------------------------------------------------

    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Look `n` chars ahead from the current position (0 = same as peek).
    pub fn peek_ahead(&self, n: usize) -> Option<char> {
        self.input[self.pos..].chars().nth(n)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.input[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.next_char();
        }
    }

    // -----------------------------------------------------------------------
    // Token-level reads
    // -----------------------------------------------------------------------

    /// Read an alphabetic command word (letters only) from current position.
    pub fn command_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                word.push(c);
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        word
    }

    /// Collect plain text until a special character.
    pub fn text_run(&mut self) -> String {
        let mut text = String::new();
        while let Some(c) = self.peek() {
            if matches!(c, '\\' | '{' | '}' | '^' | '_' | '%' | '$' | '~' | '&' | '`' | '\'' | '-') {
                break;
            }
            text.push(c);
            self.pos += c.len_utf8();
        }
        text
    }

    /// Collect everything inside the next `{…}`, respecting nesting.
    /// `\{` and `\}` are literal braces and do not affect the depth.
    pub fn brace_group(&mut self) -> String {
        self.skip_whitespace();
        if self.peek() != Some('{') {
            return String::new();
        }
        self.next_char(); // consume opening '{'

        let mut content = String::new();
        let mut depth = 1usize;
        while depth > 0 {
            let c = match self.next_char() {
                Some(c) => c,
                None => break,
            };
            if c == '\\' {
                content.push('\\');
                if matches!(self.peek(), Some('{') | Some('}')) {
                    let escaped = self.next_char().unwrap();
                    content.push(escaped);
                }
            } else if c == '{' {
                depth += 1;
                content.push('{');
            } else if c == '}' {
                depth -= 1;
                if depth > 0 {
                    content.push('}');
                }
            } else {
                content.push(c);
            }
        }
        content
    }

    /// Consume an optional `[…]` argument, returning its contents.
    /// Nested `[…]` pairs, `{…}` groups and escaped characters are kept
    /// whole — `[title={a]b}]` returns `title={a]b}`.
    pub fn optional_group(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.peek() != Some('[') {
            return None;
        }
        self.next_char(); // consume opening '['

        let mut content = String::new();
        let mut brackets = 0usize;
        let mut braces = 0usize;
        while let Some(c) = self.next_char() {
            match c {
                '\\' => {
                    content.push('\\');
                    // escaped single char: copy verbatim so \] \{ \% etc.
                    // never affect the nesting depths
                    if self.peek().is_some_and(|next| !next.is_alphanumeric()) {
                        content.push(self.next_char().unwrap());
                    }
                }
                '{' => { braces += 1; content.push(c); }
                '}' => { braces = braces.saturating_sub(1); content.push(c); }
                '[' if braces == 0 => { brackets += 1; content.push(c); }
                ']' if braces == 0 => {
                    if brackets == 0 {
                        return Some(content);
                    }
                    brackets -= 1;
                    content.push(c);
                }
                _ => content.push(c),
            }
        }
        Some(content)
    }

    /// Read everything up to the matching `\end{env}`, consuming the tag.
    /// Nested `\begin{env}` blocks of the same name are counted, so
    /// environments like center-inside-center survive intact.
    pub fn until_env_end(&mut self, env: &str) -> String {
        let begin_tag = format!("\\begin{{{}}}", env);
        let end_tag = format!("\\end{{{}}}", env);

        let mut raw = String::new();
        let mut depth = 0usize;
        while self.pos < self.input.len() {
            if self.input[self.pos..].starts_with(end_tag.as_str()) {
                self.pos += end_tag.len();
                if depth == 0 {
                    break;
                }
                depth -= 1;
                raw.push_str(&end_tag);
                continue;
            }
            if self.input[self.pos..].starts_with(begin_tag.as_str()) {
                self.pos += begin_tag.len();
                depth += 1;
                raw.push_str(&begin_tag);
                continue;
            }
            if let Some(c) = self.next_char() {
                raw.push(c);
            }
        }
        raw
    }

}
