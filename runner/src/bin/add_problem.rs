use anyhow::{Context, Result};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

// URL 파싱을 위한 모듈 선언
#[path = "parse/codeforces.rs"]
mod codeforces;
#[path = "parse/crawl.rs"]
mod crawl;

// crawl 모듈 내부에서 이미 선언된 category를 사용합니다.
use crawl::category::Category;
use crawl::{FetcherKind};
use codeforces::{Parser, Codeforces};

fn main() -> Result<()> {
    let root_path = PathBuf::from("/home/codefreak/CodingPractice/problems");

    if !root_path.exists() {
        eprintln!("Error: Problems directory not found at {:?}", root_path);
        return Ok(());
    }

    // 0. Select Input Method
    let methods = vec!["Parse from URL", "Manual Input"];
    let method_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select input method")
        .default(0)
        .items(&methods)
        .interact()
        .context("Failed to select method")?;

    let category: Category;
    let problem_number: String;
    let examples: Vec<(String, String)>;
    let parsed_readme: Option<String>;

    if method_selection == 0 {
        // URL Path
        let target_url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter problem URL")
            .default("https://...".to_string())
            .interact_text()
            .context("Failed to read URL")?;

        let fetcher = crawl::get_fetcher(FetcherKind::Cookie);

        if target_url.contains("codeforces.com") {
            category = Category::Codeforces;
            println!("Fetching and parsing problem from {}...", category);
            
            let html_content = fetcher.fetch_html(&target_url, category)?;
            let parser = Codeforces;
            let problem = parser.parse(&html_content, &target_url)?;
            
            problem_number = problem.code;
            examples = problem.examples;
            parsed_readme = Some(parser.parse_to_readme(&html_content, &target_url)?);
            
            println!("Successfully parsed problem: {}", problem_number);
        } else {
            anyhow::bail!("Unsupported site URL");
        }
    } else {
        // Manual Path
        let mut categories_dirs: Vec<String> = fs::read_dir(&root_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    path.file_name()?.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect();

        categories_dirs.sort();

        if categories_dirs.is_empty() {
            eprintln!("No categories found in {:?}", root_path);
            return Ok(());
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a problem category")
            .default(0)
            .items(&categories_dirs)
            .interact()
            .context("Failed to select category")?;

        let selected_dir = &categories_dirs[selection];
        category = match selected_dir.to_lowercase().as_str() {
            "codeforces" => Category::Codeforces,
            "boj" | "acmicpc" => Category::BOJ,
            "leetcode" => Category::LeetCode,
            _ => anyhow::bail!("Unsupported category directory: {}", selected_dir),
        };

        problem_number = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter problem number (max 10 chars)")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.len() > 10 {
                    return Err("Problem number must be 10 characters or less");
                }
                if !input.chars().all(char::is_alphanumeric) {
                    return Err("Problem number must be alphanumeric");
                }
                Ok(())
            })
            .interact_text()
            .context("Failed to read problem number")?;

        let test_case_count: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter number of test cases (1-100)")
            .default(1)
            .validate_with(|input: &usize| -> Result<(), &str> {
                if *input < 1 || *input > 100 {
                    return Err("Number of test cases must be between 1 and 100");
                }
                Ok(())
            })
            .interact_text()
            .context("Failed to read test case count")?;

        let mut collected_examples = Vec::new();
        for i in 1..=test_case_count {
            println!("\nCollecting data for Example #{}", i);
            println!("Enter Input (press Enter twice to finish):");
            let input_data = read_multiline()?;
            println!("Enter Output (press Enter twice to finish):");
            let output_data = read_multiline()?;
            collected_examples.push((input_data, output_data));
        }
        examples = collected_examples;
        parsed_readme = None;
    }

    let problem_dir = root_path.join(category.as_str()).join(&problem_number);
    if problem_dir.exists() {
        println!("Problem directory already exists: {:?}", problem_dir);
        return Ok(());
    }

    fs::create_dir_all(&problem_dir)?;

    let runner_dir = root_path.parent().unwrap().join("runner");
    let templates_dir = runner_dir.join("templates");

    let content_readme = if let Some(readme) = parsed_readme {
        readme
    } else {
        let template = fs::read_to_string(templates_dir.join("README.md"))
            .unwrap_or_else(|_| "# [{category} {problem_number} : ]()\n\n{examples}\n".to_string());
        
        let mut examples_section = String::new();
        for (i, (input_data, output_data)) in examples.iter().enumerate() {
            examples_section.push_str(&format!(
                "### {}\n\n#### 입력\n\n```\n{}\n```\n\n#### 출력\n\n```\n{}\n```\n\n",
                i + 1, input_data, output_data
            ));
        }
        
        template
            .replace("{category}", category.as_str())
            .replace("{problem_number}", &problem_number)
            .replace("{examples}", &examples_section)
    };

    let content_rs = fs::read_to_string(templates_dir.join("solution.rs")).unwrap_or_default();
    let content_cpp = fs::read_to_string(templates_dir.join("solution.cpp")).unwrap_or_default();

    create_file(&problem_dir.join("README.md"), &content_readme)?;
    create_file(&problem_dir.join("solution.rs"), &content_rs)?;
    create_file(&problem_dir.join("solution.cpp"), &content_cpp)?;

    let data_dir = problem_dir.join("data");
    fs::create_dir_all(&data_dir)?;

    for (i, (input_data, output_data)) in examples.iter().enumerate() {
        create_file(&data_dir.join(format!("{}.in", i + 1)), input_data)?;
        create_file(&data_dir.join(format!("{}.out", i + 1)), output_data)?;
    }

    println!("\nSuccessfully created problem {} in {:?}", problem_number, problem_dir);
    Ok(())
}

fn create_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to create file {:?}", path))
}

fn read_multiline() -> Result<String> {
    let mut lines = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        let mut line = String::new();
        let bytes_read = handle.read_line(&mut line)?;
        if bytes_read == 0 || line.trim().is_empty() { break; }
        lines.push(line.trim_end().to_string());
    }
    Ok(lines.join("\n"))
}
