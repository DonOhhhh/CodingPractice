use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: runner <source_file>");
        std::process::exit(1);
    }

    let source_path = Path::new(&args[1]);
    if !source_path.exists() {
        eprintln!("Error: File not found: {}", source_path.display());
        std::process::exit(1);
    }

    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    // Find project root to locate "bin" directory correctly
    let mut project_root = env::current_dir().expect("Failed to get current directory");
    loop {
        if project_root.join("runner").join("Cargo.toml").exists() {
            break;
        }
        if !project_root.pop() {
            // Reached root without finding project root; fallback to CWD or user home?
            // Fallback to original execution directory if we can't find the pattern
            project_root = env::current_dir().expect("Failed to get current directory");
            break;
        }
    }

    // Ensure bin directory exists at project root
    let bin_dir = project_root.join("bin");
    if !bin_dir.exists() {
        fs::create_dir(&bin_dir).expect("Failed to create bin directory");
    }

    let exec_path = bin_dir.join("temp_exec");

    // Compilation
    println!("Compiling {}...", source_path.display());
    let compile_status = match extension {
        "cpp" => Command::new("g++")
            .args(&[
                "-std=c++23",
                "-O2",
                "-Wall",
                source_path.to_str().unwrap(),
                "-o",
                exec_path.to_str().unwrap(),
            ])
            .status(),
        "rs" => Command::new("rustc")
            .args(&[
                source_path.to_str().unwrap(),
                "-o",
                exec_path.to_str().unwrap(),
            ])
            .status(),
        _ => {
            eprintln!("Error: Unsupported extension .{}", extension);
            std::process::exit(1);
        }
    };

    match compile_status {
        Ok(status) if status.success() => println!("Compilation successful."),
        _ => {
            eprintln!("Compilation failed.");
            std::process::exit(1);
        }
    }

    // Execution & Grading
    // Ensure we handle relative paths for source_path correctly if we changed directory?
    // Actually we didn't change CWD, just calculated paths.
    // source_path is likely relative to CWD.
    // parent_dir logic relies on source_path.

    // If source_path is relative, it needs to be canonicalized or treated relative to original CWD.
    // source_path.parent() works on relative paths.
    
    // BUT: If the user passes problems/BOJ/1000/solution.cpp, and we are in root, it works.
    // If user is in problems/BOJ/1000 and passes solution.cpp:
    // source_path is "solution.cpp". parent is "". data is "data". Correct.
    
    let parent_dir = if source_path.is_absolute() {
        source_path.parent().unwrap().to_path_buf()
    } else {
        let current = env::current_dir().unwrap();
        current.join(source_path).parent().unwrap().to_path_buf()
    };
    
    let data_dir = parent_dir.join("data");

    if !data_dir.exists() {
        println!("No data directory found at {}, skipping tests.", data_dir.display());
        return;
    }

    let mut input_files: Vec<PathBuf> = fs::read_dir(&data_dir)
        .expect("Failed to read data directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "in"))
        .collect();

    // Sort files to ensure deterministic order (1.in, 2.in, etc.)
    // Simple string sort for now; numerical sort would be better but this is MVP
    input_files.sort();

    for input_file in input_files {
        let file_stem = input_file.file_stem().unwrap().to_str().unwrap();
        let output_file = data_dir.join(format!("{}.out", file_stem));

        if !output_file.exists() {
            println!("Skipping {}: No matching .out file", input_file.display());
            continue;
        }

        print!("Test Case {}: ", file_stem);

        let input_file_handle = fs::File::open(&input_file).expect("Failed to open input file");
        
        // Execute the binary
        let output = Command::new(&exec_path)
            .stdin(Stdio::from(input_file_handle))
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute binary");

        let expected_output = fs::read_to_string(&output_file).expect("Failed to read output file");
        
        let actual_str = String::from_utf8_lossy(&output.stdout);
        let expected_trimmed = expected_output.trim();
        let actual_trimmed = actual_str.trim();

        if actual_trimmed == expected_trimmed {
            println!("\x1b[32mPASS\x1b[0m"); // Green PASS
        } else {
            println!("\x1b[31mFAIL\x1b[0m"); // Red FAIL
            println!("  Expected: {:?}", expected_trimmed);
            println!("  Actual:   {:?}", actual_trimmed);
        }
    }
    
    // Cleanup (Optional)
    // fs::remove_file(exec_path).ok();
}
