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
    /// \begin{tikzpicture}[...]. Returns None when nothing drawable is found.
    pub fn parse(raw: &str, env_options: Option<&str>) -> Option<TikzPicture> {
        let scale = env_options
            .and_then(|opts| opts.split(',').find_map(|part| {
                let (key, value) = part.split_once('=')?;
                if key.trim() != "scale" { return None; }
                value.trim().parse::<f64>().ok()
            }))
            .unwrap_or(1.0);

        let cleaned = Self::strip_comments(raw);
        let ppu = PPU * scale;
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
                    if let Some(path) = Self::parse_path(rest, stroke, false, ppu, &mut coords) {
                        elements.push(TikzElement::Path(path));
                    }
                }
                "fill" | "shade" => {
                    if let Some(path) = Self::parse_path(rest, false, true, ppu, &mut coords) {
                        elements.push(TikzElement::Path(path));
                    }
                }
                "filldraw" | "shadedraw" => {
                    if let Some(path) = Self::parse_path(rest, true, true, ppu, &mut coords) {
                        elements.push(TikzElement::Path(path));
                    }
                }
                "node" => {
                    if let Some(node) = Self::parse_node_statement(rest, &mut coords) {
                        elements.push(TikzElement::Node(node));
                    }
                }
                "coordinate" => {
                    Self::parse_coordinate_statement(rest, &mut coords);
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

    /// "1.5cm" / "30:2" (polar) / "name" / "name.anchor" → point in TikZ
    /// units plus the half-extents of the referenced node (0 for literals)
    fn resolve_coordinate(raw: &str, coords: &HashMap<String, NamedPoint>) -> Option<NamedPoint> {
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

        // named coordinate; anchors like A.north collapse to the center
        let name = raw.split('.').next().unwrap_or(raw).trim();
        coords.get(name).copied()
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
        let (mut point, extents) = Self::resolve_coordinate(&inner, coords)?;
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

    fn parse_path(
        input: &str,
        stroke: bool,
        fill: bool,
        ppu: f64,
        coords: &mut HashMap<String, NamedPoint>,
    ) -> Option<TikzPath> {
        let (options_raw, mut rest) = Self::take_brackets(input);
        let options = Self::parse_path_options(&options_raw);
        let fill = fill || options.fill_color.is_some();

        let mut segments: Vec<PathSeg> = Vec::new();
        let mut labels: Vec<TikzNode> = Vec::new();
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
                if let Some(((point, extents), remainder)) = Self::next_coordinate(trimmed, current, coords) {
                    if segments.is_empty() {
                        segments.push(PathSeg::Move(point));
                    } else if pending_line {
                        // pull both endpoints back to the node boundaries so
                        // edges between named nodes stop at their borders
                        let delta = (point.0 - current.0, point.1 - current.1);
                        let length = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
                        let mut target = point;
                        if length > 1e-9 {
                            let u = (delta.0 / length, delta.1 / length);
                            let s0 = Self::boundary_shrink(current_extents, u) / ppu;
                            let s1 = Self::boundary_shrink(extents, u) / ppu;
                            if s0 + s1 < length {
                                if s0 > 0.0 {
                                    let start = (current.0 + u.0 * s0, current.1 + u.1 * s0);
                                    if let Some(PathSeg::Move(p) | PathSeg::Line(p)) = segments.last_mut() {
                                        if *p == current { *p = start; }
                                    }
                                }
                                target = (point.0 - u.0 * s1, point.1 - u.1 * s1);
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
                    let Some(((c1, _), after)) = Self::next_coordinate(after, current, coords) else { break };
                    let after = after.trim_start();
                    let (c2, after) = if let Some(after) = after.strip_prefix("and") {
                        match Self::next_coordinate(after, current, coords) {
                            Some(((point, _), after)) => (point, after),
                            None => break,
                        }
                    } else {
                        (c1, after)
                    };
                    let after = after.trim_start();
                    let Some(after) = after.strip_prefix("..") else { break };
                    let Some(((to, _), after)) = Self::next_coordinate(after, current, coords) else { break };

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
                if let Some(((to, _), after)) = Self::next_coordinate(remainder, current, coords) {
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
                    Self::take_parens(remainder).map(|(inner, after)| (inner, after))
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
                if let Some(((to, _), after)) = Self::next_coordinate(remainder, current, coords) {
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

            // inline node: -- node[above] {label} (next) — or terminal node
            if let Some(remainder) = trimmed.strip_prefix("node") {
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

                let node = Self::build_node(&node_opts, current, &text);
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

            // unrecognized token — advance one char to keep making progress
            rest = &trimmed[trimmed.chars().next().map(|c| c.len_utf8()).unwrap_or(1)..];
        }

        if let Some(label) = pending_label.take() {
            labels.push(label);
        }
        if segments.is_empty() && labels.is_empty() {
            return None;
        }

        Some(TikzPath { segments, stroke, fill, options, labels })
    }

    /// \node[options] (name) at (x,y) {text};
    fn parse_node_statement(input: &str, coords: &mut HashMap<String, NamedPoint>) -> Option<TikzNode> {
        let (options_raw, rest) = Self::take_brackets(input);

        let mut rest = rest;
        let mut name = None;
        if let Some((inner, after)) = Self::take_parens(rest) {
            name = Some(inner.trim().to_string());
            rest = after;
        }

        let mut at = (0.0, 0.0);
        let trimmed = rest.trim_start();
        if let Some(after) = trimmed.strip_prefix("at") {
            if let Some(((point, _), after)) = Self::next_coordinate(after, (0.0, 0.0), coords) {
                at = point;
                rest = after;
            }
        }

        let (text, _) = Self::take_braces(rest)?;
        let node = Self::build_node(&options_raw, at, &text);
        if let Some(name) = name {
            let extents = Self::node_extents(node.text.chars().count(), node.shape.as_deref());
            coords.insert(name, (at, extents));
        }

        Some(node)
    }

    /// \coordinate (name) at (x,y);
    fn parse_coordinate_statement(input: &str, coords: &mut HashMap<String, NamedPoint>) {
        let (_, rest) = Self::take_brackets(input);
        let Some((name, rest)) = Self::take_parens(rest) else { return };

        let mut at = (0.0, 0.0);
        if let Some(after) = rest.trim_start().strip_prefix("at") {
            if let Some(((point, _), _)) = Self::next_coordinate(after, (0.0, 0.0), coords) {
                at = point;
            }
        }
        coords.insert(name.trim().to_string(), (at, (0.0, 0.0)));
    }

    fn build_node(options_raw: &str, at: Point, text: &str) -> TikzNode {
        let mut placement = String::new();
        let mut shape = None;
        let mut fill = None;
        let mut color = None;
        let mut draw = false;

        for part in options_raw.split(',') {
            let part = part.trim();
            match part {
                "above" | "below" | "left" | "right"
                | "above left" | "above right"
                | "below left" | "below right" => placement = part.to_string(),
                "circle" | "rectangle" => shape = Some(part.to_string()),
                "draw" => draw = true,
                "midway" | "near start" | "near end" => {}
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

        // arrow specs: ->, <-, <->
        for part in raw.split(',') {
            let part = part.trim();
            match part {
                "->" => options.arrow_end = true,
                "<-" => options.arrow_start = true,
                "<->" => { options.arrow_start = true; options.arrow_end = true; }
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
                "rounded corners" => {}
                _ => {
                    if let Some((key, value)) = part.split_once('=') {
                        match key.trim() {
                            "fill" => options.fill_color = Some(value.trim().to_string()),
                            "draw" | "color" => options.color = Some(value.trim().to_string()),
                            "step" => options.step = Self::parse_unit(value).unwrap_or(0.0),
                            "line width" => {
                                options.width = Self::parse_unit(value).unwrap_or(0.035) * PPU;
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

    /// Node/label text: strip math delimiters and TeX control words
    fn label_text(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '$' | '{' | '}' => {}
                '~' => out.push(' '),
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
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

        let (min, max) = Self::bounding_box(picture);
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

    fn bounding_box(picture: &TikzPicture) -> (Point, Point) {
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
                    let pad = 0.2 + node.text.chars().count() as f64 * 0.09;
                    grow(node.at, pad);
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

        let mut d = String::new();
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
                    let _ = write!(d, "M{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} Z ",
                        tx(from.0), ty(from.1), tx(to.0), ty(from.1),
                        tx(to.0), ty(to.1), tx(from.0), ty(to.1));
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
                PathSeg::Close => d.push_str("Z "),
            }
        }
        if d.is_empty() {
            return;
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
        let stroke = if path.stroke { stroke_color } else { "none".to_string() };

        let _ = write!(
            svg,
            "{}<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{}{}/>",
            markers, d.trim_end(), fill_color, stroke, path.options.width, dash, marker_attrs,
        );
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
            } else {
                let _ = write!(svg,
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                     fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x - half_w, y - half_h, half_w * 2.0, half_h * 2.0, fill, stroke);
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
