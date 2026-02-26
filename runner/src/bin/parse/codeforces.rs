use scraper::{Html, Selector, Node};
use regex::Regex;

#[derive(Debug, Default)]
pub struct Problem {
    pub code: String,
    pub name: String,
    pub url: String,
    pub description: String,
    pub input_spec: String,
    pub output_spec: String,
    pub examples: Vec<(String, String)>,
    pub note: String,
    pub time_limit: String,
    pub memory_limit: String,
    pub tier: String,
    pub tags: Vec<String>,
}

pub trait Parser {
    fn parse_to_readme(&self, html: &str, url: &str) -> anyhow::Result<String>;
}

pub struct Codeforces;

impl Codeforces {
    pub fn parse(&self, html: &str, url: &str) -> anyhow::Result<Problem> {
        let document = Html::parse_document(html);
        let mut problem = Problem::default();
        problem.url = url.to_string();

        if let Some(pos) = url.find("/problem/") {
            let code_part = &url[pos + 9..];
            problem.code = code_part.replace("/", ""); 
        }

        let title_sel = Selector::parse(".title").unwrap();
        if let Some(title_elem) = document.select(&title_sel).next() {
            let full_title = title_elem.text().collect::<String>().trim().to_string();
            if let Some(pos) = full_title.find('.') {
                problem.name = full_title[pos + 1..].trim().to_string();
                if problem.code.is_empty() {
                    problem.code = full_title[..pos].trim().replace(" ", "");
                }
            } else {
                problem.name = full_title;
            }
        }

        let tl_sel = Selector::parse(".time-limit").unwrap();
        if let Some(tl_elem) = document.select(&tl_sel).next() {
            let raw_tl = tl_elem.text().collect::<String>().replace("time limit per test", "").trim().to_string();
            problem.time_limit = format_time_limit(&raw_tl);
        }
        let ml_sel = Selector::parse(".memory-limit").unwrap();
        if let Some(ml_elem) = document.select(&ml_sel).next() {
            let raw_ml = ml_elem.text().collect::<String>().replace("memory limit per test", "").trim().to_string();
            problem.memory_limit = format_memory_limit(&raw_ml);
        }

        let body_selector = Selector::parse(".problem-statement > div:not(.header):not(.input-specification):not(.output-specification):not(.sample-tests):not(.note)").unwrap();
        if let Some(desc_elem) = document.select(&body_selector).next() {
            problem.description = node_to_markdown(desc_elem.into());
        }

        let ispec_sel = Selector::parse(".input-specification").unwrap();
        if let Some(input_elem) = document.select(&ispec_sel).next() {
            problem.input_spec = node_to_markdown(input_elem.into());
            problem.input_spec = problem.input_spec.replace("Input", "").trim().to_string();
        }

        let ospec_sel = Selector::parse(".output-specification").unwrap();
        if let Some(output_elem) = document.select(&ospec_sel).next() {
            problem.output_spec = node_to_markdown(output_elem.into());
            problem.output_spec = problem.output_spec.replace("Output", "").trim().to_string();
        }

        let sample_test_selector = Selector::parse(".sample-test").unwrap();
        if let Some(sample_test_elem) = document.select(&sample_test_selector).next() {
            let in_sel = Selector::parse(".input pre").unwrap();
            let out_sel = Selector::parse(".output pre").unwrap();
            let inputs: Vec<String> = sample_test_elem.select(&in_sel).map(|e| extract_text_from_pre(e)).collect();
            let outputs: Vec<String> = sample_test_elem.select(&out_sel).map(|e| extract_text_from_pre(e)).collect();
            for (i, o) in inputs.into_iter().zip(outputs.into_iter()) {
                problem.examples.push((i, o));
            }
        }

        let note_sel = Selector::parse(".note").unwrap();
        if let Some(note_elem) = document.select(&note_sel).next() {
            problem.note = node_to_markdown(note_elem.into());
            problem.note = problem.note.replace("Note", "").trim().to_string();
        }

        problem.tier = extract_tier(html);
        let all_tags = extract_tags(html);
        problem.tags = all_tags.into_iter().filter(|t| !t.starts_with('*')).collect();

        Ok(problem)
    }
}

impl Parser for Codeforces {
    fn parse_to_readme(&self, html: &str, url: &str) -> anyhow::Result<String> {
        let problem = self.parse(html, url)?;
        Ok(generate_readme(&problem))
    }
}

fn format_time_limit(raw: &str) -> String {
    let re = Regex::new(r"(\d+)\s*seconds?").unwrap();
    if let Some(caps) = re.captures(raw) {
        return format!("{}초", &caps[1]);
    }
    raw.to_string()
}

fn format_memory_limit(raw: &str) -> String {
    let re = Regex::new(r"(\d+)\s*megabytes?").unwrap();
    if let Some(caps) = re.captures(raw) {
        return format!("{}MB", &caps[1]);
    }
    raw.to_string()
}

fn extract_text_from_pre(element: scraper::element_ref::ElementRef) -> String {
    let line_selector = Selector::parse("div.test-example-line").unwrap();
    let lines: Vec<_> = element.select(&line_selector).map(|div| div.text().collect::<String>()).collect();
    if lines.is_empty() {
        element.text().collect::<Vec<_>>().join("\n").trim().to_string()
    } else {
        lines.join("\n").trim().to_string()
    }
}

fn node_to_markdown(node: scraper::element_ref::ElementRef) -> String {
    let result = node_to_markdown_internal(node, false);
    
    let res = result.replace("&nbsp;", " ")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"");
    
    // Collapse multiple spaces into one
    let re_spaces = Regex::new(r" +").unwrap();
    let res = re_spaces.replace_all(&res, " ").to_string();

    // Clean up spaces around math delimiters
    // 1. Remove spaces immediately inside $: "$ x $" -> "$x$"
    let re_math_start = Regex::new(r"\$\s+").unwrap();
    let res = re_math_start.replace_all(&res, "$").to_string();
    let re_math_end = Regex::new(r"\s+\$").unwrap();
    let res = re_math_end.replace_all(&res, "$").to_string();

    // 2. Remove spaces between parentheses and math: "( $" -> "($", "$ )" -> "$)"
    let res = res.replace("( $", "($").replace("$ )", "$)");
    
    let mut cleaned = String::new();
    let mut newline_count = 0;
    for c in res.trim().chars() {
        if c == '\n' {
            newline_count += 1;
            if newline_count <= 2 {
                cleaned.push(c);
            }
        } else {
            cleaned.push(c);
            newline_count = 0;
        }
    }
    cleaned.trim().to_string()
}

fn node_to_markdown_internal(node: scraper::element_ref::ElementRef, in_latex: bool) -> String {
    let mut result = String::new();
    for child in node.children() {
        match child.value() {
            Node::Text(text) => {
                result.push_str(&text);
            }
            Node::Element(elem) => {
                let name = elem.name();
                let class = elem.attr("class").unwrap_or("");
                
                if class.contains("MathJax_Preview") || class.contains("MathJax_PlainSource") || class.contains("mjx-") || class.contains("mjx-assistive-mml") {
                    continue;
                }
                
                if name == "script" && elem.attr("type") == Some("math/tex") {
                    let script_node = scraper::ElementRef::wrap(child).unwrap();
                    let script_text: String = script_node.text().collect();
                    result.push_str(&format!("${}$", script_text.trim()));
                    continue;
                }
                
                let child_node = scraper::ElementRef::wrap(child).unwrap();
                
                if class.contains("tex-font-style-it") {
                    let child_md = node_to_markdown_internal(child_node, true);
                    if in_latex {
                        result.push_str(&format!("\\mathit{{{}}}", child_md.trim()));
                    } else {
                        result.push_str(&format!("$\\mathit{{{}}}$", child_md.trim()));
                    }
                } else if class.contains("tex-font-style-bf") {
                    let child_md = node_to_markdown_internal(child_node, true);
                    if in_latex {
                        result.push_str(&format!("\\mathbf{{{}}}", child_md.trim()));
                    } else {
                        result.push_str(&format!("$\\mathbf{{{}}}$", child_md.trim()));
                    }
                } else if class.contains("tex-font-style-tt") {
                    let child_md = node_to_markdown_internal(child_node, true);
                    if in_latex {
                        result.push_str(&format!("\\mathtt{{{}}}", child_md.trim()));
                    } else {
                        result.push_str(&format!("$\\mathtt{{{}}}$", child_md.trim()));
                    }
                } else if class.contains("tex-span") {
                    let child_md = node_to_markdown_internal(child_node, true);
                    if in_latex {
                        result.push_str(&child_md);
                    } else {
                        result.push_str(&format!("${}$", child_md.trim()));
                    }
                } else {
                    let child_md = node_to_markdown_internal(child_node, in_latex);
                    if name == "p" {
                        result.push_str("\n\n");
                        result.push_str(&child_md);
                        result.push_str("\n\n");
                    } else if name == "li" {
                        result.push_str(&format!("- {}\n", child_md.trim()));
                    } else if name == "ul" {
                        result.push_str("\n");
                        result.push_str(&child_md);
                        result.push_str("\n");
                    } else if name == "br" {
                        result.push_str("\n");
                    } else {
                        result.push_str(&child_md);
                    }
                }
            }
            _ => {}
        }
    }
    result
}

fn extract_tier(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse("span[title='Difficulty']").unwrap();
    if let Some(tier_elem) = document.select(&selector).next() {
        tier_elem.text().collect::<String>().trim().replace("*", "")
    } else {
        "알 수 없음".to_string()
    }
}

fn extract_tags(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(".tag-box").unwrap();
    document.select(&selector).map(|e| e.text().collect::<String>().trim().to_string()).collect()
}

fn generate_readme(p: &Problem) -> String {
    let mut readme = format!("# [Codeforces {} : {}]({})\n\n", p.code, p.name, p.url);
    readme.push_str("## 문제 설명\n\n");
    readme.push_str(&p.description);
    readme.push_str("\n\n## 입력\n\n");
    readme.push_str(&p.input_spec);
    readme.push_str("\n\n## 출력\n\n");
    readme.push_str(&p.output_spec);
    
    readme.push_str("\n\n## 노트\n\n");
    if !p.note.is_empty() {
        readme.push_str(&p.note);
    }
    
    readme.push_str("\n\n## 예제\n\n");
    for (i, (input, output)) in p.examples.iter().enumerate() {
        readme.push_str(&format!("### {}\n\n#### 입력\n\n```\n{}\n```\n\n#### 출력\n\n```\n{}\n```\n\n", i + 1, input, output));
    }
    readme.push_str("## 티어(난이도)\n\n");
    readme.push_str(&p.tier);
    readme.push_str("\n\n## 제한\n\n| 시간 | 메모리 |\n|:----:|:------:|\n");
    readme.push_str(&format!("| {} | {} |\n\n", p.time_limit, p.memory_limit));
    readme.push_str("## 알고리즘 분류\n\n");
    for tag in &p.tags {
        readme.push_str(&format!("- {}\n", tag));
    }
    readme
}
