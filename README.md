# Coding Practice Environment

A structured environment for practicing algorithm problems (BOJ, Codeforces, LeetCode, etc.) with a custom automated test runner and problem parser.

## Directory Structure

```text
CodingPractice/
├── problems/
│   ├── acmicpc/       # Previously BOJ
│   │   └── 1000/
│   │       ├── data/          # Test cases (in/out)
│   │       ├── solution.cpp   # Your solution
│   │       └── README.md      # Problem description & limits
│   ├── codeforces/
│   └── leetcode/      # (Optional)
├── runner/
│   ├── resources/
│   │   ├── cookies/   # Site-specific session cookies (.json)
│   │   └── ...
│   ├── src/
│   │   └── bin/
│   │       ├── runner.rs      # Test suite runner
│   │       ├── add_problem.rs # Problem creation tool
│   │       ├── cptest.rs      # Single case debugger
│   │       └── parse/         # Parsing logic (main.rs, crawl.rs, etc.)
│   ├── templates/     # Code & README templates
│   └── Cargo.toml
├── bin/               # Compiled binaries
│   ├── runner
│   ├── add_problem
│   └── cptest
└── README.md
```

## Features

- **Integrated Problem Generator**: 
  - **Manual Input**: Create templates and input test cases manually.
  - **URL Parsing**: Automatically fetch problem titles, descriptions, constraints, and test cases from supported sites (currently **Codeforces**).
- **Automated Suite Testing**: The `runner` tool automatically compiles a solution and runs it against all associated test cases.
- **Bypass Cloudflare**: Supports both Cookie-based and CDP-based (Chromium DevTools Protocol) fetching to handle protected sites.
- **Smart Path Detection**: Run tools from any subdirectory; they automatically find the project root.

## Environment & Requirements

### Project Criteria

1.  **Multiple Solution Support**: Support various language solutions (C++, Rust) for a single problem side-by-side.
2.  **Test Case Separation**: Keep test data (`.in`/`.out`) separate from source code in a `data/` subdirectory.
3.  **Automated Runner**: One-command automation to compile and test solutions.
4.  **Structured Hierarchy**: `problems/{site}/{problem_id}` structure (all lowercase) for organized practice.

## Get Started

### 1. Prerequisite

Ensure you have Rust and Chromium (for CDP fetching) installed.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build and Install Tools

```bash
cd runner
cargo build
cp target/debug/add_problem ../bin/
cp target/debug/runner ../bin/
cp target/debug/cptest ../bin/
cd ..
```

### 3. Configure Session Cookies (Optional)

To parse problems from sites requiring login (or to bypass certain checks), place exported cookies in `runner/resources/cookies/{site}.json`.
- Supported site names: `codeforces`, `acmicpc`, `leetcode`.

## Adding a New Problem

Use the integrated `add_problem` tool:

```bash
add_problem
```

1. **Select Input Method**: 
   - `Manual Input`: Traditional manual setup.
   - `Parse from URL`: Provide a URL (e.g., Codeforces) to automate everything.
2. **Automatic Setup**: If using URL, the tool will:
   - Create the directory: `problems/{site}/{id}/`
   - Create `solution.rs` and `solution.cpp` from templates.
   - Generate `README.md` with the problem statement.
   - Save sample test cases to the `data/` folder.

## Running Tests with `runner`

```bash
runner problems/acmicpc/1000/solution.cpp
```

### Features

-   **Automatic Compilation**: C++ (`-std=c++23`), Rust (`rustc`).
-   **Output Verification**: Accurate comparison between solution output and `.out` files.
-   **Resource Limiting**: 
    - Parsed from `README.md`'s `## 제한` section.
    - Default: **1 second** / **256 MB**.

## Testing a Single Case

```bash
# Usage: cptest <testcase_id> <path_to_solution_file>
cptest 1 problems/acmicpc/1000/solution.rs
```
