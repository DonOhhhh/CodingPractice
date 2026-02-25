use std::env;
use std::fs;
use std::path::Path;
use runner::{find_project_root, compile, get_test_cases, run_exec};

fn main() {
    // Parse source file path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: runner <source_file>");
        std::process::exit(1);
    }
    let source_path = Path::new(&args[1]);

    // Get Exec Path
    let project_root = find_project_root();
    let exec_path = project_root.join("bin/temp_exec");
    
    if !project_root.join("bin").exists() {
        fs::create_dir_all(project_root.join("bin")).expect("Failed to create bin dir");
    }

    // Compile
    if let Err(err) = compile(source_path, &project_root, &exec_path) {
        eprintln!("\x1b[31mCompilation Failed:\x1b[0m\n{}", err);
        std::process::exit(1);
    }
    println!("Compilation successful.");

    // Get test cases
    let problem_dir = source_path.canonicalize().unwrap().parent().unwrap().to_path_buf();
    let test_cases = get_test_cases(&problem_dir.join("data"));

    if test_cases.is_empty() {
        println!("No test cases found in {}, skipping tests.", problem_dir.join("data").display());
        return;
    }

    // Compare test cases
    let mut all_pass = true;
    for (input, output) in test_cases {
        let file_stem = input.file_stem().unwrap().to_str().unwrap();
        print!("Test Case {}: ", file_stem);

        match run_exec(&exec_path, &input) {
            Ok(result) => {
                if !result.success {
                    println!("\x1b[31mRuntime Error\x1b[0m (Exit code: {})", result.code);
                    if !result.stderr.is_empty() { println!("  Stderr: {}", result.stderr); }
                    all_pass = false;
                    continue;
                }

                let actual = result.stdout.trim_end();
                let expected = fs::read_to_string(output).unwrap_or_default();
                let expected = expected.trim_end();

                if actual == expected {
                    println!("\x1b[32mPASS\x1b[0m");
                } else {
                    println!("\x1b[31mFAIL\x1b[0m");
                    println!("  Expected: {:?}", expected);
                    println!("  Actual:   {:?}", actual);
                    all_pass = false;
                }
            }
            Err(e) => {
                println!("\x1b[31mExecution Error\x1b[0m: {}", e);
                all_pass = false;
            }
        }
    }

    if all_pass {
        println!("\n\x1b[32mAll test cases passed!\x1b[0m");
    } else {
        println!("\n\x1b[31mSome test cases failed.\x1b[0m");
        std::process::exit(1);
    }
}
