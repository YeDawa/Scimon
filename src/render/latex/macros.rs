#[derive(Debug, Clone)]
pub enum MacroPiece {
    /// Literal body text (`##` already collapsed to `#`)
    Text(String),
    /// Parameter slot `#n` (1-based)
    Param(usize),
}

/// A macro compiled at definition time. The body is split into structural
/// pieces once, so call-time expansion is a single splice — parameter-like
/// text inside arguments (`\cmd{a#2b}`) is never re-substituted, unlike
/// sequential string replacement.
#[derive(Debug, Clone)]
pub struct MacroDef {
    pub params: usize,
    /// When present, `#1` is optional with this default value
    pub default: Option<String>,
    pub pieces: Vec<MacroPiece>,
}

impl MacroDef {

    /// Compile a macro body: `#1`..`#9` become parameter slots, `##` a
    /// literal `#`, everything else literal text.
    pub fn compile(params: usize, default: Option<String>, body: &str) -> Self {
        let mut pieces = Vec::new();
        let mut text = String::new();
        let mut chars = body.chars().peekable();

        while let Some(c) = chars.next() {
            if c != '#' {
                text.push(c);
                continue;
            }
            match chars.peek() {
                Some('#') => {
                    chars.next();
                    text.push('#');
                }
                Some(&digit) if ('1'..='9').contains(&digit) => {
                    chars.next();
                    if !text.is_empty() {
                        pieces.push(MacroPiece::Text(std::mem::take(&mut text)));
                    }
                    pieces.push(MacroPiece::Param(digit.to_digit(10).unwrap() as usize));
                }
                _ => text.push('#'),
            }
        }
        if !text.is_empty() {
            pieces.push(MacroPiece::Text(text));
        }

        MacroDef { params, default, pieces }
    }

    /// Splice the arguments into the compiled body in one pass.
    /// Missing arguments expand to nothing.
    pub fn expand(&self, args: &[String]) -> String {
        let mut out = String::new();
        for piece in &self.pieces {
            match piece {
                MacroPiece::Text(text) => out.push_str(text),
                MacroPiece::Param(slot) => {
                    if let Some(arg) = args.get(slot - 1) {
                        out.push_str(arg);
                    }
                }
            }
        }
        out
    }

}
