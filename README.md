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
├── runner/
│   ├── Cargo.toml
│   └── src/
│       └── bin/
│           ├── runner.rs      # Main test runner source
│           └── add_problem.rs # Problem creation tool source
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

- **Problem Generator**: Interactive tool (`add_problem`) to create new problem folders, templates, a basic test cases.
- **Automated Testing**: Automatically compiles your C++ (`.cpp`) or Rust (`.rs`) code and runs it against all test cases in the `data/` folder.
- **Smart Path Detection**: Run the `runner` command from any subdirectory; it automatically finds the project root.

## Get Started

### 1. Prerequisite

Ensure you have Rust installed to build the tools.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build Tools

Go to the `runner` directory and build the binaries:

```bash
cd runner
cargo build --release
```

This will compile `runner` and `add_problem` into `target/release/`.

### 3. Install Binaries

Copy the binaries to the `bin` directory (or ensure your build script does so):

```bash
# Assuming you are in the project root
cp runner/target/release/runner bin/
cp runner/target/release/add_problem bin/
```

### 4. Configure PATH

Add the project's `bin` directory to your `PATH` in your shell configuration (e.g., `~/.bashrc`):

```bash
export PATH="$PATH:/path/to/CodingPractice/bin"
```

_Reload your shell or run `source ~/.bashrc`._

## Adding a New Problem

Use the interactive `add_problem` tool:

```bash
add_problem
```

1. Select a category (e.g., BOJ).
2. Enter the problem number.
3. Enter the number of test cases.
4. Input the test case inputs and outputs directly in the terminal (end with an empty line).
5. Start coding in `problems/BOJ/<number>/solution.cpp` (or `.rs`).

## Running Tests

To run a solution against its test cases:

```bash
# From anywhere in the project
runner problems/BOJ/1000/solution.cpp

# Or if you are inside the problem folder
cd problems/BOJ/1000
runner solution.cpp
```

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
