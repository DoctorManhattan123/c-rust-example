# Hello World C-Rust Integration

A simple example demonstrating how to call a Rust function from a C program, and vice versa.

## Dependencies

Install the required dependencies on Arch Linux:

```
sudo pacman -S base-devel rustup
rustup install stable
```

## Usage and Build Instructions

1. Clone the repository:

```
git clone https://github.com/yourusername/hello-world-c-rust.git
cd hello-world-c-rust
```

2. Build the project:
```
make
```

3. Run the project:

```
make run
```

The output should display "Hello from Rust!" followed by "Extern Hello from C!".
