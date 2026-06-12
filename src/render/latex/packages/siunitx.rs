//! siunitx — \SI{3e8}{\meter\per\second}, \qty, \si, \unit, \num, \ang

use std::collections::HashMap;

use crate::render::latex::{
    parser::Parser,
    tex_ast::LatexNode,
    packages::LatexPackage,
};

pub struct Siunitx;

impl LatexPackage for Siunitx {

    fn commands(&self) -> &'static [&'static str] {
        &["SI", "qty", "si", "unit", "num", "ang"]
    }

    fn command(
        &self,
        command: &str,
        _starred: bool,
        parser: &mut Parser,
        _labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        parser.parse_optional_arg(); // [options] — ignored

        let html = match command {
            // \SI{value}{unit} / \qty{value}{unit}
            "SI" | "qty" => {
                let value = parser.parse_braces_content();
                let unit = parser.parse_braces_content();
                format!(
                    "<span class=\"si-value\">{}</span>\u{202F}<span class=\"si-unit\">{}</span>",
                    format_number(&value), format_unit(&unit),
                )
            }
            "si" | "unit" => format!(
                "<span class=\"si-unit\">{}</span>",
                format_unit(&parser.parse_braces_content()),
            ),
            "num" => format!(
                "<span class=\"si-value\">{}</span>",
                format_number(&parser.parse_braces_content()),
            ),
            // \ang{45} or \ang{30;10;5} (deg;min;sec)
            _ => format!(
                "<span class=\"si-ang\">{}</span>",
                format_angle(&parser.parse_braces_content()),
            ),
        };

        vec![LatexNode::Text(html)]
    }

}

/// Format a siunitx number: group digits, handle scientific notation.
fn format_number(raw: &str) -> String {
    let s = raw.trim();

    // Handle scientific notation: 1e6, 1.5e-3, 1.5E10
    if let Some(pos) = s.to_lowercase().find('e') {
        let mantissa = &s[..pos];
        let exp = &s[pos + 1..];
        return format!("{} × 10<sup>{}</sup>", mantissa, exp);
    }

    // Group integer digits in threes
    let (int_part, dec_part) = if let Some(dot) = s.find('.') {
        (&s[..dot], Some(&s[dot..]))
    } else {
        (s, None)
    };
    let digits: String = int_part.chars().rev().enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 3 == 0 && c.is_ascii_digit() {
                vec!['\u{202F}', c] // narrow no-break space
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .chars().rev().collect();

    match dec_part {
        Some(d) => format!("{}{}", digits, d),
        None    => digits,
    }
}

/// "45" → 45°; "30;10;5" → 30°10'5″
fn format_angle(raw: &str) -> String {
    if !raw.contains(';') {
        return format!("{}°", raw);
    }

    let parts: Vec<&str> = raw.split(';').collect();
    let mut s = String::new();
    if !parts[0].is_empty() { s.push_str(&format!("{}°", parts[0])); }
    if parts.len() > 1 && !parts[1].is_empty() { s.push_str(&format!("{}'", parts[1])); }
    if parts.len() > 2 && !parts[2].is_empty() { s.push_str(&format!("{}″", parts[2])); }
    s
}

/// Convert siunitx unit macros to HTML (e.g. \meter\per\second → m s⁻¹).
fn format_unit(raw: &str) -> String {
    let mut result = String::new();
    let mut per = false;
    let mut i = 0;
    let chars: Vec<char> = raw.chars().collect();

    while i < chars.len() {
        if chars[i] == '\\' {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i].is_alphabetic() { i += 1; }
            let cmd: String = chars[start..i].iter().collect();
            match cmd.as_str() {
                "per"        => { per = true; continue; }
                "square"     => { result.push_str("<sup>2</sup>"); continue; }
                "cubic"      => { result.push_str("<sup>3</sup>"); continue; }
                "squared"    => { result.push_str("<sup>2</sup>"); continue; }
                "cubed"      => { result.push_str("<sup>3</sup>"); continue; }
                "meter" | "metre"     => result.push('m'),
                "gram"                => result.push('g'),
                "kilogram"            => result.push_str("kg"),
                "second"              => result.push('s'),
                "minute"              => result.push_str("min"),
                "hour"                => result.push('h'),
                "kelvin"              => result.push('K'),
                "mole"                => result.push_str("mol"),
                "ampere"              => result.push('A'),
                "candela"             => result.push_str("cd"),
                "newton"              => result.push('N'),
                "pascal"              => result.push_str("Pa"),
                "joule"               => result.push('J'),
                "watt"                => result.push('W'),
                "volt"                => result.push('V'),
                "ohm"                 => result.push('Ω'),
                "siemens"             => result.push('S'),
                "farad"               => result.push('F'),
                "henry"               => result.push('H'),
                "tesla"               => result.push('T'),
                "hertz"               => result.push_str("Hz"),
                "liter" | "litre"     => result.push('L'),
                // SI prefixes used standalone (e.g. \kilo\gram)
                "kilo"   => result.push('k'),
                "mega"   => result.push('M'),
                "giga"   => result.push('G'),
                "tera"   => result.push('T'),
                "milli"  => result.push('m'),
                "micro"  => result.push('μ'),
                "nano"   => result.push('n'),
                "pico"   => result.push('p'),
                "centi"  => result.push('c'),
                "deci"   => result.push('d'),
                "hecto"  => result.push('h'),
                "degree" | "degreeCelsius" => result.push('°'),
                "celsius"  => result.push_str("°C"),
                "fahrenheit" => result.push_str("°F"),
                "radian"   => result.push_str("rad"),
                "steradian"=> result.push_str("sr"),
                "percent"  => result.push('%'),
                other      => result.push_str(other),
            }
            if per {
                // wrap last unit in superscript -1
                result.push_str("<sup>−1</sup>");
                per = false;
            }
        } else if chars[i].is_whitespace() {
            result.push('\u{202F}'); // narrow no-break space between units
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}
