use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct ExecResult {
    pub success: bool,
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn find_project_root() -> PathBuf {
    let mut project_root = env::current_dir().expect("Failed to get current directory");
    loop {
        if project_root.join("runner").join("Cargo.toml").exists() {
            return project_root;
        }
        if !project_root.pop() {
            return env::current_dir().expect("Failed to get current directory");
        }
    }
}

pub fn get_rustup_path() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| "/home/codefreak".to_string());
    format!("{}/.cargo/bin/rustup", home)
}

pub fn cleanup_long_type_files(exec_path: &Path) {
    let Some(parent) = exec_path.parent() else { return };
    let Ok(entries) = fs::read_dir(parent) else { return };

    for path in entries.filter_map(Result::ok).map(|e| e.path()) {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else { continue };
        if name.starts_with("temp_exec.long-type-") && name.ends_with(".txt") {
            let _ = fs::remove_file(path);
        }
    }
}

pub fn compile(source_path: &Path, project_root: &Path, exec_path: &Path) -> Result<(), String> {
    let extension = source_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    println!("Compiling {}...", source_path.display());

    let rustup = get_rustup_path();

    match extension {
        "cpp" => {
            let output = Command::new("g++")
                .args(&["-std=c++23", "-O2", "-Wall", "-fdiagnostics-color=always", source_path.to_str().unwrap(), "-o", exec_path.to_str().unwrap()])
                .output()
                .map_err(|e| format!("Failed to execute g++: {}", e))?;

            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        "rs" => {
            let content = fs::read_to_string(source_path).map_err(|e| format!("Read error: {}", e))?;
            let top_lines: Vec<&str> = content.lines().take(2).collect();
            
            let mode = if top_lines.iter().any(|l| l.contains("// compiler: cargo")) { "cargo" } 
                       else { "rustc" };

            let tc = if top_lines.iter().any(|l| l.contains("// toolchain: nightly")) { "nightly" } 
                     else { "1.89.0" };

            let use_nightly = tc == "nightly";
            let use_rustc_only = mode == "rustc";

            let mut cmd: Command;
            if use_rustc_only {
                cmd = Command::new(&rustup);
                if use_nightly {
                    cmd.args(&["run", "nightly", "rustc"]);
                } else {
                    cmd.args(&["run", "1.89.0", "rustc"]);
                }
                cmd.args(&["--color=always", "--diagnostic-width=1000", source_path.to_str().unwrap(), "-o", exec_path.to_str().unwrap()]);
                println!("  (Compiler: rustc, Toolchain: {})", tc);
            } else {
                let abs_source_path = fs::canonicalize(source_path).map_err(|e| e.to_string())?;
                let proxy_path = project_root.join("runner/src/bin/solution_proxy.rs");
                let proxy_content = format!("include!({:?});", abs_source_path.to_str().unwrap());
                fs::write(&proxy_path, proxy_content).map_err(|e| e.to_string())?;

                cmd = Command::new(&rustup);
                if use_nightly {
                    cmd.args(&["run", "nightly", "cargo"]);
                } else {
                    cmd.args(&["run", "1.89.0", "cargo"]);
                }
                cmd.args(&[
                    "build",
                    "--color=always",
                    "--manifest-path", project_root.join("runner/Cargo.toml").to_str().unwrap(),
                    "--bin", "solution_proxy"
                ]);
                println!("  (Compiler: cargo, Toolchain: {})", tc);
            }

            let output = cmd.output().map_err(|e| format!("Process execution error: {}", e))?;
            cleanup_long_type_files(exec_path);

            if output.status.success() {
                if !use_rustc_only {
                    let cargo_bin_path = project_root.join("runner/target/debug/solution_proxy");
                    fs::copy(&cargo_bin_path, exec_path).map_err(|e| format!("Binary copy error: {}", e))?;
                }
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        _ => Err(format!("Unsupported extension .{}", extension)),
    }
}

pub fn get_test_cases(data_dir: &Path) -> Vec<(PathBuf, PathBuf)> {
    let Ok(entries) = fs::read_dir(data_dir) else { return vec![]; };

    let mut cases: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "in"))
        .filter_map(|p| {
            let out = p.with_extension("out");
            out.exists().then_some((p, out))
        })
        .collect();

    cases.sort_by(|a, b| a.0.cmp(&b.0));
    cases
}

pub fn run_exec(exec_path: &Path, input_file: &Path) -> io::Result<ExecResult> {
    let input_file_handle = fs::File::open(input_file)?;
    
    let mut child = Command::new(exec_path)
        .stdin(Stdio::from(input_file_handle))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout_buf = Vec::new();
    let mut stderr_buf = Vec::new();
    let mut stdout_handle = child.stdout.take().unwrap();
    let mut stderr_handle = child.stderr.take().unwrap();

    let stdout_thread = std::thread::spawn(move || { stdout_handle.read_to_end(&mut stdout_buf).map(|_| stdout_buf) });
    let stderr_thread = std::thread::spawn(move || { stderr_handle.read_to_end(&mut stderr_buf).map(|_| stderr_buf) });

    let status = child.wait()?;
    let stdout = String::from_utf8_lossy(&stdout_thread.join().unwrap()?).to_string();
    let stderr = String::from_utf8_lossy(&stderr_thread.join().unwrap()?).to_string();

    Ok(ExecResult {
        success: status.success(),
        code: status.code().unwrap_or(-1),
        stdout,
        stderr,
    })
}
