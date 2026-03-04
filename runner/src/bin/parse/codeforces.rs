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
    fn get_problem_code(&self, url: &str) -> String;
}

pub struct Codeforces;

impl Codeforces {
    pub fn parse(&self, html: &str, url: &str) -> anyhow::Result<Problem> {
        let document = Html::parse_document(html);
        let mut problem = Problem::default();
        problem.url = url.to_string();
        problem.code = self.get_problem_code(url);

        // 제목 추출
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

        // 시간/메모리 제한
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

        // 본문 설명
        let body_selector = Selector::parse(".problem-statement > div:not(.header):not(.input-specification):not(.output-specification):not(.sample-tests):not(.note)").unwrap();
        if let Some(desc_elem) = document.select(&body_selector).next() {
            problem.description = node_to_markdown(desc_elem.into());
        }

        // 입력 사양
        let ispec_sel = Selector::parse(".input-specification").unwrap();
        if let Some(input_elem) = document.select(&ispec_sel).next() {
            problem.input_spec = node_to_markdown(input_elem.into());
            problem.input_spec = problem.input_spec.replace("Input", "").trim().to_string();
        }

        // 출력 사양
        let ospec_sel = Selector::parse(".output-specification").unwrap();
        if let Some(output_elem) = document.select(&ospec_sel).next() {
            problem.output_spec = node_to_markdown(output_elem.into());
            problem.output_spec = problem.output_spec.replace("Output", "").trim().to_string();
        }

        // 예제 테스트
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

        // 노트
        let note_sel = Selector::parse(".note").unwrap();
        if let Some(note_elem) = document.select(&note_sel).next() {
            problem.note = node_to_markdown(note_elem.into());
            problem.note = problem.note.replace("Note", "").trim().to_string();
        }

        // 티어 및 태그
        problem.tier = extract_tier_from_doc(&document);
        let all_tags = extract_tags_from_doc(&document);
        problem.tags = all_tags.into_iter().filter(|t| !t.starts_with('*')).collect();

        Ok(problem)
    }
}

impl Parser for Codeforces {
    fn parse_to_readme(&self, html: &str, url: &str) -> anyhow::Result<String> {
        let problem = self.parse(html, url)?;
        Ok(generate_readme(&problem))
    }

    fn get_problem_code(&self, url: &str) -> String {
        if let Some(pos) = url.find("/problem/") {
            let code_part = &url[pos + 9..];
            code_part.replace("/", "")
        } else {
            String::new()
        }
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
    let result = node_to_markdown_internal(node, false, false, false, false);
    
    let res = result.replace("&nbsp;", " ")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"");
    
    // 공백 정리
    let re_spaces = Regex::new(r" +").unwrap();
    let res = re_spaces.replace_all(&res, " ").to_string();

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

fn node_to_markdown_internal(
    node: scraper::element_ref::ElementRef,
    in_math: bool,
    in_tt: bool,
    in_bf: bool,
    in_it: bool,
) -> String {
    let mut result = String::new();
    for child in node.children() {
        match child.value() {
        Node::Text(text) => {
            let text_val = text.replace("$$$", "$").replace('\n', " ").replace('\t', " ");
            if in_math {
                // 실제 LaTeX 모드 내부라면 그대로 텍스트 추가
                result.push_str(&text_val);
            } else if in_tt || in_bf || in_it {
                // 스타일 중첩 처리 알고리즘 (공백 보존 버전)
                let has_leading = text_val.starts_with(' ');
                let has_trailing = text_val.ends_with(' ');
                let content = text_val.trim();
                
                if !content.is_empty() {
                    let mut wrapped = content.to_string();
                    if in_tt { wrapped = format!("${}$", wrapped); }
                    if in_bf { wrapped = format!("**{}**", wrapped); }
                    if in_it { wrapped = format!("*{}*", wrapped); }
                    
                    if has_leading { result.push_str(" "); }
                    result.push_str(&wrapped);
                    if has_trailing { result.push_str(" "); }
                } else if has_leading || has_trailing {
                    result.push_str(" ");
                }
            } else {
                result.push_str(&text_val);
            }
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
                    if in_math {
                        result.push_str(script_text.trim());
                    } else {
                        result.push_str(&format!("${}$", script_text.trim()));
                    }
                    continue;
                }
                
                let child_node = scraper::ElementRef::wrap(child).unwrap();
                
                if name == "table" {
                    result.push_str("\n");
                    let mut rows = Vec::new();
                    let tr_sel = Selector::parse("tr").unwrap();
                    for tr in child_node.select(&tr_sel) {
                        let mut cells = Vec::new();
                        let td_sel = Selector::parse("td, th").unwrap();
                        for td in tr.select(&td_sel) {
                            cells.push(node_to_markdown_internal(td, in_math, in_tt, in_bf, in_it).trim().to_string());
                        }
                        if !cells.is_empty() {
                            rows.push(cells);
                        }
                    }
                    
                    for (i, row) in rows.iter().enumerate() {
                        result.push_str("| ");
                        result.push_str(&row.join(" | "));
                        result.push_str(" |\n");
                        if i == 0 {
                            result.push_str("|");
                            for _ in 0..row.len() {
                                result.push_str(" :--- |");
                            }
                            result.push_str("\n");
                        }
                    }
                    result.push_str("\n");
                    continue;
                }

                let is_math = class.contains("tex-span");
                let is_tt = class.contains("tex-font-style-tt");
                let is_bf = class.contains("tex-font-style-bf");
                let is_it = class.contains("tex-font-style-it");

                let child_res = node_to_markdown_internal(
                    child_node,
                    in_math || is_math,
                    in_tt || is_tt,
                    in_bf || is_bf,
                    in_it || is_it,
                );

                if !in_math && is_math {
                    result.push_str(&format!("${}$", child_res.trim()));
                } else if name == "p" {
                    result.push_str("\n\n");
                    result.push_str(&child_res);
                    result.push_str("\n\n");
                } else if name == "li" {
                    result.push_str(&format!("- {}\n", child_res.trim()));
                } else if name == "ul" {
                    result.push_str("\n");
                    result.push_str(&child_res);
                    result.push_str("\n");
                } else if name == "br" {
                    result.push_str("\n");
                } else {
                    result.push_str(&child_res);
                }
            }
            _ => {}
        }
    }
    result
}

fn extract_tier_from_doc(document: &Html) -> String {
    let selector = Selector::parse("span[title='Difficulty']").unwrap();
    if let Some(tier_elem) = document.select(&selector).next() {
        tier_elem.text().collect::<String>().trim().replace("*", "")
    } else {
        "알 수 없음".to_string()
    }
}

fn extract_tags_from_doc(document: &Html) -> Vec<String> {
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
