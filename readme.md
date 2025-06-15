# minigrep

`minigrep` is a simple command-line utility implemented in Rust, inspired by the classic Unix `grep` command. It allows you to search for a specific string pattern within a file.

This project is often used as a practical example when learning the Rust programming language.

## Features

*   Search for a query string within a specified file.
*   Basic error handling for missing arguments or non-existent files.
*   Case-sensitive search by default.
*   Case-insensitive search can be enabled via an environment variable.
*   Colored error messages using the `colored` crate.

## Prerequisites

Before building and running `minigrep`, you need to have Rust and Cargo installed.

*   **Rust and Cargo:** Follow the instructions on the official Rust website to install `rustup`: [https://rustup.rs/](https://rustup.rs/)

## Building

1.  Clone the repository:
    ```bash
    git clone <repository_url> # Replace <repository_url> with the actual URL
    cd minigrep
    ```
2.  Build the project using Cargo:
    *   For a debug build (faster compilation):
        ```bash
        cargo build
        ```
    *   For a release build (optimized performance):
        ```bash
        cargo build --release
        ```

This will create an executable file in the `target/debug/` or `target/release/` directory.

## Usage

The basic syntax for running `minigrep` is:

```bash
./target/debug/minigrep <query> <filename>
```
or
```bash
./target/release/minigrep <query> <filename>
```

Replace `./target/debug/minigrep` with the path to the executable if you are running from a different directory or used a release build.

*   `<query>`: The string you want to search for.
*   `<filename>`: The path to the file you want to search within.

### Examples

1.  **Search for "hello" in `example.txt` (case-sensitive):**

    First, create a sample file:
    ```bash
    echo "This line says hello." > example.txt
    echo "Another line without the word." >> example.txt
    echo "HELLO there!" >> example.txt
    ```

    Now run minigrep:
    ```bash
    ./target/debug/minigrep hello example.txt
    ```
    Expected output:
    ```
    Searching for hello in example.txt
    This line says hello.
    ```
    Notice "HELLO" was not matched because the search is case-sensitive by default.

2.  **Search for "Rust" in `example.txt` (case-insensitive):**

    To perform a case-insensitive search, set the `CASE_INSENSITIVE` environment variable before running the command.

    ```bash
    CASE_INSENSITIVE=1 ./target/debug/minigrep HELLO example.txt
    ```
    Expected output:
    ```
    Searching for HELLO in example.txt
    This line says hello.
    HELLO there!
    ```
    Now both "hello" and "HELLO" are matched.

3.  **Handling Errors:**

    *   **Not enough arguments:**
        ```bash
        ./target/debug/minigrep hello
        ```
        Expected output (colored red):
        ```
