use std::fs;
use dialoguer::{Input, theme::ColorfulTheme};
use crate::codeforces::{Parser, Codeforces};

mod crawl;
mod codeforces;

use crawl::{category, FetcherKind};

fn main() -> anyhow::Result<()> {
    // 1. URL 입력 받기
    let target_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("추출할 문제의 URL을 입력하세요")
        .default("https://codeforces.com/problemset/problem/1989/B".to_string())
        .interact_text()?;

    // CDP 방식은 현재 사용하지 않으므로 기본적으로 FetcherKind::Cookie 사용
    let fetcher = crawl::get_fetcher(FetcherKind::Cookie);

    // 2. 사이트에 따른 파서 선택 및 카테고리 결정
    let (parser, category): (Box<dyn Parser>, category::Category) = if target_url.contains("codeforces.com") {
        (Box::new(Codeforces), category::Category::Codeforces)
    } else {
        anyhow::bail!("지원하지 않는 사이트입니다.");
    };

    // 3. HTML 크롤링 (카테고리 정보 전달)
    let html_content = fetcher.fetch_html(&target_url, category)?;

    // 4. 파싱 및 README 내용 생성
    println!("HTML 파싱 및 README 생성 중...");
    let readme_content = parser.parse_to_readme(&html_content, &target_url)?;

    // 5. 파일 저장
    let output_path = "resources/README.md";
    fs::write(output_path, readme_content)?;
    println!("성공! 결과가 {}에 저장되었습니다.", output_path);

    Ok(())
}
