use clap::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A Rust-based test runner for competitive programming.
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Compiles a solution file and runs it against a specific test case, comparing the output against an expected answer.")]
struct Cli {
    /// The ID of the test case to run (e.g., '1')
    testcase_id: String,

    /// Path to the solution file to test
    #[arg(value_name = "FILE_PATH")]
    path: PathBuf,
}

fn find_project_root(start_path: &Path) -> io::Result<PathBuf> {
    let mut current = start_path;
    loop {
        if current.join(".git").is_dir() && current.join("problems").is_dir() {
            return Ok(current.to_path_buf());
        }
        current = match current.parent() {
            Some(p) => p,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "Could not find project root.")),
        };
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let testcase_id = &cli.testcase_id;
    let file_path = &cli.path;

    let absolute_path = file_path.canonicalize()?;
    if !absolute_path.is_file() {
        eprintln!("Error: File not found: {}", absolute_path.display());
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let project_root = find_project_root(&absolute_path)?;
    let output_path = project_root.join("bin").join("solution");
    let parent_dir = absolute_path.parent().unwrap();
    let file_name = absolute_path.file_name().unwrap();
    let extension = absolute_path.extension().unwrap_or_default();

    // 2. Compile the solution
    println!("INFO: Compiling {}...", file_name.to_string_lossy());
    let compile_output = if extension == "rs" {
        Command::new("rustc").current_dir(parent_dir).arg("-O").arg(file_name).arg("-o").arg(&output_path).output()
    } else if extension == "cpp" {
        Command::new("g++").current_dir(parent_dir).args(["-std=c++23", "-O2", "-Wall"]).arg(file_name).arg("-o").arg(&output_path).output()
    } else {
        eprintln!("Error: Unsupported file extension: {:?}", extension.to_string_lossy());
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file type"));
    };
    
    match compile_output {
        Ok(output) if output.status.success() => println!("INFO: Compilation successful."),
        Ok(output) => {
            eprintln!("Error: Compilation failed.");
            io::stderr().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error: Failed to execute compiler: {}", e);
            return Err(e);
        }
    }

    // 3. Prepare test case paths
    let input_file_path = parent_dir.join("data").join(format!("{}.in", testcase_id));
    let output_file_path = parent_dir.join("data").join(format!("{}.out", testcase_id));

    if !input_file_path.is_file() {
        eprintln!("Error: Input file not found: {}", input_file_path.display());
        return Ok(());
    }
    if !output_file_path.is_file() {
        eprintln!("Error: Output file not found: {}", output_file_path.display());
        return Ok(());
    }

    let input_data = fs::read(&input_file_path)?;

    // 4. Run the executable with piped I/O
    println!("INFO: Running test case #{}...", testcase_id);
    let mut child = Command::new(&output_path)
        .current_dir(parent_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(&input_data)?;
    drop(stdin);

    let result = child.wait_with_output()?;
    let actual_output_raw = result.stdout;
    let actual_output = String::from_utf8_lossy(&actual_output_raw);
    
    println!("--------------------OUTPUT--------------------");
    print!("{}", actual_output);
    println!("----------------------------------------------");
    
    // 5. Compare with expected output
    let expected_output_raw = fs::read(&output_file_path)?;
    let expected_output = String::from_utf8_lossy(&expected_output_raw);

    if actual_output.trim_end() == expected_output.trim_end() {
        println!("✅ Test Case Passed!");
    } else {
        println!("❌ Test Case Failed!");
        println!("\n--- Expected Output ---\n{}", expected_output);
        println!("\n--- Actual Output ---\n{}", actual_output);
    }

    Ok(())
}