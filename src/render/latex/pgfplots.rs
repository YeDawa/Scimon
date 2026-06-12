use std::collections::HashMap;

use crate::render::latex::context::RenderContext;

// ---------------------------------------------------------------------------
// Parsed pgfplots structures
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PgfAxis {
    // axis options: title, xlabel, xmin, grid, legend pos, ...
    pub options: HashMap<String, String>,
    // bare axis flags without a value (e.g. "grid")
    pub flags:   Vec<String>,
    pub plots:   Vec<PgfPlot>,
    // legend entries from \legend{...} or \addlegendentry, in plot order
    pub legend:  Vec<String>,
    pub x_log:   bool,
    pub y_log:   bool,
}

#[derive(Debug, Clone)]
pub struct PgfPlot {
    pub options: HashMap<String, String>,
    pub flags:   Vec<String>,
    // true for bare \addplot (no [options]) — uses the default cycle marks
    pub from_cycle: bool,
    pub data:    PlotData,
}

#[derive(Debug, Clone)]
pub enum PlotData {
    // \addplot {expression} — sampled over the domain at render time
    Expression(String),
    // \addplot coordinates {...} or table {...}, already resolved
    Points(Vec<(f64, f64)>),
}

impl PgfPlot {

    fn flag(&self, name: &str) -> bool {
        self.flags.iter().any(|f| f == name)
    }

    fn option<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.options.get(name).map(|s| s.as_str())
    }

}

// ---------------------------------------------------------------------------
// pgfplots: tikzpicture parsing + SVG rendering
// ---------------------------------------------------------------------------
pub struct Pgfplots;

impl Pgfplots {

    // pgfplots default cycle list: (color, mark) per plot index
    const CYCLE: &'static [(&'static str, &'static str)] = &[
        ("#0000ff", "*"),
        ("#ff0000", "square*"),
        ("#8b4513", "triangle*"),
        ("#000000", "diamond*"),
        ("#008080", "x"),
        ("#ff8800", "+"),
    ];

    // -----------------------------------------------------------------------
    // Parsing
    // -----------------------------------------------------------------------

    // Extract every axis environment from a tikzpicture body.
    // Returns an empty Vec when the picture contains no pgfplots axis.
    pub fn parse(raw: &str) -> Vec<PgfAxis> {
        let mut axes = Vec::new();

        for env in ["axis", "semilogxaxis", "semilogyaxis", "loglogaxis"] {
            let begin = format!("\\begin{{{}}}", env);
            let end = format!("\\end{{{}}}", env);
            let mut rest = raw;

            while let Some(start) = rest.find(&begin) {
                let after = &rest[start + begin.len()..];
                let Some(stop) = after.find(&end) else { break };
                let body = &after[..stop];
                rest = &after[stop + end.len()..];

                let (options_raw, body) = Self::take_bracket_group(body);
                let (options, flags) = Self::parse_options(&options_raw);

                let x_log = env == "semilogxaxis" || env == "loglogaxis"
                    || options.get("xmode").map(|m| m == "log").unwrap_or(false);
                let y_log = env == "semilogyaxis" || env == "loglogaxis"
                    || options.get("ymode").map(|m| m == "log").unwrap_or(false);

                let mut axis = PgfAxis {
                    options, flags,
                    plots: Vec::new(),
                    legend: Vec::new(),
                    x_log, y_log,
                };
                Self::parse_axis_body(body, &mut axis);

                if let Some(entries) = axis.options.get("legend entries").cloned() {
                    if axis.legend.is_empty() {
                        axis.legend = Self::split_commas(&entries)
                            .into_iter()
                            .map(|e| Self::strip_braces(&e))
                            .collect();
                    }
                }
                axes.push(axis);
            }
        }

        axes
    }

    // Leading [options] group, if present. Returns (options, remainder).
    fn take_bracket_group(body: &str) -> (String, &str) {
        let trimmed = body.trim_start();
        if !trimmed.starts_with('[') {
            return (String::new(), body);
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
        (String::new(), body)
    }

    // Leading {balanced} group, if present. Returns (content, remainder).
    fn take_brace_group(body: &str) -> Option<(String, &str)> {
        let trimmed = body.trim_start();
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

    // Split on top-level commas, ignoring commas inside {} or [].
    fn split_commas(s: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut depth = 0usize;
        let mut current = String::new();

        for c in s.chars() {
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

    fn strip_braces(s: &str) -> String {
        s.trim().trim_start_matches('{').trim_end_matches('}').trim().to_string()
    }

    // key=value pairs and bare flags from an option list
    fn parse_options(raw: &str) -> (HashMap<String, String>, Vec<String>) {
        let mut options = HashMap::new();
        let mut flags = Vec::new();

        for part in Self::split_commas(raw) {
            // split on the first top-level '='
            let mut depth = 0usize;
            let mut split_at = None;
            for (i, c) in part.char_indices() {
                match c {
                    '{' | '[' => depth += 1,
                    '}' | ']' => depth = depth.saturating_sub(1),
                    '=' if depth == 0 => { split_at = Some(i); break; }
                    _ => {}
                }
            }

            match split_at {
                Some(i) => {
                    let key = part[..i].trim().to_lowercase();
                    let value = Self::strip_braces(&part[i + 1..]);
                    options.insert(key, value);
                }
                None => flags.push(part.trim().to_string()),
            }
        }

        (options, flags)
    }

    // Walk the axis body collecting \addplot, \legend and \addlegendentry.
    fn parse_axis_body(body: &str, axis: &mut PgfAxis) {
        let mut rest = body;

        while let Some(pos) = rest.find('\\') {
            rest = &rest[pos + 1..];

            if let Some(after) = rest.strip_prefix("addlegendentry") {
                if let Some((entry, remainder)) = Self::take_brace_group(after) {
                    axis.legend.push(entry);
                    rest = remainder;
                }
            } else if let Some(after) = rest.strip_prefix("addplot") {
                // \addplot3 (surfaces) is unsupported — skip the statement
                if after.starts_with('3') {
                    if let Some(end) = after.find(';') { rest = &after[end + 1..]; }
                    continue;
                }
                let after = after.strip_prefix('+').unwrap_or(after);
                rest = Self::parse_addplot(after, axis);
            } else if let Some(after) = rest.strip_prefix("legend") {
                // \legend{A, B, C} — but not \legendstyle or similar
                if let Some((entries, remainder)) = Self::take_brace_group(after) {
                    axis.legend = Self::split_commas(&entries)
                        .into_iter()
                        .map(|e| Self::strip_braces(&e))
                        .collect();
                    rest = remainder;
                }
            }
        }
    }

    // Parse one \addplot statement starting after the command name.
    // Returns the remainder after the closing ';'.
    fn parse_addplot<'a>(input: &'a str, axis: &mut PgfAxis) -> &'a str {
        let has_options = input.trim_start().starts_with('[');
        let (options_raw, rest) = Self::take_bracket_group(input);
        let (options, flags) = Self::parse_options(&options_raw);
        let trimmed = rest.trim_start();

        let (data, rest) = if let Some(after) = trimmed.strip_prefix("coordinates") {
            match Self::take_brace_group(after) {
                Some((coords, remainder)) => (Some(Self::parse_coordinates(&coords)), remainder),
                None => (None, after),
            }
        } else if let Some(after) = trimmed.strip_prefix("table") {
            let (table_opts, after) = Self::take_bracket_group(after);
            let (table_options, _) = Self::parse_options(&table_opts);
            match Self::take_brace_group(after) {
                Some((rows, remainder)) => (Some(Self::parse_table(&rows, &table_options)), remainder),
                None => (None, after),
            }
        } else if let Some((expr, remainder)) = Self::take_brace_group(trimmed) {
            (Some(PlotData::Expression(expr)), remainder)
        } else {
            (None, trimmed)
        };

        if let Some(data) = data {
            axis.plots.push(PgfPlot { options, flags, from_cycle: !has_options, data });
        }

        // consume up to and including the statement terminator
        match rest.find(';') {
            Some(end) => &rest[end + 1..],
            None => rest,
        }
    }

    // "(0, 1) (1, 2.5) (2, 4)" -> points
    fn parse_coordinates(raw: &str) -> PlotData {
        let mut points = Vec::new();
        let mut rest = raw;

        while let Some(open) = rest.find('(') {
            let after = &rest[open + 1..];
            let Some(close) = after.find(')') else { break };
            let pair = &after[..close];
            rest = &after[close + 1..];

            let mut nums = pair.split(',').map(|n| n.trim().parse::<f64>());
            if let (Some(Ok(x)), Some(Ok(y))) = (nums.next(), nums.next()) {
                points.push((x, y));
            }
        }

        PlotData::Points(points)
    }

    // Whitespace-separated columns; optional header row referenced by
    // table[x=name, y=name]. Defaults to the first two columns.
    fn parse_table(raw: &str, options: &HashMap<String, String>) -> PlotData {
        let rows: Vec<Vec<&str>> = raw.lines()
            .map(|line| line.split('%').next().unwrap_or("").trim())
            .filter(|line| !line.is_empty() && !line.starts_with("\\\\"))
            .map(|line| line.split_whitespace().collect())
            .collect();

        if rows.is_empty() {
            return PlotData::Points(Vec::new());
        }

        let has_header = rows[0].iter().any(|cell| cell.parse::<f64>().is_err());
        let col = |name: &str, default: usize| -> usize {
            let by_index = options.get(&format!("{} index", name))
                .and_then(|v| v.parse::<usize>().ok());
            if let Some(index) = by_index {
                return index;
            }
            options.get(name)
                .and_then(|wanted| rows[0].iter().position(|cell| cell == wanted))
                .unwrap_or(default)
        };

        let (x_col, y_col) = (col("x", 0), col("y", 1));
        let data_rows = if has_header { &rows[1..] } else { &rows[..] };

        let points = data_rows.iter()
            .filter_map(|row| {
                let x = row.get(x_col)?.parse::<f64>().ok()?;
                let y = row.get(y_col)?.parse::<f64>().ok()?;
                Some((x, y))
            })
            .collect();

        PlotData::Points(points)
    }

    // -----------------------------------------------------------------------
    // SVG rendering
    // -----------------------------------------------------------------------

    pub fn render_svg(axis: &PgfAxis, ctx: &mut RenderContext) -> String {
        use std::fmt::Write as _;

        ctx.phantom_id += 1;
        let clip_id = format!("pgfclip-{}", ctx.phantom_id);

        let width = axis.options.get("width")
            .and_then(|w| Self::parse_dimen(w)).unwrap_or(460.0);
        let height = axis.options.get("height")
            .and_then(|h| Self::parse_dimen(h)).unwrap_or(320.0);

        let title  = axis.options.get("title").map(|t| Self::label_text(t)).unwrap_or_default();
        let xlabel = axis.options.get("xlabel").map(|t| Self::label_text(t)).unwrap_or_default();
        let ylabel = axis.options.get("ylabel").map(|t| Self::label_text(t)).unwrap_or_default();

        let (ml, mr) = (58.0, 16.0);
        let mt = if title.is_empty() { 14.0 } else { 34.0 };
        let mb = if xlabel.is_empty() { 36.0 } else { 50.0 };
        let (plot_w, plot_h) = (width - ml - mr, height - mt - mb);

        // resolve every series to raw data points
        let series: Vec<Vec<(f64, f64)>> = axis.plots.iter()
            .map(|plot| Self::sample(plot, axis))
            .collect();

        // bounds operate in transformed (possibly log10) space
        let tx = |x: f64| if axis.x_log { x.log10() } else { x };
        let ty = |y: f64| if axis.y_log { y.log10() } else { y };

        let transformed: Vec<Vec<(f64, f64)>> = series.iter()
            .map(|points| points.iter()
                .map(|&(x, y)| (tx(x), ty(y)))
                .filter(|(x, y)| x.is_finite() && y.is_finite())
                .collect())
            .collect();

        let bound = |key: &str, log: bool| -> Option<f64> {
            let value = axis.options.get(key)?.parse::<f64>().ok()?;
            Some(if log { value.log10() } else { value })
        };

        let (xmin, xmax) = Self::bounds(
            transformed.iter().flatten().map(|p| p.0),
            bound("xmin", axis.x_log), bound("xmax", axis.x_log), axis.x_log,
        );
        let (ymin, ymax) = Self::bounds(
            transformed.iter().flatten().map(|p| p.1),
            bound("ymin", axis.y_log), bound("ymax", axis.y_log), axis.y_log,
        );

        let px = |x: f64| ml + (x - xmin) / (xmax - xmin) * plot_w;
        let py = |y: f64| mt + (1.0 - (y - ymin) / (ymax - ymin)) * plot_h;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" \
             viewBox=\"0 0 {w} {h}\" font-family=\"serif\" font-size=\"11\">\
             <defs><clipPath id=\"{clip}\"><rect x=\"{ml}\" y=\"{mt}\" width=\"{pw}\" height=\"{ph}\"/></clipPath></defs>",
            w = width, h = height, clip = clip_id, ml = ml, mt = mt, pw = plot_w, ph = plot_h,
        );

        // ---- grid + ticks ---------------------------------------------------
        let x_ticks = Self::ticks(xmin, xmax, axis.x_log, plot_w / 65.0);
        let y_ticks = Self::ticks(ymin, ymax, axis.y_log, plot_h / 42.0);

        let grid = axis.options.get("grid").map(|g| g.as_str())
            .or_else(|| axis.flags.iter().any(|f| f == "grid").then_some("major"))
            .unwrap_or("none");
        let show_grid = matches!(grid, "major" | "both" | "minor");

        if show_grid {
            for &(x, _) in &x_ticks {
                let _ = write!(svg,
                    "<line x1=\"{0:.1}\" y1=\"{1}\" x2=\"{0:.1}\" y2=\"{2}\" stroke=\"#cccccc\" stroke-width=\"0.6\"/>",
                    px(x), mt, mt + plot_h);
            }
            for &(y, _) in &y_ticks {
                let _ = write!(svg,
                    "<line x1=\"{1}\" y1=\"{0:.1}\" x2=\"{2}\" y2=\"{0:.1}\" stroke=\"#cccccc\" stroke-width=\"0.6\"/>",
                    py(y), ml, ml + plot_w);
            }
        }

        for &(x, ref label) in &x_ticks {
            let _ = write!(svg,
                "<line x1=\"{0:.1}\" y1=\"{1}\" x2=\"{0:.1}\" y2=\"{2}\" stroke=\"#000\" stroke-width=\"0.8\"/>\
                 <text x=\"{0:.1}\" y=\"{3}\" text-anchor=\"middle\">{4}</text>",
                px(x), mt + plot_h, mt + plot_h - 4.0, mt + plot_h + 15.0, label);
        }
        for &(y, ref label) in &y_ticks {
            let _ = write!(svg,
                "<line x1=\"{1}\" y1=\"{0:.1}\" x2=\"{2}\" y2=\"{0:.1}\" stroke=\"#000\" stroke-width=\"0.8\"/>\
                 <text x=\"{3}\" y=\"{4:.1}\" text-anchor=\"end\">{5}</text>",
                py(y), ml, ml + 4.0, ml - 7.0, py(y) + 3.5, label);
        }

        // ---- axis frame -----------------------------------------------------
        let axis_lines = axis.options.get("axis lines")
            .or_else(|| axis.options.get("axis lines*"))
            .map(|s| s.as_str()).unwrap_or("box");

        match axis_lines {
            "middle" | "center" => {
                let (x0, y0) = (px(0.0f64.clamp(xmin, xmax)), py(0.0f64.clamp(ymin, ymax)));
                let _ = write!(svg,
                    "<line x1=\"{ml}\" y1=\"{y0:.1}\" x2=\"{xe}\" y2=\"{y0:.1}\" stroke=\"#000\" stroke-width=\"1\"/>\
                     <line x1=\"{x0:.1}\" y1=\"{mt}\" x2=\"{x0:.1}\" y2=\"{ye}\" stroke=\"#000\" stroke-width=\"1\"/>",
                    ml = ml, xe = ml + plot_w, mt = mt, ye = mt + plot_h);
            }
            "left" => {
                let _ = write!(svg,
                    "<line x1=\"{ml}\" y1=\"{mt}\" x2=\"{ml}\" y2=\"{yb}\" stroke=\"#000\" stroke-width=\"1\"/>\
                     <line x1=\"{ml}\" y1=\"{yb}\" x2=\"{xe}\" y2=\"{yb}\" stroke=\"#000\" stroke-width=\"1\"/>",
                    ml = ml, mt = mt, yb = mt + plot_h, xe = ml + plot_w);
            }
            "none" => {}
            _ => {
                let _ = write!(svg,
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#000\" stroke-width=\"1\"/>",
                    ml, mt, plot_w, plot_h);
            }
        }

        // ---- series ---------------------------------------------------------
        let _ = write!(svg, "<g clip-path=\"url(#{})\">", clip_id);
        for (index, plot) in axis.plots.iter().enumerate() {
            let style = Self::plot_style(plot, index, ctx);
            let points: Vec<(f64, f64)> = transformed[index].iter()
                .map(|&(x, y)| (px(x), py(y)))
                .collect();
            Self::write_series(&mut svg, &points, &style);
        }
        svg.push_str("</g>");

        // ---- labels ----------------------------------------------------------
        if !title.is_empty() {
            let _ = write!(svg,
                "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" font-size=\"13\" font-weight=\"bold\">{}</text>",
                ml + plot_w / 2.0, mt - 12.0, title);
        }
        if !xlabel.is_empty() {
            let _ = write!(svg,
                "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                ml + plot_w / 2.0, height - 8.0, xlabel);
        }
        if !ylabel.is_empty() {
            let _ = write!(svg,
                "<text x=\"14\" y=\"{0:.1}\" text-anchor=\"middle\" font-size=\"12\" \
                 transform=\"rotate(-90 14 {0:.1})\">{1}</text>",
                mt + plot_h / 2.0, ylabel);
        }

        // ---- legend ----------------------------------------------------------
        if !axis.legend.is_empty() {
            Self::write_legend(&mut svg, axis, ctx, ml, mt, plot_w, plot_h);
        }

        svg.push_str("</svg>");
        svg
    }

    // Resolve a plot to data points, sampling expressions over the domain.
    fn sample(plot: &PgfPlot, axis: &PgfAxis) -> Vec<(f64, f64)> {
        match &plot.data {
            PlotData::Points(points) => points.clone(),
            PlotData::Expression(expr) => {
                let domain = plot.option("domain")
                    .or_else(|| axis.options.get("domain").map(|s| s.as_str()));
                let (start, stop) = domain
                    .and_then(|d| {
                        let (a, b) = d.split_once(':')?;
                        Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
                    })
                    .or_else(|| {
                        let min = axis.options.get("xmin")?.parse().ok()?;
                        let max = axis.options.get("xmax")?.parse().ok()?;
                        Some((min, max))
                    })
                    .unwrap_or((-5.0, 5.0));

                let samples = plot.option("samples")
                    .or_else(|| axis.options.get("samples").map(|s| s.as_str()))
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(100)
                    .max(2);

                (0..samples)
                    .map(|i| {
                        let x = start + (stop - start) * i as f64 / (samples - 1) as f64;
                        (x, PgfMath::eval(expr, x))
                    })
                    .collect()
            }
        }
    }

    // Data extent with 5% padding, honoring explicit min/max overrides.
    fn bounds(values: impl Iterator<Item = f64>, min: Option<f64>, max: Option<f64>, log: bool) -> (f64, f64) {
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for value in values.filter(|v| v.is_finite()) {
            lo = lo.min(value);
            hi = hi.max(value);
        }
        if !lo.is_finite() || !hi.is_finite() {
            (lo, hi) = if log { (0.0, 2.0) } else { (0.0, 1.0) };
        }
        if hi - lo < 1e-12 {
            lo -= 1.0;
            hi += 1.0;
        }

        let pad = (hi - lo) * 0.05;
        (min.unwrap_or(lo - pad), max.unwrap_or(hi + pad))
    }

    // Tick positions with display labels, in transformed space.
    fn ticks(min: f64, max: f64, log: bool, target: f64) -> Vec<(f64, String)> {
        if log {
            let mut ticks = Vec::new();
            let (lo, hi) = (min.floor() as i64, max.ceil() as i64);
            for exp in lo..=hi {
                let pos = exp as f64;
                if pos >= min - 1e-9 && pos <= max + 1e-9 {
                    ticks.push((pos, format!(
                        "10<tspan baseline-shift=\"super\" font-size=\"8\">{}</tspan>", exp
                    )));
                }
            }
            return ticks;
        }

        let range = max - min;
        let raw_step = range / target.max(2.0);
        let magnitude = 10f64.powf(raw_step.log10().floor());
        let normalized = raw_step / magnitude;
        let step = magnitude * if normalized < 1.5 { 1.0 }
            else if normalized < 3.0 { 2.0 }
            else if normalized < 7.0 { 5.0 }
            else { 10.0 };

        let decimals = (-step.log10().floor()).max(0.0) as usize;
        let mut ticks = Vec::new();
        let mut value = (min / step).ceil() * step;
        while value <= max + step * 1e-6 {
            let label = if value.abs() < step * 1e-6 {
                "0".to_string()
            } else {
                format!("{:.*}", decimals, value)
            };
            ticks.push((value, label));
            value += step;
        }
        ticks
    }

    // Visual style resolved from plot options + the default cycle list
    fn plot_style(plot: &PgfPlot, index: usize, ctx: &RenderContext) -> SeriesStyle {
        let (cycle_color, cycle_mark) = Self::CYCLE[index % Self::CYCLE.len()];

        let color = plot.option("color")
            .map(|c| Self::resolve_color(c, ctx))
            .or_else(|| plot.flags.iter().find_map(|flag| {
                let resolved = Self::resolve_color(flag, ctx);
                resolved.starts_with('#').then_some(resolved)
            }))
            .unwrap_or_else(|| cycle_color.to_string());

        let only_marks = plot.flag("only marks") || plot.flag("scatter");
        let mark = plot.option("mark")
            .map(|m| m.to_string())
            .unwrap_or_else(|| {
                if plot.from_cycle || only_marks { cycle_mark.to_string() } else { "none".to_string() }
            });
        let mark = if plot.flag("no marks") || plot.flag("no markers") {
            "none".to_string()
        } else {
            mark
        };

        let stroke_width =
            if plot.flag("ultra thick")     { 3.6 }
            else if plot.flag("very thick") { 2.8 }
            else if plot.flag("thick")      { 2.0 }
            else if plot.flag("semithick")  { 1.6 }
            else if plot.flag("very thin")  { 0.6 }
            else if plot.flag("thin")       { 0.9 }
            else                            { 1.3 };

        let dash =
            if plot.flag("densely dashed")      { "4,2" }
            else if plot.flag("loosely dashed") { "9,6" }
            else if plot.flag("dashed")         { "6,3" }
            else if plot.flag("densely dotted") { "1,1.5" }
            else if plot.flag("dotted")         { "1.5,2.5" }
            else                                { "" };

        let mark_size = plot.option("mark size")
            .and_then(|s| s.parse::<f64>().ok())
            .map(|pt| pt * 1.33)
            .unwrap_or(2.6);

        SeriesStyle {
            color,
            mark,
            mark_size,
            stroke_width,
            dash: dash.to_string(),
            only_marks,
        }
    }

    // "blue", "blue!50!black", "color=red" → CSS color via the document palette
    fn resolve_color(name: &str, ctx: &RenderContext) -> String {
        let base = name.split('!').next().unwrap_or(name).trim();
        ctx.resolve_color(base)
    }

    fn write_series(svg: &mut String, points: &[(f64, f64)], style: &SeriesStyle) {
        use std::fmt::Write as _;

        if !style.only_marks && points.len() > 1 {
            let dash = if style.dash.is_empty() {
                String::new()
            } else {
                format!(" stroke-dasharray=\"{}\"", style.dash)
            };

            // break the polyline at non-finite samples (poles, log gaps)
            let mut path = String::new();
            let mut pen_down = false;
            for &(x, y) in points {
                if !x.is_finite() || !y.is_finite() {
                    pen_down = false;
                    continue;
                }
                let _ = write!(path, "{}{:.1} {:.1} ", if pen_down { "L" } else { "M" }, x, y);
                pen_down = true;
            }

            if path.contains('L') {
                let _ = write!(svg,
                    "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
                    path.trim_end(), style.color, style.stroke_width, dash);
            }
        }

        if style.mark != "none" {
            for &(x, y) in points.iter().filter(|(x, y)| x.is_finite() && y.is_finite()) {
                Self::write_mark(svg, x, y, style);
            }
        }
    }

    fn write_mark(svg: &mut String, x: f64, y: f64, style: &SeriesStyle) {
        use std::fmt::Write as _;
        let (r, color) = (style.mark_size, &style.color);

        let _ = match style.mark.as_str() {
            "*" | "." => write!(svg,
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\"/>", x, y, r, color),
            "o" => write!(svg,
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>",
                x, y, r, color),
            "square" | "square*" => {
                let fill = if style.mark == "square*" { color.as_str() } else { "none" };
                write!(svg,
                    "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x - r, y - r, r * 2.0, r * 2.0, fill, color)
            }
            "triangle" | "triangle*" => {
                let fill = if style.mark == "triangle*" { color.as_str() } else { "none" };
                write!(svg,
                    "<polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x, y - r * 1.2, x - r * 1.1, y + r, x + r * 1.1, y + r, fill, color)
            }
            "diamond" | "diamond*" => {
                let fill = if style.mark == "diamond*" { color.as_str() } else { "none" };
                write!(svg,
                    "<polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x, y - r * 1.3, x + r, y, x, y + r * 1.3, x - r, y, fill, color)
            }
            "x" | "asterisk" | "star" => write!(svg,
                "<path d=\"M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1}\" stroke=\"{}\" stroke-width=\"1.2\"/>",
                x - r, y - r, x + r, y + r, x - r, y + r, x + r, y - r, color),
            "+" => write!(svg,
                "<path d=\"M{:.1} {:.1} L{:.1} {:.1} M{:.1} {:.1} L{:.1} {:.1}\" stroke=\"{}\" stroke-width=\"1.2\"/>",
                x - r, y, x + r, y, x, y - r, x, y + r, color),
            _ => write!(svg,
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\"/>", x, y, r, color),
        };
    }

    fn write_legend(
        svg: &mut String,
        axis: &PgfAxis,
        ctx: &RenderContext,
        ml: f64, mt: f64, plot_w: f64, plot_h: f64,
    ) {
        use std::fmt::Write as _;

        let entries: Vec<String> = axis.legend.iter().map(|e| Self::label_text(e)).collect();
        let row_h = 17.0;
        let box_w = entries.iter().map(|e| e.chars().count()).max().unwrap_or(0) as f64 * 6.2 + 44.0;
        let box_h = entries.len() as f64 * row_h + 8.0;

        let pos = axis.options.get("legend pos").map(|s| s.as_str()).unwrap_or("north east");
        let (bx, by) = match pos {
            "north west" => (ml + 8.0, mt + 8.0),
            "south west" => (ml + 8.0, mt + plot_h - box_h - 8.0),
            "south east" => (ml + plot_w - box_w - 8.0, mt + plot_h - box_h - 8.0),
            _            => (ml + plot_w - box_w - 8.0, mt + 8.0), // north east
        };

        let _ = write!(svg,
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
             fill=\"#ffffff\" fill-opacity=\"0.9\" stroke=\"#000\" stroke-width=\"0.7\"/>",
            bx, by, box_w, box_h);

        for (i, entry) in entries.iter().enumerate() {
            let cy = by + 8.0 + i as f64 * row_h + row_h / 2.0 - 4.0;
            if let Some(plot) = axis.plots.get(i) {
                let style = Self::plot_style(plot, i, ctx);
                if !style.only_marks {
                    let dash = if style.dash.is_empty() {
                        String::new()
                    } else {
                        format!(" stroke-dasharray=\"{}\"", style.dash)
                    };
                    let _ = write!(svg,
                        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
                        bx + 6.0, cy, bx + 30.0, cy, style.color, style.stroke_width, dash);
                }
                if style.mark != "none" {
                    Self::write_mark(svg, bx + 18.0, cy, &style);
                }
            }
            let _ = write!(svg,
                "<text x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                bx + 36.0, cy + 3.5, entry);
        }
    }

    // LaTeX dimension to CSS pixels (96 dpi)
    fn parse_dimen(value: &str) -> Option<f64> {
        let value = value.trim();
        let split = value.find(|c: char| c.is_alphabetic()).unwrap_or(value.len());
        let number: f64 = value[..split].trim().parse().ok()?;

        let scale = match value[split..].trim() {
            "cm"        => 37.8,
            "mm"        => 3.78,
            "in"        => 96.0,
            "pt" | "bp" => 1.33,
            "em"        => 16.0,
            "ex"        => 8.0,
            "" | "px"   => 1.0,
            _           => return None,
        };
        Some(number * scale)
    }

    // Axis label / legend text: drop math delimiters and TeX control noise,
    // escape for SVG.
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
                    // skip the control word, keep its argument text
                    while chars.peek().is_some_and(|n| n.is_alphabetic()) {
                        chars.next();
                    }
                    out.push(' ');
                }
                _ => out.push(c),
            }
        }

        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

}

struct SeriesStyle {
    color:        String,
    mark:         String,
    mark_size:    f64,
    stroke_width: f64,
    dash:         String,
    only_marks:   bool,
}

// ---------------------------------------------------------------------------
// pgfmath expression evaluator
// ---------------------------------------------------------------------------
// Evaluates pgfmath expressions like "x^2 + 3*sin(deg(x))".
// Trigonometric functions take degrees, matching pgfplots defaults.
struct PgfMath<'a> {
    input: &'a [u8],
    pos:   usize,
    x:     f64,
}

impl<'a> PgfMath<'a> {

    fn eval(expr: &str, x: f64) -> f64 {
        let mut parser = PgfMath { input: expr.as_bytes(), pos: 0, x };
        let value = parser.expression();
        parser.skip_ws();
        if parser.pos < parser.input.len() {
            return f64::NAN; // trailing garbage — not a valid expression
        }
        value
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_ws();
        self.input.get(self.pos).copied()
    }

    fn expression(&mut self) -> f64 {
        let mut value = self.term();
        loop {
            match self.peek() {
                Some(b'+') => { self.pos += 1; value += self.term(); }
                Some(b'-') => { self.pos += 1; value -= self.term(); }
                _ => return value,
            }
        }
    }

    fn term(&mut self) -> f64 {
        let mut value = self.factor();
        loop {
            match self.peek() {
                Some(b'*') => { self.pos += 1; value *= self.factor(); }
                Some(b'/') => { self.pos += 1; value /= self.factor(); }
                _ => return value,
            }
        }
    }

    fn factor(&mut self) -> f64 {
        let base = self.unary();
        if self.peek() == Some(b'^') {
            self.pos += 1;
            let exponent = self.factor(); // right-associative
            return base.powf(exponent);
        }
        base
    }

    fn unary(&mut self) -> f64 {
        match self.peek() {
            Some(b'-') => { self.pos += 1; -self.unary() }
            Some(b'+') => { self.pos += 1; self.unary() }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> f64 {
        match self.peek() {
            Some(b'(') => {
                self.pos += 1;
                let value = self.expression();
                if self.peek() == Some(b')') { self.pos += 1; }
                value
            }
            Some(c) if c.is_ascii_digit() || c == b'.' => self.number(),
            Some(c) if c.is_ascii_alphabetic() => self.identifier(),
            _ => f64::NAN,
        }
    }

    fn number(&mut self) -> f64 {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == b'.')
        {
            self.pos += 1;
        }
        // scientific notation: 1.5e-3
        if self.input.get(self.pos).is_some_and(|&c| c == b'e' || c == b'E') {
            let mut probe = self.pos + 1;
            if self.input.get(probe).is_some_and(|&c| c == b'+' || c == b'-') {
                probe += 1;
            }
            if self.input.get(probe).is_some_and(|c| c.is_ascii_digit()) {
                self.pos = probe + 1;
                while self.input.get(self.pos).is_some_and(|c| c.is_ascii_digit()) {
                    self.pos += 1;
                }
            }
        }

        std::str::from_utf8(&self.input[start..self.pos])
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(f64::NAN)
    }

    fn identifier(&mut self) -> f64 {
        let start = self.pos;
        while self.input.get(self.pos).is_some_and(|c| c.is_ascii_alphanumeric()) {
            self.pos += 1;
        }
        let name = std::str::from_utf8(&self.input[start..self.pos]).unwrap_or("");

        match name {
            "x"  => return self.x,
            "pi" => return std::f64::consts::PI,
            "e"  => return std::f64::consts::E,
            _ => {}
        }

        // function call
        if self.peek() != Some(b'(') {
            return f64::NAN;
        }
        self.pos += 1;
        let first = self.expression();
        let second = if self.peek() == Some(b',') {
            self.pos += 1;
            Some(self.expression())
        } else {
            None
        };
        if self.peek() == Some(b')') { self.pos += 1; }

        let deg = std::f64::consts::PI / 180.0;
        match (name, second) {
            // pgfmath trig works in degrees
            ("sin", None)   => (first * deg).sin(),
            ("cos", None)   => (first * deg).cos(),
            ("tan", None)   => (first * deg).tan(),
            ("asin", None)  => first.asin() / deg,
            ("acos", None)  => first.acos() / deg,
            ("atan", None)  => first.atan() / deg,
            ("sinh", None)  => first.sinh(),
            ("cosh", None)  => first.cosh(),
            ("tanh", None)  => first.tanh(),
            ("deg", None)   => first / deg,
            ("rad", None)   => first * deg,
            ("exp", None)   => first.exp(),
            ("ln", None)    => first.ln(),
            ("log10", None) => first.log10(),
            ("log2", None)  => first.log2(),
            ("sqrt", None)  => first.sqrt(),
            ("abs", None)   => first.abs(),
            ("floor", None) => first.floor(),
            ("ceil", None)  => first.ceil(),
            ("round", None) => first.round(),
            ("sign", None)  => first.signum(),
            ("min", Some(b))   => first.min(b),
            ("max", Some(b))   => first.max(b),
            ("pow", Some(b))   => first.powf(b),
            ("mod", Some(b))   => first % b,
            ("atan2", Some(b)) => first.atan2(b) / deg,
            _ => f64::NAN,
        }
    }

}
