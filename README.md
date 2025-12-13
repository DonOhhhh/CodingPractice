# Coding Practice Environment

A structured environment for practicing algorithm problems (BOJ, LeetCode, etc.) with a custom automated test runner.

## Directory Structure
```text
CodingPractice/
├── problems/
│   ├── BOJ/
│   │   └── 1000/
│   │       ├── data/          # Test cases (in/out)
│   │       ├── solution.cpp   # Your solution
│   │       └── ...
│   ├── LeetCode/
│   └── ...
├── playground/                # Free coding area (auto-cleanup)
│   ├── run.sh                 # Playground runner script
│   └── ...
├── runner/                    # Rust-based test runner source
├── bin/                       # Compiled binaries (auto-generated)
└── README.md
```

## Environment & Requirements

### Project Criteria
1.  **Multiple Solution Support**: Support various language solutions (C++, Rust) for a single problem side-by-side.
2.  **Test Case Separation**: Keep test data (`.in`/`.out`) separate from source code in a `data/` subdirectory.
3.  **Automated Runner**: One-command automation to compile and test solutions.
4.  **Clean Builds**: Manage build artifacts in a centralized `bin/` directory to keep source folders clean.
5.  **Structured Hierarchy**: `problems/{Site}/{ProblemID}` structure for organized practice.

### Environment Details
- **OS**: Linux
- **Languages Supported**:
    - **C++**: Compiled with `g++ -std=c++23 -O2 -Wall`
    - **Rust**: Compiled with `rustc`
- **Shell**: Bash (tested with aliases)

## Features
- **Organized Structure**: Keep all your algorithmic solutions sorted by site and problem ID.
- **Automated Testing**: Automatically compiles your C++ (`.cpp`) or Rust (`.rs`) code and runs it against all test cases in the `data/` folder.
- **Smart Path Detection**: Run the `runner` command from any subdirectory; it automatically finds the project root.

## Setup & Usage

### 1. Prerequisite
Ensure you have Rust installed to build the runner.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build the Runner
Go to the `runner` directory and build the release binary:
```bash
cd runner
cargo build --release
```

### 3. Setup Alias (Optional but Recommended)
Add an alias to your shell configuration (e.g., `.bashrc`, `.zshrc`) for easy access:
```bash
alias runner="/path/to/CodingPractice/runner/target/release/runner"
```
*Note: I have already added a `runner` alias to your `.bashrc` automatically.*

### 4. Running Tests
To run a solution against its test cases:

```bash
# From anywhere in the project
runner problems/BOJ/1000/solution_bfs.cpp

# Or if you are inside the problem folder
cd problems/BOJ/1000
runner solution_bfs.cpp
```

## Adding a New Problem
1. Create a folder: `problems/BOJ/1001/`
2. Create a `data` folder inside it.
3. Add test cases:
   - `data/1.in`, `data/1.out`
   - `data/2.in`, `data/2.out`
4. Write your solution (e.g., `solution.cpp`).
5. Run `runner solution.cpp`.

## Playground (Free Coding)

A dedicated space for testing code snippets without the structure of a problem.

- **Directory**: `playground/`
- **Runner**: `./run.sh <file>`
- **Features**:
    - Compiles to `../bin/temp_exec` to keep the playground clean.
    - Runs the executable immediately.
    - Supports C++, Rust, C, and Python.
    - **C++ Compiler**: Uses `clang++-19` with C++23 support (`-std=c++23`).

### Usage
```bash
cd playground
./run.sh hello.cpp
```
