use std::env;
use std::fs;
use std::io::Read;
use std::mem;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ExitStatus};
use std::time::Instant;
use regex::Regex;

/// Represents the resource limits for a problem.
#[derive(Debug, Clone, Copy)]
struct Limits {
    time_seconds: u64,
    memory_mb: u64,
}

impl Default for Limits {
    fn default() -> Self {
        // Default limits if not specified in README.md
        Self {
            time_seconds: 1,
            memory_mb: 256,
        }
    }
}

/// Parses the time and memory limits from the problem's README.md file.
fn parse_limits_from_readme(problem_dir: &Path) -> Limits {
    let readme_path = problem_dir.join("README.md");
    if !readme_path.exists() {
        println!("INFO: README.md not found, using default limits.");
        return Limits::default();
    }

    let content = fs::read_to_string(readme_path).unwrap_or_default();
    let re = Regex::new(r"(?is)##\s*제한.*?\s*\|\s*(\d+)\s*초\s*\|\s*(\d+)\s*MB\s*\|").unwrap();

    if let Some(caps) = re.captures(&content) {
        let time_seconds = caps.get(1).map_or("1", |m| m.as_str()).parse().unwrap_or(1);
        let memory_mb = caps.get(2).map_or("256", |m| m.as_str()).parse().unwrap_or(256);
        println!("INFO: Found limits: {}s, {}MB", time_seconds, memory_mb);
        Limits { time_seconds, memory_mb }
    } else {
        println!("INFO: Limits table not found in README.md, using default limits.");
        Limits::default()
    }
}

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

    let extension = source_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let mut project_root = env::current_dir().expect("Failed to get current directory");
    loop {
        if project_root.join("runner").join("Cargo.toml").exists() { break; }
        if !project_root.pop() {
            project_root = env::current_dir().expect("Failed to get current directory");
            break;
        }
    }

    let bin_dir = project_root.join("bin");
    if !bin_dir.exists() { fs::create_dir(&bin_dir).expect("Failed to create bin directory"); }
    let exec_path = bin_dir.join("temp_exec");

    println!("Compiling {}...", source_path.display());
    let compile_status = match extension {
        "cpp" => Command::new("g++").args(&["-std=c++23", "-O2", "-Wall", source_path.to_str().unwrap(), "-o", exec_path.to_str().unwrap()]).status(),
        "rs" => Command::new("rustc").args(&[source_path.to_str().unwrap(), "-o", exec_path.to_str().unwrap()]).status(),
        _ => { eprintln!("Error: Unsupported extension .{}", extension); std::process::exit(1); }
    };

    match compile_status {
        Ok(status) if status.success() => println!("Compilation successful."),
        _ => { eprintln!("Compilation failed."); std::process::exit(1); }
    }

    let parent_dir = if source_path.is_absolute() { source_path.parent().unwrap().to_path_buf() } else { env::current_dir().unwrap().join(source_path).parent().unwrap().to_path_buf() };
    let limits = parse_limits_from_readme(&parent_dir);
    let data_dir = parent_dir.join("data");

    if !data_dir.exists() {
        println!("No data directory found at {}, skipping tests.", data_dir.display());
        return;
    }

    let mut input_files: Vec<PathBuf> = fs::read_dir(&data_dir).expect("Failed to read data directory").filter_map(|entry| entry.ok()).map(|entry| entry.path()).filter(|path| path.extension().map_or(false, |ext| ext == "in")).collect();
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
        
        let start_time = Instant::now();
        let mut command = Command::new(&exec_path);
        unsafe {
            let time_limit = limits.time_seconds;
            let mem_limit = limits.memory_mb * 1024 * 1024; // MB to Bytes

            command.pre_exec(move || {
                let rlimit_cpu = libc::rlimit { rlim_cur: time_limit, rlim_max: time_limit + 1 };
                if libc::setrlimit(libc::RLIMIT_CPU, &rlimit_cpu) != 0 { return Err(std::io::Error::last_os_error()); }
                let rlimit_as = libc::rlimit { rlim_cur: mem_limit, rlim_max: mem_limit };
                if libc::setrlimit(libc::RLIMIT_AS, &rlimit_as) != 0 { return Err(std::io::Error::last_os_error()); }
                Ok(())
            });
        }
        
        let mut child = command
            .stdin(Stdio::from(input_file_handle))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute binary");

        let mut stdout_buf = Vec::new();
        let mut stderr_buf = Vec::new();

        let mut stdout_handle = child.stdout.take().unwrap();
        let mut stderr_handle = child.stderr.take().unwrap();

        let stdout_thread = std::thread::spawn(move || {
            stdout_handle.read_to_end(&mut stdout_buf).unwrap();
            stdout_buf
        });
        let stderr_thread = std::thread::spawn(move || {
            stderr_handle.read_to_end(&mut stderr_buf).unwrap();
            stderr_buf
        });

        let mut status: i32 = 0;
        let mut rusage = unsafe { mem::zeroed() };
        let pid = child.id() as libc::pid_t;

        unsafe { libc::wait4(pid, &mut status, 0, &mut rusage) };
        
        let elapsed_time = start_time.elapsed();
        let exit_status: ExitStatus = ExitStatusExt::from_raw(status);
        
        let peak_mem_kb = rusage.ru_maxrss; // On Linux, this is in KB
        let peak_mem_mb = peak_mem_kb as f64 / 1024.0;

        if let Some(signal) = exit_status.signal() {
            if signal == libc::SIGXCPU {
                println!("\x1b[31mTime Limit Exceeded\x1b[0m ( > {}s )", limits.time_seconds);
            } else {
                println!("\x1b[31mRuntime Error (Signal {})\x1b[0m", signal);
            }
            continue;
        }

        if !exit_status.success() {
             println!("\x1b[31mRuntime Error\x1b[0m (Exit code: {})", exit_status.code().unwrap_or(-1));
             let stderr_bytes = stderr_thread.join().unwrap();
             let stderr = String::from_utf8_lossy(&stderr_bytes);
             if !stderr.is_empty() {
                 println!("  Stderr: {}", stderr);
             }
             continue;
        }

        let stdout_bytes = stdout_thread.join().unwrap();
        let actual_str = String::from_utf8_lossy(&stdout_bytes);
        let expected_output = fs::read_to_string(&output_file).expect("Failed to read output file");
        let expected_trimmed = expected_output.trim_end();
        let actual_trimmed = actual_str.trim_end();

        if actual_trimmed == expected_trimmed {
            println!("\x1b[32mPASS\x1b[0m (Time: {:.2?}, Memory: {:.2} MB)", elapsed_time, peak_mem_mb);
        } else {
            println!("\x1b[31mFAIL\x1b[0m (Time: {:.2?}, Memory: {:.2} MB)", elapsed_time, peak_mem_mb);
            println!("  Expected: {:?}", expected_trimmed);
            println!("  Actual:   {:?}", actual_trimmed);
        }
    }
}
