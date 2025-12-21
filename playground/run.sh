#!/bin/bash

# Get the source file from argument
SOURCE_FILE="$1"
shift


# Check if file is provided
if [ -z "$SOURCE_FILE" ]; then
  echo "Usage: ./run.sh <source_file>"
  exit 1
fi

# Get filename and extension
FILENAME=$(basename "$SOURCE_FILE")
EXTENSION="${FILENAME##*.}"

# Define output directory and executable name
# Assuming run.sh is in playground/, so ../bin is the sibling bin directory
BIN_DIR="../bin" 
EXEC_TARGET="$BIN_DIR/temp_exec"

# Ensure bin directory exists
mkdir -p "$BIN_DIR"

# Compilation and Execution Logic
case "$EXTENSION" in
    rs)
        echo "[*] Compiling Rust file: $FILENAME"
        rustc "$SOURCE_FILE" -o "$EXEC_TARGET"
        COMPILE_STATUS=$?
        ;;
    cpp)
        echo "[*] Compiling C++ file: $FILENAME"
        clang++-19 -std=c++23 --gcc-toolchain=/usr "$SOURCE_FILE" -o "$EXEC_TARGET"
        COMPILE_STATUS=$?
        ;;
    *)
        echo "[!] Unsupported file extension: $EXTENSION"
        exit 1
        ;;
esac

# Check compilation status
if [ $COMPILE_STATUS -eq 0 ]; then
    echo "[*] Compilation successful. Running output..."
    echo "=========================================="
    "$EXEC_TARGET" "$@"
    echo -e "\n=========================================="
    
    # Cleanup: The user requested files to disappear after compilation/run if possible.
    # Since we output to ../bin/temp_exec, we don't pollute the current dir.
    # We leave temp_exec there as requested ("created as temp_exec").
    # If there are any .o files (unlikely with these commands), we could remove them.
    rm -f *.o
else
    echo "[!] Compilation failed."
    exit 1
fi
