use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct Ranges;

impl Ranges {

    // Expands a numeric range token `{A..B}` into one line per value, replacing
    // the token everywhere it appears (so a matching range in `as "name"` is
    // expanded consistently). Lines without a valid range are returned as-is.
    //
    // The width of `B` controls zero-padding, and the leading part of `A` that
    // isn't the counter is kept as a prefix — so `{2203.08877..08880}` yields
    // `2203.08877`, `2203.08878`, `2203.08879`, `2203.08880`.
    pub fn expand_line(&self, line: &str) -> Vec<String> {
        let pattern = Regex::new(BlocksRegExp::GET_RANGE).unwrap();

        let Some(caps) = pattern.captures(line) else {
            return vec![line.to_string()];
        };

        let token = caps.get(0).unwrap().as_str();
        let start = caps.get(1).unwrap().as_str();
        let end = caps.get(2).unwrap().as_str();

        match Self::values(start, end) {
            Some(values) => values.into_iter()
                .map(|value| line.replace(token, &value))
                .collect(),

            None => vec![line.to_string()],
        }
    }

    // Resolves a `{A..B}` token to its list of values; returns the source
    // unchanged (as a single item) when it isn't a valid range.
    pub fn list(&self, source: &str) -> Vec<String> {
        let pattern = Regex::new(BlocksRegExp::GET_RANGE).unwrap();

        if let Some(caps) = pattern.captures(source) {
            if let Some(values) = Self::values(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()) {
                return values;
            }
        }

        vec![source.to_string()]
    }

    fn values(start: &str, end: &str) -> Option<Vec<String>> {
        let width = end.len();

        // The counter is the trailing `width` chars of `A`; the rest is a literal
        // prefix carried onto every value.
        let (prefix, counter) = if start.len() >= width {
            start.split_at(start.len() - width)
        } else {
            ("", start)
        };

        let from: u64 = counter.parse().ok()?;
        let to: u64 = end.parse().ok()?;

        // Only ascending ranges expand; a guard keeps a typo from generating a
        // runaway number of entries.
        if from > to || to - from > 10_000 {
            return None;
        }

        Some(
            (from..=to)
                .map(|n| format!("{}{:0width$}", prefix, n, width = width))
                .collect()
        )
    }

}
