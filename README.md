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

- **Problem Generator**: Interactive tool (`add_problem`) to create new problem folders and templates.
- **Automated Suite Testing**: The `runner` tool automatically compiles a solution and runs it against all associated test cases.
- **Single Case Debugging**: The `cptest` tool compiles and runs a solution against a single, specific test case for easy debugging.
- **Smart Path Detection**: Run tools from any subdirectory; they automatically find the project root.

## Get Started

### 1. Prerequisite

Ensure you have Rust installed to build the tools.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build and Install Tools

Go to the `runner` directory, compile the tools, and copy them to the local `bin` directory.

**1. Install Rust Binaries**

The `cargo install` command will compile the tools and install them to your central cargo directory (`~/.cargo/bin`).

```bash
cd runner
cargo install --path .
cd ..
```

**2. Copy to Local Bin**

For this project's setup, copy the installed binaries into the project's local `bin` folder.

```bash
# Assuming you are in the project root
cp ~/.cargo/bin/add_problem bin/
cp ~/.cargo/bin/runner bin/
cp ~/.cargo/bin/cptest bin/
```

### 3. Configure PATH

Add the project's `bin` directory to your `PATH` in your shell configuration (e.g., `~/.bashrc`):

```bash
export PATH="$PATH:$HOME/CodingPractice/bin"
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

## Testing a Single Case

For quickly compiling and checking a single test case without running the entire suite, use the `cptest` command.

This is useful for debugging a specific scenario.

```bash
# Usage: cptest <testcase_id> <path_to_solution_file>
cptest 1 problems/BOJ/1000/solution.cpp
```

This command will:
1. Compile `solution.cpp` with the project's standard settings.
2. Run the compiled binary with `problems/BOJ/1000/data/1.in` as input.
3. Compare the result against `problems/BOJ/1000/data/1.out`.
4. Report whether the test passed or failed.

For more details, run `cptest --help`.

