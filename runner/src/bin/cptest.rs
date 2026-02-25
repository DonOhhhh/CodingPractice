use clap::Parser;
use std::fs;
use std::io::{self};
use std::path::{PathBuf};
use runner::{find_project_root, compile, run_exec};

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

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let testcase_id = &cli.testcase_id;
    let file_path = &cli.path;

    let absolute_path = file_path.canonicalize()?;
    let project_root = find_project_root();
    let exec_path = project_root.join("bin/temp_exec");
    let parent_dir = absolute_path.parent().unwrap();

    // 1. Compile
    if let Err(err) = compile(&absolute_path, &project_root, &exec_path) {
        eprintln!("\x1b[31mCompilation Failed:\x1b[0m\n{}", err);
        std::process::exit(1);
    }
    println!("INFO: Compilation successful.");

    // 2. Prepare test cases
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

    // 3. Run
    println!("INFO: Running test case #{}...", testcase_id);
    match run_exec(&exec_path, &input_file_path) {
        Ok(result) => {
            if !result.success {
                eprintln!("\n\x1b[31m!!!!!!!!!!!!!!!!!!!! ERROR !!!!!!!!!!!!!!!!!!!!");
                eprintln!("Exit code: {}", result.code);
                if !result.stderr.is_empty() {
                    eprintln!("Stderr: {}", result.stderr);
                }
                eprintln!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\x1b[0m\n");
            }

            let actual_output = result.stdout.trim_end();
            let expected_output = fs::read_to_string(&output_file_path)?;
            let expected_output = expected_output.trim_end();

            println!("--------------------OUTPUT--------------------");
            println!("{}", actual_output);
            println!("----------------------------------------------");

            if actual_output == expected_output {
                println!("✅ \x1b[32mTest Case Passed!\x1b[0m");
            } else {
                println!("❌ \x1b[31mTest Case Failed!\x1b[0m");
                println!("\n--- Expected Output ---\n{}", expected_output);
                println!("\n--- Actual Output ---\n{}", actual_output);
            }
        }
        Err(e) => {
            eprintln!("\x1b[31mExecution Error\x1b[0m: {}", e);
        }
    }

    Ok(())
}
