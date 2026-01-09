use anyhow::{Context, Result};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    // Define the root path for problems
    // We use absolute path to ensure it works regardless of where the command is run
    let root_path = PathBuf::from("/home/codefreak/CodingPractice/problems");

    if !root_path.exists() {
        eprintln!("Error: Problems directory not found at {:?}", root_path);
        return Ok(());
    }

    // 1. Get Categories
    let mut categories: Vec<String> = fs::read_dir(&root_path)?
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

    categories.sort();

    if categories.is_empty() {
        eprintln!("No categories found in {:?}", root_path);
        return Ok(());
    }

    // 2. Select Category
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a problem category")
        .default(0)
        .items(&categories)
        .interact()
        .context("Failed to select category")?;

    let selected_category = &categories[selection];

    // 3. Input Problem Number
    let problem_number: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter problem number (max 10 chars)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() > 10 {
                return Err("Problem number must be 10 characters or less");
            }
            if !input.chars().all(char::is_alphanumeric) {
                return Err(
                    "Problem number must be alphanumeric (no special characters or spaces)",
                );
            }
            Ok(())
        })
        .interact_text()
        .context("Failed to read problem number")?;

    // 4. Create Directory Structure
    let problem_dir = root_path.join(selected_category).join(&problem_number);
    if problem_dir.exists() {
        println!("Problem directory already exists: {:?}", problem_dir);
        return Ok(());
    }

    fs::create_dir_all(&problem_dir)
        .with_context(|| format!("Failed to create directory {:?}", problem_dir))?;

    // Template paths
    let runner_dir = root_path.parent().unwrap().join("runner");
    let templates_dir = runner_dir.join("templates");

    // Load templates
    let content_readme = fs::read_to_string(templates_dir.join("README.md"))
        .unwrap_or_else(|_| format!("# [{} {} : ]()\n\n## 문제 설명\n\n\n\n## 입력\n\n\n\n## 출력\n\n|\n\n## 예제\n\n| 입력 | 출력 |\n| :-| :- |\n| | |\n| | |\n\n## 티어\n\n\n\n## 제한\n\n|시간|메모리|\n|---|---|\n|1초|256MB|\n\n## 알고리즘 분류\n\n\n", selected_category, problem_number));

    // Replace placeholders in README
    let content_readme = content_readme
        .replace("{category}", selected_category)
        .replace("{problem_number}", &problem_number);

    let content_rs = fs::read_to_string(templates_dir.join("solution.rs"))
        .unwrap_or_else(|_| "fn main() {\n    println!(\"Hello, world!\");\n}\n".to_string());

    let content_cpp = fs::read_to_string(templates_dir.join("solution.cpp"))
        .unwrap_or_else(|_| "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, world!\" << std::endl;\n    return 0;\n}\n".to_string());

    // Create files
    create_file(&problem_dir.join("README.md"), &content_readme)?;
    create_file(&problem_dir.join("solution.rs"), &content_rs)?;
    create_file(&problem_dir.join("solution.cpp"), &content_cpp)?;

    // Create data directory
    let data_dir = problem_dir.join("data");
    fs::create_dir_all(&data_dir)?;

    create_file(&data_dir.join("1.in"), "")?;
    create_file(&data_dir.join("1.out"), "")?;
    create_file(&data_dir.join("2.in"), "")?;
    create_file(&data_dir.join("2.out"), "")?;

    println!(
        "Successfully created problem {} in {:?}",
        problem_number, problem_dir
    );

    Ok(())
}

fn create_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to create file {:?}", path))
}
