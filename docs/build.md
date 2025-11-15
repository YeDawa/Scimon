# How to Build

### 1. Clone the Repository

If you don't already have the project on your local machine, you need to clone the repository from Git. Open a terminal and run:

```sh
git clone https://github.com/YeDawa/Scimon.git
```

### 2. Setup Your Environment

Ensure that Rust and Cargo are installed and added to your system's PATH. You can check this by running:

```sh
rustc --version
cargo --version
```

If Rust and Cargo are properly installed, these commands should return the version numbers.

### 3. Navigate to Your Project Directory

Open a terminal and navigate to the root directory of your Rust project. For example:

```sh
cd Scimon
```

If you just cloned the repository, navigate to the newly created directory:

```sh
cd Scimon
```

### 4. Build the Project

To build the project, simply run:

```sh
cargo build
```

This command compiles your ``Scimon`` and places the output binaries in the `target/debug` directory. You should see output similar to:

```
Compiling scimon v0.1.0 (/path/to/scimon)
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
```

### 5. Run the Project

To run the project after building it, use:

```sh
cargo run
```

This command builds and runs the project in one step.

### 6. Building for Release

If you want to build an optimized release version of your project, run:

```sh
cargo build --release
```

This command compiles your project with optimizations and places the output binaries in the `target/release` directory. The output will be similar to:

```
Compiling scimon v0.1.0 (/path/to/scimon)
    Finished release [optimized] target(s) in 2m 13s
```
