use std::fs;
use std::path::Path;
use dialoguer::{Input, Select, theme::ColorfulTheme, theme::Theme};
use crate::codeforces::{Parser, Codeforces};

mod crawl;
mod codeforces;

use crawl::{category, FetcherKind, HtmlFetcher};

enum Action {
    FetchHtml,
    GenerateReadme,
    Both,
}

fn main() -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    
    // 1. 작업 선택 메뉴
    let actions = vec!["1. HTML 저장 (Fetch)", "2. README 생성 (Parse)", "3. 전체 실행 (Both)"];
    let selection = Select::with_theme(&theme)
        .with_prompt("수행할 작업을 선택하세요")
        .items(&actions)
        .default(2)
        .interact()?;

    let action = match selection {
        0 => Action::FetchHtml,
        1 => Action::GenerateReadme,
        _ => Action::Both,
    };

    let parser: Box<dyn Parser> = Box::new(Codeforces);
    let fetcher = crawl::get_fetcher(FetcherKind::Cookie);

    let theme = ColorfulTheme::default();

    match action {
        Action::FetchHtml => {
            let url = get_url(&theme)?;
            fetch_and_save_html(&url, &*parser, &*fetcher)?;
        }
        Action::GenerateReadme => {
            let (html_path, url) = select_local_html(&theme)?;
            generate_readme_from_file(&html_path, &url, &*parser)?;
        }
        Action::Both => {
            let url = get_url(&theme)?;
            let html_path = fetch_and_save_html(&url, &*parser, &*fetcher)?;
            generate_readme_from_file(&html_path, &url, &*parser)?;
        }
    }

    Ok(())
}

fn get_url(theme: &dyn Theme) -> anyhow::Result<String> {
    Input::with_theme(theme)
        .with_prompt("추출할 문제의 URL을 입력하세요")
        .default("https://codeforces.com/problemset/problem/1989/B".to_string())
        .interact_text()
        .map_err(|e| anyhow::anyhow!(e))
}

fn fetch_and_save_html(url: &str, parser: &dyn Parser, fetcher: &dyn HtmlFetcher) -> anyhow::Result<String> {
    println!("원격 사이트에서 HTML을 가져오는 중...");
    let category = if url.contains("codeforces.com") {
        category::Category::Codeforces
    } else {
        anyhow::bail!("지원하지 않는 사이트입니다.");
    };

    // 원본 HTML 컨텐츠를 그대로 가져옴
    let html_content = fetcher.fetch_html(url, category)?;
    let problem_code = parser.get_problem_code(url);
    
    fs::create_dir_all("resources")?;
    let output_path = format!("resources/{}.html", problem_code);
    
    // 파싱 없이 원본 그대로 저장
    fs::write(&output_path, &html_content)?;
    println!("성공! 원본 HTML이 {}에 저장되었습니다.", output_path);
    
    Ok(output_path)
}

fn select_local_html(theme: &dyn Theme) -> anyhow::Result<(String, String)> {
    let dir = Path::new("resources");
    if !dir.exists() {
        anyhow::bail!("resources 폴더가 존재하지 않습니다.");
    }

    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("html"))
        .collect();

    if entries.is_empty() {
        anyhow::bail!("resources 폴더에 .html 파일이 없습니다.");
    }

    let file_names: Vec<_> = entries.iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let selection = Select::with_theme(theme)
        .with_prompt("README로 변환할 HTML 파일을 선택하세요")
        .items(&file_names)
        .default(0)
        .interact()?;

    let selected_file = &file_names[selection];
    let file_path = format!("resources/{}", selected_file);
    
    // 문제 번호를 추출하여 URL을 유추 (README 링크 생성용)
    let problem_code = selected_file.replace(".html", "");
    let guessed_url = if problem_code.chars().all(|c| c.is_numeric()) {
        format!("https://codeforces.com/problemset/problem/{}/A", problem_code)
    } else {
        let (num, alpha): (String, String) = problem_code.chars().partition(|c| c.is_numeric());
        format!("https://codeforces.com/problemset/problem/{}/{}", num, alpha)
    };

    Ok((file_path, guessed_url))
}

fn generate_readme_from_file(html_path: &str, url: &str, parser: &dyn Parser) -> anyhow::Result<()> {
    println!("{} 파일로부터 README.md 생성 중...", html_path);
    let saved_html = fs::read_to_string(html_path)?;
    
    // 파싱 로직은 이제 전체 HTML 문서를 대상으로 작동함
    let readme_content = parser.parse_to_readme(&saved_html, url)?;

    let readme_output_path = "resources/README.md";
    fs::write(readme_output_path, readme_content)?;
    println!("성공! 결과가 {}에 저장되었습니다.", readme_output_path);
    Ok(())
}
