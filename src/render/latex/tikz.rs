use std::collections::HashMap;

use crate::render::latex::{
    parser::math_symbol,
    context::RenderContext,
};

type Point = (f64, f64);

/// Named coordinate: position + node half-extents in px (0 for \coordinate).
/// Extents let edges between named nodes stop at the node boundary.
type NamedPoint = (Point, (f64, f64));

/// Pixels per TikZ unit (1cm at 96 dpi)
const PPU: f64 = 37.8;

// ---------------------------------------------------------------------------
// Parsed TikZ structures (coordinates in TikZ space, y pointing up)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct TikzPicture {
    pub scale:    f64,
    pub elements: Vec<TikzElement>,
}

#[derive(Debug, Clone)]
pub enum TikzElement {
    Path(TikzPath),
    Node(TikzNode),
}

#[derive(Debug, Clone)]
pub struct TikzPath {
    pub segments: Vec<PathSeg>,
    pub stroke:   bool,
    pub fill:     bool,
    pub options:  TikzOptions,
    /// inline edge labels: `-- node[above] {text}`
    pub labels:   Vec<TikzNode>,
}

#[derive(Debug, Clone, Default)]
pub struct TikzOptions {
    pub color:       Option<String>,
    pub fill_color:  Option<String>,
    pub width:       f64,
    pub dash:        String,
    pub arrow_start: bool,
    pub arrow_end:   bool,
    pub step:        f64,
    /// corner radius in px from `rounded corners`
    pub rounding:    f64,
}

#[derive(Debug, Clone)]
pub enum PathSeg {
    Move(Point),
    Line(Point),
    Curve { c1: Point, c2: Point, to: Point },
    Circle { center: Point, radius: f64 },
    Ellipse { center: Point, rx: f64, ry: f64 },
    Rect { from: Point, to: Point },
    /// circular arc from `from`, sweeping start° → end° at `radius`
    Arc { from: Point, start: f64, end: f64, radius: f64 },
    Grid { from: Point, to: Point, step: f64 },
    /// circuitikz `to[R=...]` two-terminal component
    Component { from: Point, to: Point, kind: String, label: String },
    Close,
}

#[derive(Debug, Clone)]
pub struct TikzNode {
    pub at:        Point,
    pub text:      String,
    /// "above", "below left", ... — "" means centered on the point
    pub placement: String,
    /// circle | rectangle, drawn when the node has the draw option
    pub shape:     Option<String>,
    pub fill:      Option<String>,
    pub color:     Option<String>,
    /// automata library: \node[state, initial] / [state, accepting]
    pub initial:   bool,
    pub accepting: bool,
}

/// Per-picture parsing state: unit scale, named styles, spacing defaults
struct Scope {
    ppu:           f64,
    node_distance: f64,
    styles:        HashMap<String, String>,
    /// picture-level options applied to every path (e.g. [->, thick])
    global:        String,
}

// ---------------------------------------------------------------------------
// TikZ: parsing + SVG rendering
// ---------------------------------------------------------------------------
pub struct Tikz;

impl Tikz {

    // -----------------------------------------------------------------------
    // Parsing
    // -----------------------------------------------------------------------

    /// Parse a tikzpicture body. `env_options` is the [scale=...] group from
    /// \begin{tikzpicture}[...]; `doc_styles` are \tikzstyle/\tikzset
    /// definitions collected from the whole document.
    /// Returns None when nothing drawable is found.
    pub fn parse(
        raw: &str,
        env_options: Option<&str>,
        doc_styles: &HashMap<String, String>,
    ) -> Option<TikzPicture> {
        let mut styles = Self::builtin_styles();
        styles.extend(doc_styles.clone());

        let mut scope = Scope {
            ppu: PPU,
            node_distance: 1.5,
            styles,
            global: String::new(),
        };

        // picture-level options: scale and node distance are consumed here,
        // everything else becomes a default for every path in the picture
        let env_expanded = Self::expand_options(&scope, env_options.unwrap_or(""), 4);
        let mut scale = 1.0;
        let mut global = Vec::new();
        for part in Self::split_commas(&env_expanded) {
            match part.split_once('=') {
                Some((key, value)) if key.trim() == "scale" => {
                    scale = value.trim().parse().unwrap_or(1.0);
                }
                Some((key, value)) if key.trim() == "node distance" => {
                    scope.node_distance = Self::parse_unit(value).unwrap_or(1.5);
                }
                _ => global.push(part),
            }
        }
        scope.ppu = PPU * scale;
        scope.global = global.join(", ");

        let cleaned = Self::expand_foreach(&Self::strip_comments(raw));
        let mut coords: HashMap<String, NamedPoint> = HashMap::new();
        let mut elements = Vec::new();

        for statement in Self::split_statements(&cleaned) {
            let statement = statement.trim();
            let Some(rest) = statement.strip_prefix('\\') else { continue };

            let command_end = rest.find(|c: char| !c.is_alphabetic()).unwrap_or(rest.len());
            let (command, rest) = rest.split_at(command_end);

            match command {
                "draw" | "path" => {
                    let stroke = command == "draw";
                    Self::parse_path(&scope, rest, stroke, false, &mut coords, &mut elements);
                }
                "fill" | "shade" => {
                    Self::parse_path(&scope, rest, false, true, &mut coords, &mut elements);
                }
                "filldraw" | "shadedraw" => {
                    Self::parse_path(&scope, rest, true, true, &mut coords, &mut elements);
                }
                "node" => {
                    if let Some(node) = Self::parse_node_statement(&scope, rest, &mut coords) {
                        elements.push(TikzElement::Node(node));
                    }
                }
                "coordinate" => {
                    Self::parse_coordinate_statement(&scope, rest, &mut coords);
                }
                // styles defined inside the picture body
                "tikzset" => {
                    if let Some((content, _)) = Self::take_braces(rest) {
                        for part in Self::split_commas(&content) {
                            if let Some((key, value)) = part.split_once("/.style") {
                                let value = value.trim_start().strip_prefix('=').unwrap_or(&value);
                                scope.styles.insert(
                                    key.trim().to_string(),
                                    Self::strip_outer_braces(value),
                                );
                            }
                        }
                    }
                }
                "tikzstyle" => {
                    if let Some((name, after)) = Self::take_braces(rest) {
                        let after = after.trim_start().strip_prefix('=').unwrap_or(after);
                        let (options, _) = Self::take_brackets(after);
                        scope.styles.insert(name.trim().to_string(), options);
                    }
                }
                _ => {}
            }
        }

        if elements.is_empty() {
            None
        } else {
            Some(TikzPicture { scale, elements })
        }
    }

    /// Styles the automata library would otherwise provide
    fn builtin_styles() -> HashMap<String, String> {
        let mut styles = HashMap::new();
        styles.insert("state".to_string(), "circle, draw".to_string());
        styles.insert("vertex".to_string(), "circle, draw".to_string());
        styles
    }

    /// Replace style names with their definitions, recursively.
    /// "state" additionally pulls in the user's "every state" style.
    fn expand_options(scope: &Scope, raw: &str, depth: usize) -> String {
        if depth == 0 {
            return raw.to_string();
        }

        let mut parts = Vec::new();
        for part in Self::split_commas(raw) {
            let name = part.trim();
            let style = scope.styles.get(name)
                .or_else(|| scope.styles.get(&name.to_lowercase()));

            match style {
                Some(style) => {
                    parts.push(Self::expand_options(scope, style, depth - 1));
                    if name == "state" {
                        if let Some(every) = scope.styles.get("every state") {
                            parts.push(Self::expand_options(scope, every, depth - 1));
                        }
                    }
                }
                None => parts.push(part),
            }
        }
        parts.join(", ")
    }

    // -----------------------------------------------------------------------
    // \foreach expansion
    // -----------------------------------------------------------------------

    /// Textually expand \foreach \x in {...} {body} (and the single-statement
    /// form ending at ';'). Handles "..." ranges and \x/\y value pairs.
    fn expand_foreach(input: &str) -> String {
        let mut text = input.to_string();

        for _ in 0..64 {
            let Some(pos) = text.find("\\foreach") else { break };
            let after = &text[pos + 8..];

            let Some(in_pos) = after.find(" in") else { break };
            let vars: Vec<String> = after[..in_pos]
                .split('/')
                .map(|v| v.trim().to_string())
                .filter(|v| v.starts_with('\\'))
                .collect();

            let Some((list, rest)) = Self::take_braces(&after[in_pos + 3..]) else { break };
            let trimmed = rest.trim_start();
            let (body, tail) = if trimmed.starts_with('{') {
                match Self::take_braces(trimmed) {
                    Some((body, tail)) => (body, tail.to_string()),
                    None => break,
                }
            } else {
                match trimmed.find(';') {
                    Some(end) => (trimmed[..=end].to_string(), trimmed[end + 1..].to_string()),
                    None => break,
                }
            };

            let mut expansion = String::new();
            for value in Self::foreach_values(&list) {
                let bindings: Vec<&str> = value.split('/').map(str::trim).collect();
                let mut instance = body.clone();
                for (i, var) in vars.iter().enumerate() {
                    instance = Self::substitute_var(&instance, var, bindings.get(i).copied().unwrap_or(""));
                }
                expansion.push_str(&instance);
                expansion.push('\n');
            }

            text = format!("{}{}{}", &text[..pos], expansion, tail);
        }

        text
    }

    /// "1,2,3" / "0,...,4" / "0,0.5,...,2" / "1/a, 2/b"
    fn foreach_values(list: &str) -> Vec<String> {
        let items = Self::split_commas(list);

        if let Some(dots) = items.iter().position(|item| item == "...") {
            let before: Vec<f64> = items[..dots].iter()
                .filter_map(|item| item.trim().parse().ok())
                .collect();
            let end = items.get(dots + 1).and_then(|item| item.trim().parse::<f64>().ok());

            if let (Some(&start), Some(end)) = (before.first(), end) {
                let step = if before.len() >= 2 { before[1] - before[0] } else { 1.0 };
                if step.abs() > 1e-9 && (end - start) * step >= 0.0 {
                    let mut values = Vec::new();
                    let mut v = start;
                    while (v - end) * step.signum() <= 1e-9 {
                        values.push(Self::format_number(v));
                        v += step;
                    }
                    return values;
                }
            }
        }

        items
    }

    fn format_number(v: f64) -> String {
        if (v - v.round()).abs() < 1e-9 {
            format!("{}", v.round() as i64)
        } else {
            format!("{}", (v * 1000.0).round() / 1000.0)
        }
    }

    /// Replace \x with `value` wherever it is not a prefix of a longer word
    fn substitute_var(body: &str, var: &str, value: &str) -> String {
        let mut out = String::with_capacity(body.len());
        let mut rest = body;

        while let Some(pos) = rest.find(var) {
            let whole_word = rest[pos + var.len()..]
                .chars()
                .next()
                .map_or(true, |c| !c.is_alphabetic());

            out.push_str(&rest[..pos]);
            out.push_str(if whole_word { value } else { var });
            rest = &rest[pos + var.len()..];
        }
        out.push_str(rest);
        out
    }

    // -----------------------------------------------------------------------
    // Low-level text helpers
    // -----------------------------------------------------------------------

    /// Remove % comments (but keep escaped \%)
    fn strip_comments(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    out.push(c);
                    if let Some(&next) = chars.peek() {
                        out.push(next);
                        chars.next();
                    }
                }
                '%' => {
                    for next in chars.by_ref() {
                        if next == '\n' { out.push('\n'); break; }
                    }
                }
                _ => out.push(c),
            }
        }
        out
    }

    /// Split on top-level ';' (braces and brackets protect their content)
    fn split_statements(raw: &str) -> Vec<String> {
        let mut statements = Vec::new();
        let mut current = String::new();
        let mut depth = 0usize;

        for c in raw.chars() {
            match c {
                '{' | '[' => { depth += 1; current.push(c); }
                '}' | ']' => { depth = depth.saturating_sub(1); current.push(c); }
                ';' if depth == 0 => statements.push(std::mem::take(&mut current)),
                _ => current.push(c),
            }
        }
        statements
    }

    /// Split on top-level ',' (braces, brackets and parens protect content)
    fn split_commas(raw: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut depth = 0usize;

        for c in raw.chars() {
            match c {
                '{' | '[' | '(' => { depth += 1; current.push(c); }
                '}' | ']' | ')' => { depth = depth.saturating_sub(1); current.push(c); }
                ',' if depth == 0 => { parts.push(current.trim().to_string()); current.clear(); }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }
        parts.retain(|p| !p.is_empty());
        parts
    }

    fn strip_outer_braces(s: &str) -> String {
        s.trim().trim_start_matches('{').trim_end_matches('}').trim().to_string()
    }

    /// Leading [options] group; returns (content, remainder)
    fn take_brackets(s: &str) -> (String, &str) {
        let trimmed = s.trim_start();
        if !trimmed.starts_with('[') {
            return (String::new(), s);
        }
        let mut depth = 0usize;
        for (i, c) in trimmed.char_indices() {
            match c {
                '[' | '{' => depth += 1,
                ']' | '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return (trimmed[1..i].to_string(), &trimmed[i + 1..]);
                    }
                }
                _ => {}
            }
        }
        (String::new(), s)
    }

    /// Leading {balanced} group; returns (content, remainder)
    fn take_braces(s: &str) -> Option<(String, &str)> {
        let trimmed = s.trim_start();
        if !trimmed.starts_with('{') {
            return None;
        }
        let mut depth = 0usize;
        for (i, c) in trimmed.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((trimmed[1..i].to_string(), &trimmed[i + 1..]));
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Leading (group); returns (content, remainder)
    fn take_parens(s: &str) -> Option<(String, &str)> {
        let trimmed = s.trim_start();
        if !trimmed.starts_with('(') {
            return None;
        }
        let close = trimmed.find(')')?;
        Some((trimmed[1..close].to_string(), &trimmed[close + 1..]))
    }

    /// True when `s` continues with a non-word character (token boundary)
    fn token_boundary(s: &str) -> bool {
        s.chars().next().map_or(true, |c| !c.is_alphanumeric())
    }

    // -----------------------------------------------------------------------
    // Coordinates
    // -----------------------------------------------------------------------

    /// "1.5cm" / "30:2" (polar) / "name" / "name.anchor" → point in TikZ
    /// units plus the half-extents of the referenced node (0 for literals
    /// and anchored references, which already sit on the boundary)
    fn resolve_coordinate(
        raw: &str,
        coords: &HashMap<String, NamedPoint>,
        ppu: f64,
    ) -> Option<NamedPoint> {
        let raw = raw.trim();

        if let Some((x, y)) = raw.split_once(',') {
            return Some(((Self::parse_unit(x)?, Self::parse_unit(y)?), (0.0, 0.0)));
        }
        if let Some((angle, radius)) = raw.split_once(':') {
            let angle: f64 = angle.trim().parse().ok()?;
            let radius = Self::parse_unit(radius)?;
            let rad = angle.to_radians();
            return Some(((radius * rad.cos(), radius * rad.sin()), (0.0, 0.0)));
        }

        let (name, anchor) = match raw.split_once('.') {
            Some((name, anchor)) => (name.trim(), anchor.trim()),
            None => (raw, ""),
        };
        let &(center, extents) = coords.get(name)?;

        if anchor.is_empty() || anchor == "center" {
            return Some((center, extents));
        }

        // boundary anchors: offset by the node's half-extents
        let (hw, hh) = (extents.0 / ppu, extents.1 / ppu);
        let offset = match anchor {
            "north"      => (0.0, hh),
            "south"      => (0.0, -hh),
            "east"       => (hw, 0.0),
            "west"       => (-hw, 0.0),
            "north east" => (hw, hh),
            "north west" => (-hw, hh),
            "south east" => (hw, -hh),
            "south west" => (-hw, -hh),
            _            => (0.0, 0.0),
        };
        Some(((center.0 + offset.0, center.1 + offset.1), (0.0, 0.0)))
    }

    fn parse_unit(value: &str) -> Option<f64> {
        let value = value.trim();
        let split = value.find(|c: char| c.is_alphabetic()).unwrap_or(value.len());
        let number: f64 = value[..split].trim().parse().ok()?;

        let factor = match value[split..].trim() {
            "cm" | ""  => 1.0,
            "mm"       => 0.1,
            "pt"       => 0.0352,
            "in"       => 2.54,
            _          => 1.0,
        };
        Some(number * factor)
    }

    /// Next coordinate in the path stream, handling ++(relative) and (named).
    fn next_coordinate<'a>(
        s: &'a str,
        current: Point,
        coords: &HashMap<String, NamedPoint>,
        ppu: f64,
    ) -> Option<(NamedPoint, &'a str)> {
        let trimmed = s.trim_start();
        let (relative, trimmed) = if let Some(rest) = trimmed.strip_prefix("++") {
            (true, rest)
        } else if let Some(rest) = trimmed.strip_prefix('+') {
            (true, rest)
        } else {
            (false, trimmed)
        };

        let (inner, rest) = Self::take_parens(trimmed)?;
        let (mut point, extents) = Self::resolve_coordinate(&inner, coords, ppu)?;
        if relative {
            point = (current.0 + point.0, current.1 + point.1);
        }
        Some(((point, extents), rest))
    }

    /// Half-extents (px) of a node's visual footprint, used to draw its
    /// shape and to stop incoming edges at its boundary
    fn node_extents(text_len: usize, shape: Option<&str>) -> (f64, f64) {
        let half_w = text_len as f64 * 4.2 + 6.0;
        match shape {
            Some("circle") => {
                let r = half_w.max(12.0);
                (r, r)
            }
            Some(_) => (half_w, 11.0),
            None    => (half_w * 0.8, 8.0),
        }
    }

    /// Distance from a node's center to its boundary along direction `u`
    /// (approximating rectangles by their inscribed ellipse)
    fn boundary_shrink(extents: (f64, f64), u: Point) -> f64 {
        ((extents.0 * u.0).powi(2) + (extents.1 * u.1).powi(2)).sqrt()
    }

    /// Pull a segment's endpoints back to the node boundaries on both ends
    fn shrink_segment(
        from: Point, from_ext: (f64, f64),
        to: Point, to_ext: (f64, f64),
        ppu: f64,
    ) -> (Point, Point) {
        let delta = (to.0 - from.0, to.1 - from.1);
        let length = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
        if length < 1e-9 {
            return (from, to);
        }

        let u = (delta.0 / length, delta.1 / length);
        let s0 = Self::boundary_shrink(from_ext, u) / ppu;
        let s1 = Self::boundary_shrink(to_ext, u) / ppu;
        if s0 + s1 >= length {
            return (from, to);
        }

        (
            (from.0 + u.0 * s0, from.1 + u.1 * s0),
            (to.0 - u.0 * s1, to.1 - u.1 * s1),
        )
    }

    fn rotate(v: Point, degrees: f64) -> Point {
        let rad = degrees.to_radians();
        (
            v.0 * rad.cos() - v.1 * rad.sin(),
            v.0 * rad.sin() + v.1 * rad.cos(),
        )
    }

    // -----------------------------------------------------------------------
    // Statements
    // -----------------------------------------------------------------------

    fn parse_path(
        scope: &Scope,
        input: &str,
        stroke: bool,
        fill: bool,
        coords: &mut HashMap<String, NamedPoint>,
        out: &mut Vec<TikzElement>,
    ) {
        let (options_raw, mut rest) = Self::take_brackets(input);
        let merged = format!(
            "{}, {}",
            scope.global,
            Self::expand_options(scope, &options_raw, 4),
        );
        let options = Self::parse_path_options(&merged);
        let fill = fill || options.fill_color.is_some();

        let mut segments: Vec<PathSeg> = Vec::new();
        let mut labels: Vec<TikzNode> = Vec::new();
        let mut edges: Vec<TikzElement> = Vec::new();
        let mut current: Point = (0.0, 0.0);
        let mut current_extents: (f64, f64) = (0.0, 0.0);
        let mut pending_line = false;
        let mut pending_label: Option<TikzNode> = None;

        loop {
            let trimmed = rest.trim_start();
            if trimmed.is_empty() {
                break;
            }

            // coordinate
            if trimmed.starts_with('(') || trimmed.starts_with('+') {
                if let Some(((point, extents), remainder)) =
                    Self::next_coordinate(trimmed, current, coords, scope.ppu)
                {
                    if segments.is_empty() {
                        segments.push(PathSeg::Move(point));
                    } else if pending_line {
                        let (start, target) = Self::shrink_segment(
                            current, current_extents, point, extents, scope.ppu,
                        );
                        if start != current {
                            if let Some(PathSeg::Move(p) | PathSeg::Line(p)) = segments.last_mut() {
                                if *p == current { *p = start; }
                            }
                        }
                        segments.push(PathSeg::Line(target));
                        if let Some(mut label) = pending_label.take() {
                            label.at = ((current.0 + point.0) / 2.0, (current.1 + point.1) / 2.0);
                            labels.push(label);
                        }
                    } else {
                        segments.push(PathSeg::Move(point));
                    }
                    current = point;
                    current_extents = extents;
                    pending_line = false;
                    rest = remainder;
                    continue;
                }
                rest = &trimmed[1..];
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("--") {
                pending_line = true;
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("cycle") {
                segments.push(PathSeg::Close);
                pending_line = false;
                rest = remainder;
                continue;
            }

            // .. controls (c1) and (c2) .. (to)
            if let Some(remainder) = trimmed.strip_prefix("..") {
                let after = remainder.trim_start();
                if let Some(after) = after.strip_prefix("controls") {
                    let Some(((c1, _), after)) = Self::next_coordinate(after, current, coords, scope.ppu) else { break };
                    let after = after.trim_start();
                    let (c2, after) = if let Some(after) = after.strip_prefix("and") {
                        match Self::next_coordinate(after, current, coords, scope.ppu) {
                            Some(((point, _), after)) => (point, after),
                            None => break,
                        }
                    } else {
                        (c1, after)
                    };
                    let after = after.trim_start();
                    let Some(after) = after.strip_prefix("..") else { break };
                    let Some(((to, _), after)) = Self::next_coordinate(after, current, coords, scope.ppu) else { break };

                    segments.push(PathSeg::Curve { c1, c2, to });
                    current = to;
                    current_extents = (0.0, 0.0);
                    rest = after;
                    continue;
                }
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("circle") {
                let (bracket_opts, remainder) = Self::take_brackets(remainder);
                let radius = if !bracket_opts.is_empty() {
                    // circle [radius=1.5]
                    bracket_opts.split(',').find_map(|part| {
                        let (key, value) = part.split_once('=')?;
                        if key.trim() != "radius" { return None; }
                        Self::parse_unit(value)
                    })
                } else if let Some((inner, after)) = Self::take_parens(remainder) {
                    rest = after;
                    segments.push(PathSeg::Circle {
                        center: current,
                        radius: Self::parse_unit(&inner).unwrap_or(1.0),
                    });
                    continue;
                } else {
                    None
                };
                segments.push(PathSeg::Circle { center: current, radius: radius.unwrap_or(1.0) });
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("ellipse") {
                if let Some((inner, after)) = Self::take_parens(remainder) {
                    let mut parts = inner.split(" and ");
                    let rx = parts.next().and_then(Self::parse_unit).unwrap_or(1.0);
                    let ry = parts.next().and_then(Self::parse_unit).unwrap_or(rx);
                    segments.push(PathSeg::Ellipse { center: current, rx, ry });
                    rest = after;
                    continue;
                }
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("rectangle") {
                if let Some(((to, _), after)) = Self::next_coordinate(remainder, current, coords, scope.ppu) {
                    segments.push(PathSeg::Rect { from: current, to });
                    current = to;
                    current_extents = (0.0, 0.0);
                    rest = after;
                    continue;
                }
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("arc") {
                let (bracket_opts, remainder) = Self::take_brackets(remainder);
                let spec = if !bracket_opts.is_empty() {
                    Some((bracket_opts, remainder))
                } else {
                    Self::take_parens(remainder)
                };
                if let Some((inner, after)) = spec {
                    // (start:end:radius) or [start angle=, end angle=, radius=]
                    let nums: Vec<f64> = if inner.contains(':') {
                        inner.split(':').filter_map(|n| Self::parse_unit(n)).collect()
                    } else {
                        ["start angle", "end angle", "radius"].iter()
                            .filter_map(|key| inner.split(',').find_map(|part| {
                                let (k, v) = part.split_once('=')?;
                                if k.trim() != *key { return None; }
                                Self::parse_unit(v)
                            }))
                            .collect()
                    };
                    if nums.len() == 3 {
                        let (start, end, radius) = (nums[0], nums[1], nums[2]);
                        let center = (
                            current.0 - radius * start.to_radians().cos(),
                            current.1 - radius * start.to_radians().sin(),
                        );
                        let endpoint = (
                            center.0 + radius * end.to_radians().cos(),
                            center.1 + radius * end.to_radians().sin(),
                        );
                        segments.push(PathSeg::Arc { from: current, start, end, radius });
                        current = endpoint;
                        current_extents = (0.0, 0.0);
                    }
                    rest = after;
                    continue;
                }
                rest = remainder;
                continue;
            }

            if let Some(remainder) = trimmed.strip_prefix("grid") {
                if let Some(((to, _), after)) = Self::next_coordinate(remainder, current, coords, scope.ppu) {
                    let step = if options.step > 0.0 { options.step } else { 1.0 };
                    segments.push(PathSeg::Grid { from: current, to, step });
                    current = to;
                    current_extents = (0.0, 0.0);
                    rest = after;
                    continue;
                }
                rest = remainder;
                continue;
            }

            // circuitikz: (a) to[R=$R_1$] (b) — two-terminal component
            if let Some(remainder) = trimmed.strip_prefix("to") {
                if Self::token_boundary(remainder) {
                    let (component_opts, remainder) = Self::take_brackets(remainder);
                    if let Some(((point, extents), after)) =
                        Self::next_coordinate(remainder, current, coords, scope.ppu)
                    {
                        let (kind, label) = Self::component_spec(&component_opts);
                        let (start, target) = Self::shrink_segment(
                            current, current_extents, point, extents, scope.ppu,
                        );
                        segments.push(PathSeg::Component { from: start, to: target, kind, label });
                        segments.push(PathSeg::Move(target));
                        current = point;
                        current_extents = extents;
                        rest = after;
                        continue;
                    }
                    rest = remainder;
                    continue;
                }
            }

            // automata: (q0) edge [bend left] node {a} (q1)
            if let Some(remainder) = trimmed.strip_prefix("edge") {
                if Self::token_boundary(remainder) {
                    rest = Self::parse_edge(
                        scope, remainder, &merged, current, current_extents,
                        coords, &mut edges,
                    );
                    continue;
                }
            }

            // inline node: -- node[above] {label} (next) — or terminal node
            if let Some(remainder) = trimmed.strip_prefix("node") {
                if Self::token_boundary(remainder) {
                    let (node_opts, remainder) = Self::take_brackets(remainder);
                    // optional (name) before the text
                    let (name, remainder) = match Self::take_parens(remainder) {
                        Some((name, after)) if !after.trim_start().starts_with('{')
                            => (Some(name), remainder),
                        Some((name, after)) => (Some(name), after),
                        None => (None, remainder),
                    };
                    let (text, remainder) = Self::take_braces(remainder)
                        .unwrap_or((String::new(), remainder));

                    let node = Self::build_node(scope, &node_opts, current, &text);
                    if let Some(name) = name {
                        coords.insert(name.trim().to_string(), (current, (0.0, 0.0)));
                    }
                    if pending_line {
                        pending_label = Some(node); // midway label, placed on next coord
                    } else {
                        labels.push(node);
                    }
                    rest = remainder;
                    continue;
                }
            }

            // unrecognized token — advance one char to keep making progress
            rest = &trimmed[trimmed.chars().next().map(|c| c.len_utf8()).unwrap_or(1)..];
        }

        if let Some(label) = pending_label.take() {
            labels.push(label);
        }

        if !segments.is_empty() || !labels.is_empty() {
            out.push(TikzElement::Path(TikzPath { segments, stroke, fill, options, labels }));
        }
        out.extend(edges);
    }

    /// Component kind + label from `to[...]` options (R=, C=, L=, l=, ...)
    fn component_spec(options: &str) -> (String, String) {
        const KINDS: [&str; 8] = ["R", "C", "L", "battery", "V", "I", "D", "short"];
        let mut kind = String::new();
        let mut label = String::new();

        for part in Self::split_commas(options) {
            match part.split_once('=') {
                Some((key, value)) => {
                    let key = key.trim();
                    if KINDS.contains(&key) {
                        kind = key.to_string();
                        label = Self::label_text(value.trim());
                    } else if matches!(key, "l" | "l_" | "l^" | "label") {
                        label = Self::label_text(value.trim());
                    }
                }
                None => {
                    let flag = part.trim();
                    if KINDS.contains(&flag) {
                        kind = flag.to_string();
                    }
                }
            }
        }

        if kind == "short" { kind.clear(); }
        (kind, label)
    }

    /// `edge [bend left] node {a} (target)` — emitted as a separate path so
    /// the pen position of the main path is unaffected
    fn parse_edge<'a>(
        scope: &Scope,
        input: &'a str,
        path_options: &str,
        from: Point,
        from_ext: (f64, f64),
        coords: &mut HashMap<String, NamedPoint>,
        edges: &mut Vec<TikzElement>,
    ) -> &'a str {
        let (edge_opts_raw, mut rest) = Self::take_brackets(input);
        let edge_opts = Self::expand_options(scope, &edge_opts_raw, 4);

        // optional label node
        let mut label = None;
        let trimmed = rest.trim_start();
        if let Some(after) = trimmed.strip_prefix("node") {
            if Self::token_boundary(after) {
                let (node_opts, after) = Self::take_brackets(after);
                if let Some((text, after)) = Self::take_braces(after) {
                    label = Some(Self::build_node(scope, &node_opts, (0.0, 0.0), &text));
                    rest = after;
                }
            }
        }

        let Some(((target, target_ext), after)) =
            Self::next_coordinate(rest, from, coords, scope.ppu)
        else {
            return rest;
        };
        rest = after;

        let options = Self::parse_path_options(&format!("{}, {}", path_options, edge_opts));
        let (segments, apex) = Self::edge_geometry(scope, from, from_ext, target, target_ext, &edge_opts);

        let labels = match label {
            Some(mut node) => {
                node.at = apex;
                if node.placement.is_empty() {
                    node.placement = "above".to_string();
                }
                vec![node]
            }
            None => Vec::new(),
        };

        edges.push(TikzElement::Path(TikzPath {
            segments,
            stroke: true,
            fill: false,
            options,
            labels,
        }));
        rest
    }

    /// Geometry for an edge: straight, bent or a self-loop.
    /// Returns the segments plus the label anchor point.
    fn edge_geometry(
        scope: &Scope,
        from: Point,
        from_ext: (f64, f64),
        to: Point,
        to_ext: (f64, f64),
        edge_opts: &str,
    ) -> (Vec<PathSeg>, Point) {
        let mut bend = 0.0f64;
        let mut loop_dir: Option<f64> = None; // base angle in degrees

        for part in Self::split_commas(edge_opts) {
            let (key, value) = match part.split_once('=') {
                Some((key, value)) => (key.trim(), value.trim().parse::<f64>().ok()),
                None => (part.trim(), None),
            };
            match key {
                "bend left"  => bend = value.unwrap_or(30.0),
                "bend right" => bend = -value.unwrap_or(30.0),
                "loop above" | "loop" => loop_dir = Some(90.0),
                "loop below" => loop_dir = Some(270.0),
                "loop left"  => loop_dir = Some(180.0),
                "loop right" => loop_dir = Some(0.0),
                _ => {}
            }
        }

        // self-loop: leave and re-enter the node boundary around `base`
        if let Some(base) = loop_dir {
            let r = from_ext.0.max(from_ext.1).max(10.0) / scope.ppu;
            let reach = r + 0.85;
            let point_at = |angle: f64, dist: f64| -> Point {
                let rad = angle.to_radians();
                (from.0 + dist * rad.cos(), from.1 + dist * rad.sin())
            };

            let a = point_at(base + 25.0, r);
            let b = point_at(base - 25.0, r);
            let c1 = point_at(base + 40.0, reach);
            let c2 = point_at(base - 40.0, reach);
            let apex = point_at(base, reach + 0.15);

            return (vec![PathSeg::Move(a), PathSeg::Curve { c1, c2, to: b }], apex);
        }

        let (a, b) = Self::shrink_segment(from, from_ext, to, to_ext, scope.ppu);

        if bend.abs() < 1e-9 {
            let apex = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            return (vec![PathSeg::Move(a), PathSeg::Line(b)], apex);
        }

        // bent edge: rotate the departure/arrival directions by the angle
        let delta = (b.0 - a.0, b.1 - a.1);
        let length = (delta.0 * delta.0 + delta.1 * delta.1).sqrt().max(1e-9);
        let u = (delta.0 / length, delta.1 / length);

        let out_dir = Self::rotate(u, bend);
        let in_dir = Self::rotate(u, -bend);
        let c1 = (a.0 + out_dir.0 * length / 3.0, a.1 + out_dir.1 * length / 3.0);
        let c2 = (b.0 - in_dir.0 * length / 3.0, b.1 - in_dir.1 * length / 3.0);
        let apex = (
            (a.0 + 3.0 * c1.0 + 3.0 * c2.0 + b.0) / 8.0,
            (a.1 + 3.0 * c1.1 + 3.0 * c2.1 + b.1) / 8.0,
        );

        (vec![PathSeg::Move(a), PathSeg::Curve { c1, c2, to: b }], apex)
    }

    /// \node[options] (name) at (x,y) {text};
    /// also supports the positioning library: \node[right=of A] {text}
    fn parse_node_statement(
        scope: &Scope,
        input: &str,
        coords: &mut HashMap<String, NamedPoint>,
    ) -> Option<TikzNode> {
        let (options_raw, rest) = Self::take_brackets(input);

        let mut rest = rest;
        let mut name = None;
        if let Some((inner, after)) = Self::take_parens(rest) {
            name = Some(inner.trim().to_string());
            rest = after;
        }

        let mut at = None;
        let trimmed = rest.trim_start();
        if let Some(after) = trimmed.strip_prefix("at") {
            if let Some(((point, _), after)) = Self::next_coordinate(after, (0.0, 0.0), coords, scope.ppu) {
                at = Some(point);
                rest = after;
            }
        }

        let (text, _) = Self::take_braces(rest)?;
        let mut node = Self::build_node(scope, &options_raw, at.unwrap_or((0.0, 0.0)), &text);

        // positioning library: right=of A, above=1cm of A, right of=A
        if at.is_none() {
            let expanded = Self::expand_options(scope, &options_raw, 4);
            if let Some(position) = Self::positioned_at(scope, &expanded, &node, coords) {
                node.at = position;
            }
        }

        if let Some(name) = name {
            let extents = Self::node_extents(node.text.chars().count(), node.shape.as_deref());
            coords.insert(name, (node.at, extents));
        }

        Some(node)
    }

    /// Resolve `right=of A` (border to border) and `right of=A` (center to
    /// center) into an absolute position
    fn positioned_at(
        scope: &Scope,
        options: &str,
        node: &TikzNode,
        coords: &HashMap<String, NamedPoint>,
    ) -> Option<Point> {
        for part in Self::split_commas(options) {
            let Some((key, value)) = part.split_once('=') else { continue };
            let (key, value) = (key.trim(), value.trim());

            // old syntax: right of=A
            if let Some(direction) = key.strip_suffix(" of") {
                let Some(dir) = Self::direction_vector(direction) else { continue };
                let Some(&(center, _)) = coords.get(value) else { continue };
                let d = scope.node_distance;
                return Some((center.0 + dir.0 * d, center.1 + dir.1 * d));
            }

            // new syntax: right=of A / right=1.5cm of A
            let Some(dir) = Self::direction_vector(key) else { continue };
            let Some((distance, reference)) = value.rsplit_once("of ") else { continue };
            let Some(&(center, ref_ext)) = coords.get(reference.trim()) else { continue };

            let distance = Self::parse_unit(distance.trim()).unwrap_or(scope.node_distance);
            let own_ext = Self::node_extents(node.text.chars().count(), node.shape.as_deref());
            let border = (Self::boundary_shrink(ref_ext, dir)
                + Self::boundary_shrink(own_ext, dir)) / scope.ppu;

            let total = distance + border;
            return Some((center.0 + dir.0 * total, center.1 + dir.1 * total));
        }
        None
    }

    fn direction_vector(name: &str) -> Option<Point> {
        const DIAG: f64 = std::f64::consts::FRAC_1_SQRT_2;
        match name {
            "above"       => Some((0.0, 1.0)),
            "below"       => Some((0.0, -1.0)),
            "left"        => Some((-1.0, 0.0)),
            "right"       => Some((1.0, 0.0)),
            "above left"  => Some((-DIAG, DIAG)),
            "above right" => Some((DIAG, DIAG)),
            "below left"  => Some((-DIAG, -DIAG)),
            "below right" => Some((DIAG, -DIAG)),
            _ => None,
        }
    }

    /// \coordinate (name) at (x,y);
    fn parse_coordinate_statement(
        scope: &Scope,
        input: &str,
        coords: &mut HashMap<String, NamedPoint>,
    ) {
        let (_, rest) = Self::take_brackets(input);
        let Some((name, rest)) = Self::take_parens(rest) else { return };

        let mut at = (0.0, 0.0);
        if let Some(after) = rest.trim_start().strip_prefix("at") {
            if let Some(((point, _), _)) = Self::next_coordinate(after, (0.0, 0.0), coords, scope.ppu) {
                at = point;
            }
        }
        coords.insert(name.trim().to_string(), (at, (0.0, 0.0)));
    }

    fn build_node(scope: &Scope, options_raw: &str, at: Point, text: &str) -> TikzNode {
        let mut expanded = Self::expand_options(scope, options_raw, 4);
        if let Some(every) = scope.styles.get("every node") {
            expanded = format!("{}, {}", every, expanded);
        }

        let mut placement = String::new();
        let mut shape = None;
        let mut fill = None;
        let mut color = None;
        let mut draw = false;
        let mut initial = false;
        let mut accepting = false;

        for part in Self::split_commas(&expanded) {
            let part = part.trim();
            match part {
                "above" | "below" | "left" | "right"
                | "above left" | "above right"
                | "below left" | "below right" => placement = part.to_string(),
                "circle" | "rectangle" => shape = Some(part.to_string()),
                "draw" => draw = true,
                "accepting" => accepting = true,
                "midway" | "near start" | "near end" => {}
                _ if part.starts_with("initial") => initial = true,
                _ => {
                    if let Some((key, value)) = part.split_once('=') {
                        match key.trim() {
                            "fill" => fill = Some(value.trim().to_string()),
                            "draw" | "color" => { color = Some(value.trim().to_string()); draw = true; }
                            _ => {}
                        }
                    } else if !part.is_empty() && color.is_none() && Self::looks_like_color(part) {
                        color = Some(part.to_string());
                    }
                }
            }
        }

        // a filled or explicitly drawn node defaults to a rectangle shape
        if (draw || fill.is_some()) && shape.is_none() {
            shape = Some("rectangle".to_string());
        }
        if !draw && fill.is_none() {
            shape = None;
        }

        TikzNode {
            at,
            text: Self::label_text(text),
            placement,
            shape,
            fill,
            color,
            initial,
            accepting,
        }
    }

    fn looks_like_color(name: &str) -> bool {
        let base = name.split('!').next().unwrap_or(name);
        matches!(base,
            "red" | "green" | "blue" | "cyan" | "magenta" | "yellow" | "black"
            | "white" | "gray" | "grey" | "orange" | "violet" | "purple"
            | "brown" | "pink" | "teal" | "lime" | "olive" | "navy")
    }

    fn parse_path_options(raw: &str) -> TikzOptions {
        let mut options = TikzOptions { width: 1.3, ..Default::default() };

        for part in Self::split_commas(raw) {
            let part = part.trim();
            match part {
                "->" => options.arrow_end = true,
                "<-" => options.arrow_start = true,
                "<->" => { options.arrow_start = true; options.arrow_end = true; }
                "-" => { options.arrow_start = false; options.arrow_end = false; }
                "ultra thick" => options.width = 3.6,
                "very thick"  => options.width = 2.8,
                "thick"       => options.width = 2.0,
                "semithick"   => options.width = 1.6,
                "thin"        => options.width = 0.9,
                "very thin"   => options.width = 0.6,
                "ultra thin"  => options.width = 0.4,
                "dashed"          => options.dash = "6,3".into(),
                "densely dashed"  => options.dash = "4,2".into(),
                "loosely dashed"  => options.dash = "9,6".into(),
                "dotted"          => options.dash = "1.5,2.5".into(),
                "densely dotted"  => options.dash = "1,1.5".into(),
                "rounded corners" => options.rounding = 5.0,
                _ => {
                    if let Some((key, value)) = part.split_once('=') {
                        match key.trim() {
                            "fill" => options.fill_color = Some(value.trim().to_string()),
                            "draw" | "color" => options.color = Some(value.trim().to_string()),
                            "step" => options.step = Self::parse_unit(value).unwrap_or(0.0),
                            "line width" => {
                                options.width = Self::parse_unit(value).unwrap_or(0.035) * PPU;
                            }
                            "rounded corners" => {
                                options.rounding = Self::parse_unit(value).unwrap_or(0.13) * PPU;
                            }
                            _ => {}
                        }
                    } else if !part.is_empty()
                        && options.color.is_none()
                        && Self::looks_like_color(part)
                    {
                        options.color = Some(part.to_string());
                    }
                }
            }
        }

        options
    }

    /// Node/label text: strip math delimiters, map \commands to symbols,
    /// turn _1/^2 into Unicode sub/superscripts
    fn label_text(raw: &str) -> String {
        const SUB: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
        const SUP: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '$' | '{' | '}' => {}
                '~' => out.push(' '),
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '_' | '^' => {
                    // single char or {group}
                    let mut script = String::new();
                    if chars.peek() == Some(&'{') {
                        chars.next();
                        for next in chars.by_ref() {
                            if next == '}' { break; }
                            script.push(next);
                        }
                    } else if let Some(next) = chars.next() {
                        script.push(next);
                    }

                    let table = if c == '_' { &SUB } else { &SUP };
                    for ch in script.chars() {
                        match ch.to_digit(10) {
                            Some(digit) => out.push(table[digit as usize]),
                            None => out.push(ch),
                        }
                    }
                }
                '\\' => {
                    let mut word = String::new();
                    while chars.peek().is_some_and(|n| n.is_alphabetic()) {
                        word.push(chars.next().unwrap());
                    }
                    match math_symbol(&word) {
                        Some(symbol) => out.push_str(symbol),
                        None => out.push(' '),
                    }
                }
                _ => out.push(c),
            }
        }
        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    // -----------------------------------------------------------------------
    // SVG rendering
    // -----------------------------------------------------------------------

    pub fn render_svg(picture: &TikzPicture, ctx: &mut RenderContext) -> String {
        use std::fmt::Write as _;

        let ppu = PPU * picture.scale;
        let margin = 14.0;

        let (min, max) = Self::bounding_box(picture, ppu);
        let width  = ((max.0 - min.0) * ppu + 2.0 * margin).max(40.0);
        let height = ((max.1 - min.1) * ppu + 2.0 * margin).max(40.0);

        // TikZ y points up; SVG y points down
        let tx = |x: f64| (x - min.0) * ppu + margin;
        let ty = |y: f64| height - ((y - min.1) * ppu + margin);

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" \
             viewBox=\"0 0 {w:.0} {h:.0}\" font-family=\"serif\" font-size=\"13\">",
            w = width, h = height,
        );

        for element in &picture.elements {
            match element {
                TikzElement::Path(path) => {
                    Self::write_path(&mut svg, path, ppu, &tx, &ty, ctx);
                    for label in &path.labels {
                        Self::write_node(&mut svg, label, &tx, &ty, ctx);
                    }
                }
                TikzElement::Node(node) => Self::write_node(&mut svg, node, &tx, &ty, ctx),
            }
        }

        let _ = write!(svg, "</svg>");
        svg
    }

    fn bounding_box(picture: &TikzPicture, ppu: f64) -> (Point, Point) {
        let mut min = (f64::INFINITY, f64::INFINITY);
        let mut max = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        let mut grow = |p: Point, pad: f64| {
            min.0 = min.0.min(p.0 - pad);
            min.1 = min.1.min(p.1 - pad);
            max.0 = max.0.max(p.0 + pad);
            max.1 = max.1.max(p.1 + pad);
        };

        for element in &picture.elements {
            match element {
                TikzElement::Path(path) => {
                    for seg in &path.segments {
                        match seg {
                            PathSeg::Move(p) | PathSeg::Line(p) => grow(*p, 0.0),
                            PathSeg::Curve { c1, c2, to } => {
                                grow(*c1, 0.0); grow(*c2, 0.0); grow(*to, 0.0);
                            }
                            PathSeg::Circle { center, radius } => grow(*center, *radius),
                            PathSeg::Ellipse { center, rx, ry } => grow(*center, rx.max(*ry)),
                            PathSeg::Rect { from, to } | PathSeg::Grid { from, to, .. } => {
                                grow(*from, 0.0); grow(*to, 0.0);
                            }
                            PathSeg::Component { from, to, .. } => {
                                // room for the glyph plus its label above
                                grow(*from, 0.65); grow(*to, 0.65);
                            }
                            PathSeg::Arc { from, start, end, radius } => {
                                let center = (
                                    from.0 - radius * start.to_radians().cos(),
                                    from.1 - radius * start.to_radians().sin(),
                                );
                                grow(center, *radius);
                                let _ = end;
                            }
                            PathSeg::Close => {}
                        }
                    }
                    for label in &path.labels {
                        grow(label.at, 0.4);
                    }
                }
                TikzElement::Node(node) => {
                    let (hw, hh) = Self::node_extents(
                        node.text.chars().count(), node.shape.as_deref(),
                    );
                    let extra = if node.initial { 30.0 } else { 4.0 };
                    grow(node.at, (hw.max(hh) + extra) / ppu);
                }
            }
        }

        if !min.0.is_finite() {
            return ((0.0, 0.0), (1.0, 1.0));
        }
        (min, max)
    }

    /// "blue!30" → 30% blue + 70% white; "red!50!black" → mix with black
    fn resolve_color(name: &str, ctx: &RenderContext) -> String {
        let mut parts = name.split('!');
        let base = ctx.resolve_color(parts.next().unwrap_or(name).trim());

        let Some(percent) = parts.next().and_then(|p| p.trim().parse::<f64>().ok()) else {
            return base;
        };
        let other = ctx.resolve_color(parts.next().unwrap_or("white").trim());

        let (Some(a), Some(b)) = (Self::hex_rgb(&base), Self::hex_rgb(&other)) else {
            return base;
        };
        let t = (percent / 100.0).clamp(0.0, 1.0);
        format!(
            "#{:02x}{:02x}{:02x}",
            (a.0 as f64 * t + b.0 as f64 * (1.0 - t)) as u8,
            (a.1 as f64 * t + b.1 as f64 * (1.0 - t)) as u8,
            (a.2 as f64 * t + b.2 as f64 * (1.0 - t)) as u8,
        )
    }

    fn hex_rgb(color: &str) -> Option<(u8, u8, u8)> {
        let hex = color.strip_prefix('#')?;
        if hex.len() != 6 { return None; }
        Some((
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        ))
    }

    /// Polyline with rounded corners: cut each corner and bridge it with a
    /// quadratic curve
    fn rounded_polyline_d(points: &[Point], closed: bool, radius: f64) -> String {
        use std::fmt::Write as _;

        if points.len() < 3 {
            let mut d = String::new();
            for (i, p) in points.iter().enumerate() {
                let _ = write!(d, "{}{:.1} {:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1);
            }
            return d;
        }

        let corner = |prev: Point, at: Point, next: Point, d: &mut String| {
            let v_in = (at.0 - prev.0, at.1 - prev.1);
            let v_out = (next.0 - at.0, next.1 - at.1);
            let len_in = (v_in.0 * v_in.0 + v_in.1 * v_in.1).sqrt().max(1e-9);
            let len_out = (v_out.0 * v_out.0 + v_out.1 * v_out.1).sqrt().max(1e-9);
            let cut = radius.min(len_in / 2.0).min(len_out / 2.0);

            let entry = (at.0 - v_in.0 / len_in * cut, at.1 - v_in.1 / len_in * cut);
            let exit = (at.0 + v_out.0 / len_out * cut, at.1 + v_out.1 / len_out * cut);
            let _ = write!(d, "L{:.1} {:.1} Q{:.1} {:.1} {:.1} {:.1} ",
                entry.0, entry.1, at.0, at.1, exit.0, exit.1);
        };

        let mut d = String::new();
        if closed {
            let start = (
                (points[points.len() - 1].0 + points[0].0) / 2.0,
                (points[points.len() - 1].1 + points[0].1) / 2.0,
            );
            let _ = write!(d, "M{:.1} {:.1} ", start.0, start.1);
            let n = points.len();
            for i in 0..n {
                let prev = points[(i + n - 1) % n];
                let next = points[(i + 1) % n];
                corner(prev, points[i], next, &mut d);
            }
            d.push_str("Z ");
        } else {
            let _ = write!(d, "M{:.1} {:.1} ", points[0].0, points[0].1);
            for i in 1..points.len() - 1 {
                corner(points[i - 1], points[i], points[i + 1], &mut d);
            }
            let last = points[points.len() - 1];
            let _ = write!(d, "L{:.1} {:.1} ", last.0, last.1);
        }
        d
    }

    fn write_path(
        svg: &mut String,
        path: &TikzPath,
        ppu: f64,
        tx: &dyn Fn(f64) -> f64,
        ty: &dyn Fn(f64) -> f64,
        ctx: &mut RenderContext,
    ) {
        use std::fmt::Write as _;

        let stroke_color = path.options.color.as_deref()
            .map(|c| Self::resolve_color(c, ctx))
            .unwrap_or_else(|| "#000000".to_string());
        let fill_color = if path.fill {
            path.options.fill_color.as_deref()
                .or(path.options.color.as_deref())
                .map(|c| Self::resolve_color(c, ctx))
                .unwrap_or_else(|| "#000000".to_string())
        } else {
            "none".to_string()
        };

        // rounded corners apply to pure polyline paths
        let rounding = path.options.rounding;
        let pure_polyline = rounding > 0.0 && path.segments.iter().all(|seg|
            matches!(seg, PathSeg::Move(_) | PathSeg::Line(_) | PathSeg::Close));

        let mut d = String::new();
        let mut components: Vec<(Point, Point, &str, &str)> = Vec::new();

        if pure_polyline {
            let mut points: Vec<Point> = Vec::new();
            let mut closed = false;
            for seg in &path.segments {
                match seg {
                    PathSeg::Move(p) => {
                        if points.len() > 1 {
                            d.push_str(&Self::rounded_polyline_d(&points, closed, rounding));
                        }
                        points = vec![(tx(p.0), ty(p.1))];
                        closed = false;
                    }
                    PathSeg::Line(p) => points.push((tx(p.0), ty(p.1))),
                    PathSeg::Close => closed = true,
                    _ => unreachable!(),
                }
            }
            if points.len() > 1 {
                d.push_str(&Self::rounded_polyline_d(&points, closed, rounding));
            }
        } else {
            for seg in &path.segments {
                match seg {
                    PathSeg::Move(p) =>
                        { let _ = write!(d, "M{:.1} {:.1} ", tx(p.0), ty(p.1)); }
                    PathSeg::Line(p) =>
                        { let _ = write!(d, "L{:.1} {:.1} ", tx(p.0), ty(p.1)); }
                    PathSeg::Curve { c1, c2, to } => {
                        let _ = write!(d, "C{:.1} {:.1} {:.1} {:.1} {:.1} {:.1} ",
                            tx(c1.0), ty(c1.1), tx(c2.0), ty(c2.1), tx(to.0), ty(to.1));
                    }
                    PathSeg::Circle { center, radius } => {
                        let (r, cx, cy) = (radius * ppu, tx(center.0), ty(center.1));
                        let _ = write!(d,
                            "M{:.1} {:.1} A{r:.1} {r:.1} 0 1 0 {:.1} {:.1} A{r:.1} {r:.1} 0 1 0 {:.1} {:.1} ",
                            cx + r, cy, cx - r, cy, cx + r, cy, r = r);
                    }
                    PathSeg::Ellipse { center, rx, ry } => {
                        let (rx, ry) = (rx * ppu, ry * ppu);
                        let (cx, cy) = (tx(center.0), ty(center.1));
                        let _ = write!(d,
                            "M{:.1} {:.1} A{rx:.1} {ry:.1} 0 1 0 {:.1} {:.1} A{rx:.1} {ry:.1} 0 1 0 {:.1} {:.1} ",
                            cx + rx, cy, cx - rx, cy, cx + rx, cy, rx = rx, ry = ry);
                    }
                    PathSeg::Rect { from, to } => {
                        if rounding > 0.0 {
                            let (x0, x1) = (tx(from.0).min(tx(to.0)), tx(from.0).max(tx(to.0)));
                            let (y0, y1) = (ty(from.1).min(ty(to.1)), ty(from.1).max(ty(to.1)));
                            let r = rounding.min((x1 - x0) / 2.0).min((y1 - y0) / 2.0);
                            let _ = write!(d,
                                "M{:.1} {y0:.1} L{:.1} {y0:.1} Q{x1:.1} {y0:.1} {x1:.1} {:.1} \
                                 L{x1:.1} {:.1} Q{x1:.1} {y1:.1} {:.1} {y1:.1} \
                                 L{:.1} {y1:.1} Q{x0:.1} {y1:.1} {x0:.1} {:.1} \
                                 L{x0:.1} {:.1} Q{x0:.1} {y0:.1} {:.1} {y0:.1} Z ",
                                x0 + r, x1 - r, y0 + r, y1 - r, x1 - r, x0 + r,
                                y1 - r, y0 + r, x0 + r,
                                x0 = x0, x1 = x1, y0 = y0, y1 = y1);
                        } else {
                            let _ = write!(d, "M{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} Z ",
                                tx(from.0), ty(from.1), tx(to.0), ty(from.1),
                                tx(to.0), ty(to.1), tx(from.0), ty(to.1));
                        }
                    }
                    PathSeg::Arc { from, start, end, radius } => {
                        let center = (
                            from.0 - radius * start.to_radians().cos(),
                            from.1 - radius * start.to_radians().sin(),
                        );
                        let endpoint = (
                            center.0 + radius * end.to_radians().cos(),
                            center.1 + radius * end.to_radians().sin(),
                        );
                        let r = radius * ppu;
                        let large = if (end - start).abs() > 180.0 { 1 } else { 0 };
                        // counterclockwise in TikZ becomes clockwise on screen
                        let sweep = if end > start { 0 } else { 1 };
                        let _ = write!(d, "A{r:.1} {r:.1} 0 {} {} {:.1} {:.1} ",
                            large, sweep, tx(endpoint.0), ty(endpoint.1), r = r);
                    }
                    PathSeg::Grid { from, to, step } => {
                        let (x0, x1) = (from.0.min(to.0), from.0.max(to.0));
                        let (y0, y1) = (from.1.min(to.1), from.1.max(to.1));
                        let mut x = x0;
                        while x <= x1 + 1e-9 {
                            let _ = write!(d, "M{:.1} {:.1} L{:.1} {:.1} ", tx(x), ty(y0), tx(x), ty(y1));
                            x += step;
                        }
                        let mut y = y0;
                        while y <= y1 + 1e-9 {
                            let _ = write!(d, "M{:.1} {:.1} L{:.1} {:.1} ", tx(x0), ty(y), tx(x1), ty(y));
                            y += step;
                        }
                    }
                    PathSeg::Component { from, to, kind, label } => {
                        components.push((*from, *to, kind.as_str(), label.as_str()));
                    }
                    PathSeg::Close => d.push_str("Z "),
                }
            }
        }

        let mut markers = String::new();
        let mut marker_attrs = String::new();
        if path.options.arrow_end || path.options.arrow_start {
            ctx.phantom_id += 1;
            let id = format!("tikz-arrow-{}", ctx.phantom_id);
            let _ = write!(
                markers,
                "<defs><marker id=\"{id}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" \
                 markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\">\
                 <path d=\"M0 0 L10 5 L0 10 Z\" fill=\"{}\"/></marker></defs>",
                stroke_color, id = id,
            );
            if path.options.arrow_end {
                let _ = write!(marker_attrs, " marker-end=\"url(#{})\"", id);
            }
            if path.options.arrow_start {
                let _ = write!(marker_attrs, " marker-start=\"url(#{})\"", id);
            }
        }

        let dash = if path.options.dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{}\"", path.options.dash)
        };
        let stroke = if path.stroke { stroke_color.clone() } else { "none".to_string() };

        // a path holding only Move commands (e.g. "\path (q0) edge ...")
        // draws nothing — emitting it would still paint stray arrow markers
        let drawable = d.chars().any(|c| matches!(c, 'L' | 'C' | 'A' | 'Q' | 'Z'));
        if drawable {
            let _ = write!(
                svg,
                "{}<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{}{}/>",
                markers, d.trim_end(), fill_color, stroke, path.options.width, dash, marker_attrs,
            );
        }

        for (from, to, kind, label) in components {
            Self::write_component(svg, from, to, kind, label, &stroke_color, path.options.width, tx, ty);
        }
    }

    /// Two-terminal circuit component: wire — glyph — wire, with a label
    fn write_component(
        svg: &mut String,
        from: Point,
        to: Point,
        kind: &str,
        label: &str,
        color: &str,
        width: f64,
        tx: &dyn Fn(f64) -> f64,
        ty: &dyn Fn(f64) -> f64,
    ) {
        use std::fmt::Write as _;

        let a = (tx(from.0), ty(from.1));
        let b = (tx(to.0), ty(to.1));
        let delta = (b.0 - a.0, b.1 - a.1);
        let length = (delta.0 * delta.0 + delta.1 * delta.1).sqrt().max(1e-9);
        let u = (delta.0 / length, delta.1 / length);
        // perpendicular pointing screen-up, so labels sit above the component
        let mut perp = (-u.1, u.0);
        if perp.1 > 0.0 {
            perp = (-perp.0, -perp.1);
        }
        let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);

        // plain wire
        if kind.is_empty() {
            let _ = write!(svg,
                "<path d=\"M{:.1} {:.1} L{:.1} {:.1}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>",
                a.0, a.1, b.0, b.1, color, width);
            if !label.is_empty() {
                let _ = write!(svg,
                    "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                    mid.0 + perp.0 * 12.0, mid.1 + perp.1 * 12.0 - 4.0, label);
            }
            return;
        }

        let glyph = (length * 0.5).min(34.0);
        let g0 = (mid.0 - u.0 * glyph / 2.0, mid.1 - u.1 * glyph / 2.0);
        let g1 = (mid.0 + u.0 * glyph / 2.0, mid.1 + u.1 * glyph / 2.0);

        // lead wires
        let _ = write!(svg,
            "<path d=\"M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1}\" \
             stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>",
            a.0, a.1, g0.0, g0.1, g1.0, g1.1, b.0, b.1, color, width);

        let mut d = String::new();
        match kind {
            // zigzag resistor
            "R" => {
                let amp = 5.5;
                let _ = write!(d, "M{:.1} {:.1} ", g0.0, g0.1);
                for i in 0..6 {
                    let t = (i as f64 + 0.5) / 6.0;
                    let side = if i % 2 == 0 { 1.0 } else { -1.0 };
                    let p = (
                        g0.0 + u.0 * glyph * t + perp.0 * amp * side,
                        g0.1 + u.1 * glyph * t + perp.1 * amp * side,
                    );
                    let _ = write!(d, "L{:.1} {:.1} ", p.0, p.1);
                }
                let _ = write!(d, "L{:.1} {:.1}", g1.0, g1.1);
            }
            // inductor bumps
            "L" => {
                let bumps = 4;
                let step = glyph / bumps as f64;
                let _ = write!(d, "M{:.1} {:.1} ", g0.0, g0.1);
                for i in 0..bumps {
                    let s = (g0.0 + u.0 * step * i as f64, g0.1 + u.1 * step * i as f64);
                    let e = (s.0 + u.0 * step, s.1 + u.1 * step);
                    let c1 = (s.0 + perp.0 * 9.0, s.1 + perp.1 * 9.0);
                    let c2 = (e.0 + perp.0 * 9.0, e.1 + perp.1 * 9.0);
                    let _ = write!(d, "C{:.1} {:.1} {:.1} {:.1} {:.1} {:.1} ",
                        c1.0, c1.1, c2.0, c2.1, e.0, e.1);
                }
            }
            // capacitor: two parallel plates
            "C" => {
                let gap = 3.5;
                let plate = 9.0;
                let p0 = (mid.0 - u.0 * gap, mid.1 - u.1 * gap);
                let p1 = (mid.0 + u.0 * gap, mid.1 + u.1 * gap);
                let _ = write!(d,
                    "M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1} \
                     M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1}",
                    g0.0, g0.1, p0.0, p0.1,
                    p0.0 + perp.0 * plate, p0.1 + perp.1 * plate,
                    p0.0 - perp.0 * plate, p0.1 - perp.1 * plate,
                    p1.0 + perp.0 * plate, p1.1 + perp.1 * plate,
                    p1.0 - perp.0 * plate, p1.1 - perp.1 * plate,
                    p1.0, p1.1, g1.0, g1.1);
            }
            // battery: long and short plates
            "battery" => {
                let gap = 3.0;
                let p0 = (mid.0 - u.0 * gap, mid.1 - u.1 * gap);
                let p1 = (mid.0 + u.0 * gap, mid.1 + u.1 * gap);
                let _ = write!(d,
                    "M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1} \
                     M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1}",
                    g0.0, g0.1, p0.0, p0.1,
                    p0.0 + perp.0 * 11.0, p0.1 + perp.1 * 11.0,
                    p0.0 - perp.0 * 11.0, p0.1 - perp.1 * 11.0,
                    p1.0 + perp.0 * 5.0, p1.1 + perp.1 * 5.0,
                    p1.0 - perp.0 * 5.0, p1.1 - perp.1 * 5.0,
                    p1.0, p1.1, g1.0, g1.1);
            }
            // sources and diodes: circle with the kind letter
            _ => {
                let r = glyph / 2.0;
                let _ = write!(svg,
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>\
                     <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>",
                    mid.0, mid.1, r, color, width, mid.0, mid.1 + 3.5, kind);
            }
        }

        if !d.is_empty() {
            let _ = write!(svg,
                "<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>",
                d.trim_end(), color, width);
        }

        if !label.is_empty() {
            let _ = write!(svg,
                "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                mid.0 + perp.0 * 16.0, mid.1 + perp.1 * 16.0 - 4.0, label);
        }
    }

    fn write_node(
        svg: &mut String,
        node: &TikzNode,
        tx: &dyn Fn(f64) -> f64,
        ty: &dyn Fn(f64) -> f64,
        ctx: &mut RenderContext,
    ) {
        use std::fmt::Write as _;

        let (x, y) = (tx(node.at.0), ty(node.at.1));
        let (half_w, half_h) = Self::node_extents(node.text.chars().count(), node.shape.as_deref());

        if let Some(shape) = &node.shape {
            let fill = node.fill.as_deref()
                .map(|c| Self::resolve_color(c, ctx))
                .unwrap_or_else(|| "none".to_string());
            let stroke = node.color.as_deref()
                .map(|c| Self::resolve_color(c, ctx))
                .unwrap_or_else(|| "#000000".to_string());

            if shape == "circle" {
                let _ = write!(svg,
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x, y, half_w, fill, stroke);
                // automata: accepting states get a double border
                if node.accepting {
                    let _ = write!(svg,
                        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>",
                        x, y, half_w - 3.5, stroke);
                }
            } else {
                let _ = write!(svg,
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                     fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x - half_w, y - half_h, half_w * 2.0, half_h * 2.0, fill, stroke);
            }

            // automata: initial states get a short incoming arrow
            if node.initial {
                ctx.phantom_id += 1;
                let id = format!("tikz-arrow-{}", ctx.phantom_id);
                let _ = write!(svg,
                    "<defs><marker id=\"{id}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" \
                     markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\">\
                     <path d=\"M0 0 L10 5 L0 10 Z\" fill=\"{stroke}\"/></marker></defs>\
                     <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                     stroke=\"{stroke}\" stroke-width=\"1.3\" marker-end=\"url(#{id})\"/>",
                    x - half_w - 26.0, y, x - half_w - 2.0, y,
                    id = id, stroke = stroke);
            }
        }

        if node.text.is_empty() {
            return;
        }

        let (dx, dy, anchor) = match node.placement.as_str() {
            "above"       => (0.0, -7.0, "middle"),
            "below"       => (0.0, 15.0, "middle"),
            "left"        => (-7.0, 4.5, "end"),
            "right"       => (7.0, 4.5, "start"),
            "above left"  => (-5.0, -7.0, "end"),
            "above right" => (5.0, -7.0, "start"),
            "below left"  => (-5.0, 15.0, "end"),
            "below right" => (5.0, 15.0, "start"),
            _             => (0.0, 4.5, "middle"),
        };
        let color = node.color.as_deref()
            .map(|c| Self::resolve_color(c, ctx))
            .unwrap_or_else(|| "#000000".to_string());

        let _ = write!(svg,
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" fill=\"{}\">{}</text>",
            x + dx, y + dy, anchor, color, node.text);
    }

}
