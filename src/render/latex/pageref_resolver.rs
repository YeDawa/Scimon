// pageref_resolver.rs
//
// Resolves \pageref{} references server-side by scanning the rendered HTML
// and estimating page numbers from element heights — no JavaScript required.
//
// Strategy:
//   1. Walk the HTML string token by token, accumulating an estimated vertical
//      offset (in CSS pixels) for each element we recognise.
//   2. When we encounter an anchor element (id="item-N" or id="label-…"),
//      record its estimated offset.
//   3. After the full scan, for every <a data-ref="…">?? replace ?? with
//      floor(offset / PAGE_HEIGHT) + 1.
//
// Heights are rough estimates based on typical Lora 18px rendering at 850px
// container width with 60px top/bottom padding.  They don't need to be exact —
// ±half a page is acceptable for a \pageref.

pub struct PageRefResolver;

// ---------------------------------------------------------------------------
// Estimated heights for known HTML patterns (in px)
// ---------------------------------------------------------------------------
const PAGE_H: f32       = 1122.0; // A4 @ 96 dpi
const BODY_OFFSET: f32  =   40.0; // body padding-top
const CONTAINER_PAD: f32 =  60.0; // .document-container padding-top

// Per-element height estimates
const H1: f32           = 120.0; // title-block or chapter
const H2: f32           =  72.0; // section heading + margin
const H3: f32           =  52.0; // subsection
const H4: f32           =  44.0; // subsubsection
const P_LINE: f32       =  30.6; // one line of body text (18px * 1.7)
const MATH_BLOCK: f32   =  90.0; // equation / align block
const CODE_BLOCK: f32   = 120.0; // pre block (rough)
const TABLE_ROW: f32    =  42.0; // one <tr>
const TABLE_HEAD: f32   =  20.0; // <tbody> open
const IMG: f32          = 300.0; // default image height
const LIST_ITEM: f32    =  32.0; // <li>
const HR: f32           =  30.0; // horizontal rule
const VSPACE_DEFAULT: f32 = 20.0;
const TITLE_BLOCK: f32  = 180.0;

impl PageRefResolver {

    /// Takes the rendered HTML body (without the outer template wrapper),
    /// scans it for anchor ids and data-ref attributes, computes page numbers,
    /// and returns the body with all `>??<` placeholders replaced.
    pub fn resolve(html: &str) -> String {
        // -----------------------------------------------------------------
        // Pass 1: scan for id= positions and build id → page map
        // -----------------------------------------------------------------
        let mut offset: f32 = BODY_OFFSET + CONTAINER_PAD;
        let mut id_page: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();

        let mut pos = 0usize;
        let len   = html.len();

        while pos < len {
            // Find next '<'
            let tag_start = match html[pos..].find('<') {
                Some(i) => pos + i,
                None    => break,
            };

            // Find '>'
            let tag_end = match html[tag_start..].find('>') {
                Some(i) => tag_start + i + 1,
                None    => break,
            };

            let tag = &html[tag_start..tag_end];
            pos = tag_end;

            // Record id → current page before accounting for this element's height
            if let Some(id) = Self::extract_attr(tag, "id") {
                let page = (offset / PAGE_H).floor() as u32 + 1;
                id_page.insert(id, page);
            }

            // Accumulate height based on tag type
            let tag_lower = tag.to_ascii_lowercase();
            let h = Self::height_of(&tag_lower, tag);
            offset += h;
        }

        // -----------------------------------------------------------------
        // Pass 2: replace data-ref="..." >??< with computed page numbers
        // -----------------------------------------------------------------
        let mut result = html.to_string();
        // We look for the pattern: data-ref="ID">??
        // and replace ?? with the page number.
        // Iterate over all occurrences.
        let mut search_pos = 0usize;
        loop {
            let Some(dr_start) = result[search_pos..].find("data-ref=\"") else { break };
            let dr_start = search_pos + dr_start + "data-ref=\"".len();
            let Some(dr_end) = result[dr_start..].find('"') else { break };
            let ref_id = result[dr_start..dr_start + dr_end].to_string();
            search_pos = dr_start + dr_end + 1;

            // Find the closing >?? after this attribute
            let Some(gt_rel) = result[search_pos..].find('>') else { break };
            let content_start = search_pos + gt_rel + 1;
            if result[content_start..].starts_with("??") {
                // Resolve: try the id directly, then item- / label- variants
                let page = Self::resolve_id(&ref_id, &id_page);
                let page_str = page.to_string();
                result.replace_range(content_start..content_start + 2, &page_str);
                search_pos = content_start + page_str.len();
            }
        }

        result
    }

    // -----------------------------------------------------------------------
    // Try to find a page for id, with item-/label- fallback
    // -----------------------------------------------------------------------
    fn resolve_id(id: &str, map: &std::collections::HashMap<String, u32>) -> u32 {
        if let Some(&p) = map.get(id) { return p; }
        if id.starts_with("item-") {
            let suffix = &id["item-".len()..];
            if let Some(&p) = map.get(&format!("label-{}", suffix)) { return p; }
        }

        if id.starts_with("label-") {
            let suffix = &id["label-".len()..];
            if let Some(&p) = map.get(&format!("item-{}", suffix)) { return p; }
        }

        1 // fallback: page 1
    }

    // -----------------------------------------------------------------------
    // Estimate the height contributed by a given opening tag
    // -----------------------------------------------------------------------
    fn height_of(tag_lower: &str, tag: &str) -> f32 {
        if tag_lower.starts_with("<h1") { H1 }
        else if tag_lower.starts_with("<h2") { H2 }
        else if tag_lower.starts_with("<h3") { H3 }
        else if tag_lower.starts_with("<h4") { H4 }
        else if tag_lower.starts_with("<hr") { HR }
        else if tag_lower.starts_with("<br") { P_LINE }
        else if tag_lower.starts_with("<img") { IMG }
        else if tag_lower.starts_with("<li") { LIST_ITEM }
        else if tag_lower.starts_with("<tr") { TABLE_ROW }
        else if tag_lower.starts_with("<tbody") { TABLE_HEAD }
        else if tag_lower.starts_with("<pre") { CODE_BLOCK }
        else if tag_lower.starts_with("<div") {
            if tag.contains("title-block")  { TITLE_BLOCK }
            else if tag.contains("math-block") { MATH_BLOCK }
            else if tag.contains("math-display") { MATH_BLOCK }
            else if tag.contains("height:") {
                // VSpace: try to parse the inline height value
                Self::parse_vspace(tag).unwrap_or(VSPACE_DEFAULT)
            }
            else { P_LINE } // generic div: one line
        }
        else if tag_lower.starts_with("<p") { P_LINE * 3.0 } // avg paragraph
        else { 0.0 }
    }

    // -----------------------------------------------------------------------
    // Parse height from style="height: Xpx" or style="height: Xem"
    // -----------------------------------------------------------------------
    fn parse_vspace(tag: &str) -> Option<f32> {
        let height_pos = tag.find("height:")?;
        let val_str = tag[height_pos + 7..].trim_start();
        if let Some(px_end) = val_str.find("px") {
            return val_str[..px_end].trim().parse::<f32>().ok();
        }

        if let Some(em_end) = val_str.find("em") {
            let em = val_str[..em_end].trim().parse::<f32>().ok()?;
            return Some(em * 18.0); // 1em = 18px (body font-size)
        }

        None
    }

    // -----------------------------------------------------------------------
    // Extract the value of a named attribute from a tag string
    // e.g. extract_attr(`<h2 id="item-3">`, "id") → Some("item-3")
    // -----------------------------------------------------------------------
    fn extract_attr(tag: &str, attr: &str) -> Option<String> {
        let needle = format!("{}=\"", attr);
        let start  = tag.find(&needle)? + needle.len();
        let end    = tag[start..].find('"')? + start;
        Some(tag[start..end].to_string())
    }

}