use crate::render::latex::tex_ast::LatexNode;

pub struct Parser {
    chars: Vec<char>,
    pos: usize,
    in_document: bool,
}

impl Parser {

    pub fn new(input: &str) -> Self {
        Parser {
            chars: input.chars().collect(), 
            pos: 0, 
            in_document: false 
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let res = self.chars[self.pos];
            self.pos += 1;
            Some(res)
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.pos])
        }
    }
    
    pub fn parse_text(&mut self) -> String {
        let mut text = String::new();
        
        while let Some(&c) = self.chars.get(self.pos) {
            if c == '\\' || c == '{' || c == '}' || c == '^' || c == '_' || c == '%' || c == '$' { 
                break; 
            }

            text.push(c); self.pos += 1;
        }
        
        text
    }

    pub fn parse_braces_content(&mut self) -> String {
        while self.peek().map_or(false, |c| c.is_whitespace()) { self.next_char(); }
        if self.peek() == Some('{') {
            self.next_char();
            let mut content = String::new();
            let mut depth = 1;

            while depth > 0 {
                if let Some(c) = self.next_char() {
                    if c == '{' { depth += 1; } if c == '}' {
                        depth -= 1;
                    }

                    if depth > 0 {
                        content.push(c);
                    }
                } else {
                    break;
                }
            }

            content
        } else {
            String::new()
        }
    }

    pub fn parse_argument(&mut self) -> Vec<LatexNode> {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.next_char();
        }

        if self.peek() == Some('{') {
            Parser::new(&self.parse_braces_content()).parse(true)
        } else {
            vec![LatexNode::Text(self.next_char().unwrap_or(' ').to_string())]
        }
    }

    pub fn parse(&mut self, force_active: bool) -> Vec<LatexNode> {
        let mut nodes = Vec::new();
        if force_active { self.in_document = true; }

        while self.pos < self.chars.len() {
            let current = self.chars[self.pos];

            if current == '%' {
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    self.pos += 1;
                }
                
                continue;
            }

            if current == '\\' {
                self.pos += 1; let mut command = String::new();

                while let Some(&c) = self.chars.get(self.pos) {
                    if c.is_alphabetic() {
                        command.push(c); self.pos += 1;
                    } else {
                        break;
                    }
                }

                match command.as_str() {
                    "begin" => {
                        let env = self.parse_braces_content();

                        if env == "document" {
                            self.in_document = true;
                        }else if (env == "lstlisting" || env == "verbatim") && self.in_document {
                            let mut options = String::new();
                            while self.peek().map_or(false, |c| c.is_whitespace()) {
                                self.next_char();
                            }
                            
                            if self.peek() == Some('[') {
                                self.next_char();
                                
                                while let Some(c) = self.next_char() {
                                    if c == ']' {
                                        break;
                                    }

                                    options.push(c);
                                }
                            }

                            let mut raw_code = String::new();
                            let end_tag = format!("\\end{{{}}}", env);

                            while self.pos < self.chars.len() {
                                let lookahead: String = self.chars[self.pos..].iter().take(end_tag.len()).collect();
                                if lookahead == end_tag {
                                    self.pos += end_tag.len();
                                    break;
                                }

                                if let Some(c) = self.next_char() {
                                    raw_code.push(c);
                                }
                            }

                            nodes.push(LatexNode::CodeBlock(raw_code.trim_matches('\n').to_string()));
                        } else if env == "itemize" && self.in_document {
                            let mut items = Vec::new(); 
                            let mut block = String::new();

                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{itemize}") {
                                    self.pos += "\\end{itemize}".len(); 
                                    break;
                                }

                                if let Some(c) = self.next_char() {
                                    block.push(c);
                                } 
                            }

                            for item in block.split("\\item") {
                                let trimmed = item.trim();
                                if !trimmed.is_empty() { items.push(Parser::new(trimmed).parse(true)); }
                            }
                            
                            nodes.push(LatexNode::Itemize(items));
                        } else if env == "enumerate" && self.in_document {
                            let mut items = Vec::new(); let mut block = String::new();

                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{enumerate}") {
                                    self.pos += "\\end{enumerate}".len();
                                    break;
                                }

                                if let Some(c) = self.next_char() { block.push(c); }
                            }
                            
                            for item in block.split("\\item") {
                                let trimmed = item.trim();
                                
                                if !trimmed.is_empty() {
                                    items.push(Parser::new(trimmed).parse(true));
                                }
                            }
                            
                            nodes.push(LatexNode::Enumerate(items));
                        } else if env == "mermaid" && self.in_document {
                            let mut raw_block = String::new();

                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{mermaid}") {
                                    self.pos += "\\end{mermaid}".len(); 
                                    break;
                                }

                                if let Some(c) = self.next_char() {
                                    raw_block.push(c);
                                }
                            }

                            nodes.push(LatexNode::Mermaid(raw_block));
                        } else if env == "tabular" && self.in_document {
                            self.parse_braces_content();

                            let mut table_block = String::new();
                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{tabular}") {
                                    self.pos += "\\end{tabular}".len();
                                    break;
                                }

                                if let Some(c) = self.next_char() {
                                    table_block.push(c);
                                }
                            }

                            let mut rows = Vec::new();

                            for row_str in table_block.split(r"\\") {
                                let clean_row = row_str.replace("\\hline", "").trim().to_string();
                                if !clean_row.is_empty() {
                                    let mut cells = Vec::new();
                                    for cell_str in clean_row.split('&') { cells.push(Parser::new(cell_str.trim()).parse(true)); }
                                    rows.push(cells);
                                }
                            }

                            nodes.push(LatexNode::Table(rows));
                        } else if env == "verbatim" && self.in_document {
                            let mut raw_block = String::new();

                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{verbatim}") {
                                    self.pos += "\\end{verbatim}".len(); break;
                                }

                                if let Some(c) = self.next_char() {
                                    raw_block.push(c);
                                }
                            }

                            nodes.push(LatexNode::CodeBlock(raw_block.trim_matches('\n').to_string()));
                        } else if env == "equation" && self.in_document { 
                            let mut math_block = String::new();

                            while self.pos < self.chars.len() {
                                if self.chars[self.pos..].iter().collect::<String>().starts_with("\\end{equation}") {
                                    self.pos += "\\end{equation}".len(); break;
                                }

                                if let Some(c) = self.next_char() {
                                    math_block.push(c);
                                }
                            }

                            nodes.push(LatexNode::EquationBlock(Parser::new(&math_block).parse(true)));
                        }
                    }
                    
                    "end" => {
                        let env = self.parse_braces_content();
                        
                        if env == "document" {
                            self.in_document = false;
                        } 
                    }
                    "documentclass" | "usepackage" => {
                        while self.peek() == Some('[') { while let Some(c) = self.next_char() { if c == ']' { break; } } }
                        self.parse_braces_content();
                    }
                    
                    "maketitle" if self.in_document => nodes.push(LatexNode::MakeTitle),
                    "tableofcontents" if self.in_document => nodes.push(LatexNode::TableOfContents),
                    "section" if self.in_document => nodes.push(LatexNode::Section(self.parse_braces_content())),
                    "subsection" if self.in_document => nodes.push(LatexNode::Subsection(self.parse_braces_content())),
                    "label" if self.in_document => nodes.push(LatexNode::Label(self.parse_braces_content())),
                    "ref" if self.in_document => nodes.push(LatexNode::Ref(self.parse_braces_content())),
                    "caption" if self.in_document => nodes.push(LatexNode::Caption(self.parse_braces_content())),
                    "includegraphics" if self.in_document => nodes.push(LatexNode::Image(self.parse_braces_content())),
                    "textbf" if self.in_document => nodes.push(LatexNode::Bold(Parser::new(&self.parse_braces_content()).parse(true))),
                    "textit" if self.in_document => nodes.push(LatexNode::Italic(Parser::new(&self.parse_braces_content()).parse(true))),
                    "math" if self.in_document => nodes.push(LatexNode::MathInline(self.parse_argument())),
                    "frac" if self.in_document => { let num = self.parse_argument(); let den = self.parse_argument(); nodes.push(LatexNode::Fraction { num, den }); }
                    "sqrt" if self.in_document => { let arg = self.parse_argument(); nodes.push(LatexNode::Text("√(".to_string())); nodes.extend(arg); nodes.push(LatexNode::Text(")".to_string())); }
                    "int" | "infty" | "pi" | "alpha" | "beta" | "gamma" | "Delta" if self.in_document => {
                        let symbol = match command.as_str() { "int"=>"∫", "infty"=>"∞", "pi"=>"π", "alpha"=>"α", "beta"=>"β", "gamma"=>"γ", "Delta"=>"Δ", _=>"" };
                        nodes.push(LatexNode::Text(symbol.to_string()));
                    }

                    "textbf" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Bold(Parser::new(&content).parse(true)));
                    }

                    "textit" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Italic(Parser::new(&content).parse(true)));
                    }

                    "author" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Author(Parser::new(&content).parse(true)));
                    }
                    
                    "title" => {
                        let content = self.parse_braces_content();
                        nodes.push(LatexNode::Title(Parser::new(&content).parse(true)));
                    }

                    "underline" if self.in_document => {
                        nodes.push(LatexNode::Underline(Parser::new(&self.parse_braces_content()).parse(true)));
                    }
                    "texttt" if self.in_document => {
                        nodes.push(LatexNode::Monospace(Parser::new(&self.parse_braces_content()).parse(true)));
                    }
                    "textsc" if self.in_document => {
                        nodes.push(LatexNode::SmallCaps(Parser::new(&self.parse_braces_content()).parse(true)));
                    }

                    "tiny" | "small" | "large" | "Large" | "LARGE" | "huge" | "Huge" | "HUGE" if self.in_document => {
                        let mut rest_of_content = String::new();
                        
                        while self.pos < self.chars.len() {
                            rest_of_content.push(self.chars[self.pos]);
                            self.pos += 1;
                        }
                        
                        nodes.push(LatexNode::FontSize(
                            command.clone(), 
                            Parser::new(&rest_of_content).parse(true)
                        ));
                    }

                    "vspace" if self.in_document => {
                        nodes.push(LatexNode::VSpace(self.parse_braces_content()));
                    }

                    "url" if self.in_document => {
                        nodes.push(LatexNode::Url(self.parse_braces_content()));
                    }
                    "href" if self.in_document => {
                        let link_url = self.parse_braces_content();
                        let link_text = self.parse_argument();

                        nodes.push(LatexNode::Href { 
                            url: link_url, 
                            text: link_text 
                        });
                    }

                    "cite" if self.in_document => nodes.push(LatexNode::Cite(self.parse_braces_content())),
                    "bibliography" if self.in_document => nodes.push(LatexNode::Bibliography(self.parse_braces_content())),
                    "hline" => {} 
                    _ => {}
                }
            } else if current == '^' && self.in_document {
                self.pos += 1; nodes.push(LatexNode::Superscript(self.parse_argument()));
            } else if current == '_' && self.in_document {
                self.pos += 1; nodes.push(LatexNode::Subscript(self.parse_argument()));
            } else if current == '$' && self.in_document {
                self.pos += 1;
                let mut math_block = String::new();

                while let Some(&c) = self.chars.get(self.pos) {
                    if c == '$' {
                        self.pos += 1;
                        break;
                    }

                    math_block.push(c);
                    self.pos += 1;
                }

                nodes.push(LatexNode::MathInline(Parser::new(&math_block).parse(true)));
            } else if current == '{' && self.in_document {
                let content = self.parse_braces_content();
                nodes.extend(Parser::new(&content).parse(true));
            } else if current == '}' && self.in_document {
                self.pos += 1;
            } else {
                let text = self.parse_text();

                if text.is_empty() {
                    let special_char = self.chars[self.pos];
                    
                    if self.in_document { 
                        nodes.push(LatexNode::Text(special_char.to_string())); 
                    }
                    
                    self.pos += 1;
                } else if self.in_document && !text.trim().is_empty() { 
                    nodes.push(LatexNode::Text(text)); 
                }
            }
        }

        nodes
    }

}