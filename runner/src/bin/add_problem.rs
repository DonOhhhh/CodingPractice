use anyhow::{Context, Result};
use dialoguer::{Input, theme::ColorfulTheme};
use std::fs;
use std::io::{self, BufRead};
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
    println!("Select a problem category:");
    for (i, category) in categories.iter().enumerate() {
        println!("{}: {}", i + 1, category);
    }

    let selection: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the number for the category")
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input > 0 && *input <= categories.len() {
                Ok(())
            } else {
                Err("Invalid number")
            }
        })
        .interact_text()?;

    let selected_category = &categories[selection - 1]; // Adjust for 0-based index

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

    // 4. Select Test Case Count
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

    // 5. Collect Input/Output for each test case
    let mut examples = Vec::new();
    for i in 1..=test_case_count {
        println!("\nCollecting data for Example #{}", i);
        
        println!("Enter Input (press Enter twice to finish):");
        let input_data = read_multiline()?;

        println!("Enter Output (press Enter twice to finish):");
        let output_data = read_multiline()?;

        examples.push((input_data, output_data));
    }

    // 6. Create Directory Structure
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

    // Generate examples section
    let mut examples_section = String::new();
    for (i, (input_data, output_data)) in examples.iter().enumerate() {
        let example_num = i + 1;
        examples_section.push_str(&format!(
            "### {}\n\n#### 입력\n\n```\n{}\n```\n\n#### 출력\n\n```\n{}\n```\n\n",
            example_num, input_data, output_data
        ));
    }

    // Replace placeholders in README
    let content_readme = content_readme
        .replace("{category}", selected_category)
        .replace("{problem_number}", &problem_number)
        .replace("{examples}", &examples_section);

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

    for (i, (input_data, output_data)) in examples.iter().enumerate() {
        let example_num = i + 1;
        create_file(&data_dir.join(format!("{}.in", example_num)), input_data)?;
        create_file(&data_dir.join(format!("{}.out", example_num)), output_data)?;
    }

    println!(
        "Successfully created problem {} in {:?}",
        problem_number, problem_dir
    );

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
        
        // EOF or empty line (just newline) signals end
        if bytes_read == 0 || line.trim().is_empty() {
            break;
        }
        
        // Remove the newline character for storage
        lines.push(line.trim_end().to_string());
    }
    
    Ok(lines.join("\n"))
}
