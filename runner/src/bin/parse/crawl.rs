use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Network::CookieParam};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;
use anyhow::Context;

#[path = "category.rs"]
pub mod category;
use category::Category;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebData {
    pub cookies: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetcherKind {
    Cookie,
    Cdp,
}

pub trait HtmlFetcher {
    fn fetch_html(&self, target_url: &str, category: Category) -> anyhow::Result<String>;
}

pub struct CookieFetcher;
pub struct CdpFetcher;

impl HtmlFetcher for CookieFetcher {
    fn fetch_html(&self, target_url: &str, category: Category) -> anyhow::Result<String> {
        let file_path = category.cookie_path();
        if !Path::new(&file_path).exists() {
            anyhow::bail!("쿠키 파일을 찾을 수 없습니다: {}", file_path);
        }

        println!("CookieFetcher [{}]: 쿠키 로드 중 (경로: {})...", category, file_path);
        let json_content = fs::read_to_string(file_path)?;
        let web_data: WebData = serde_json::from_str(&json_content)?;

        let options = LaunchOptions::default_builder()
            .headless(true)
            .args(vec![
                std::ffi::OsStr::new("--no-sandbox"),
                std::ffi::OsStr::new("--disable-gpu"),
                std::ffi::OsStr::new("--disable-dev-shm-usage"),
                std::ffi::OsStr::new("--user-agent=Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
            ])
            .build()
            .context("브라우저 실행 옵션 설정 실패")?;
        let browser = Browser::new(options).context("브라우저 인스턴스 생성 실패")?;
        let tab = browser.new_tab().context("새 탭 열기 실패")?;

        println!("CookieFetcher: 쿠키 설정 중...");
        let domain = match category {
            Category::Codeforces => ".codeforces.com",
            Category::BOJ => ".acmicpc.net",
            Category::LeetCode => ".leetcode.com",
        };

        for (name, value) in &web_data.cookies {
            let value = if name == "lastOnlineTimeUpdaterInvocation" {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis().to_string())
                    .unwrap_or_else(|_| value.clone())
            } else {
                value.clone()
            };

            tab.set_cookies(vec![CookieParam {
                name: name.clone(),
                value,
                url: Some(format!("https://{}", domain.trim_start_matches('.'))),
                domain: Some(domain.to_string()),
                path: Some("/".to_string()),
                secure: Some(true),
                http_only: None,
                same_site: None,
                expires: None,
                priority: None,
                same_party: None,
                source_scheme: None,
                source_port: None,
                partition_key: None,
            }]).context(format!("쿠키 설정 실패: {}", name))?;
        }

        println!("CookieFetcher: {} 이동 중...", target_url);
        tab.navigate_to(target_url).context(format!("URL 이동 실패: {}", target_url))?;
        
        // wait_until_navigated 대신 요소 대기 방식 사용
        println!("CookieFetcher: 페이지 로딩 대기 중 (최대 15초)...");
        let wait_res = tab.wait_for_element(".problem-statement");
        if wait_res.is_err() {
            println!("경고: .problem-statement 요소를 찾지 못했습니다. 5초 추가 대기 후 시도합니다...");
            std::thread::sleep(Duration::from_secs(5));
        }
        
        let html_content = tab.get_content().context("HTML 컨텐츠 추출 실패")?;
        
        if !html_content.contains("problem-statement") {
            if html_content.contains("Cloudflare") || html_content.contains("Checking your browser") {
                anyhow::bail!("Cloudflare 차단에 걸렸습니다. 쿠키를 갱신하거나 CDP 방식을 시도해보세요.");
            }
            anyhow::bail!("문제 내용을 찾을 수 없습니다. (쿠키 만료 또는 잘못된 URL)");
        }
        
        Ok(html_content)
    }
}

impl HtmlFetcher for CdpFetcher {
    fn fetch_html(&self, target_url: &str, _category: Category) -> anyhow::Result<String> {
        println!("CdpFetcher: Cloudflare 우회를 위한 CDP 방식 사용...");
        
        let options = LaunchOptions::default_builder()
            .headless(true)
            .args(vec![
                std::ffi::OsStr::new("--no-sandbox"),
                std::ffi::OsStr::new("--disable-gpu"),
                std::ffi::OsStr::new("--disable-dev-shm-usage"),
                std::ffi::OsStr::new("--user-agent=Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
                std::ffi::OsStr::new("--disable-blink-features=AutomationControlled"),
            ])
            .build()
            .context("브라우저 실행 옵션 설정 실패")?;
        
        let browser = Browser::new(options).context("브라우저 인스턴스 생성 실패")?;
        let tab = browser.new_tab().context("새 탭 열기 실패")?;

        println!("CdpFetcher: {} 이동 및 렌더링 대기 중...", target_url);
        tab.navigate_to(target_url).context(format!("URL 이동 실패: {}", target_url))?;
        
        println!("CdpFetcher: 10초 대기 중...");
        std::thread::sleep(Duration::from_secs(10));
        
        let html_content = tab.get_content().context("HTML 컨텐츠 추출 실패")?;
        Ok(html_content)
    }
}

pub fn get_fetcher(kind: FetcherKind) -> Box<dyn HtmlFetcher> {
    match kind {
        FetcherKind::Cookie => Box::new(CookieFetcher),
        FetcherKind::Cdp => Box::new(CdpFetcher),
    }
}
