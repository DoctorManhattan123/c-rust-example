# Hello World C-Rust Integration

A simple example demonstrating how to call a Rust function from a C program, and vice versa.

## Dependencies

Install the required dependencies:

### Arch

```
sudo pacman -S base-devel rustup
rustup install stable
```

## Usage and Build Instructions

1. Build the project:
```
make
```

2. Run the project:

```
./main
```

The output should display "Hello from Rust!" followed by "Extern Hello from C!".

