# Fractal Explorer 🌀

A high-performance fractal renderer built in Rust with an interactive GUI for exploring the beautiful world of
mathematical fractals including the Mandelbrot set, Julia sets, and more.

![Fractal Explorer Demo](demo.gif)

## Features

- **Real-time Fractal Rendering**: Smooth, interactive exploration of fractals
- **Multiple Fractal Types**:
    - Mandelbrot Set
    - Julia Sets (with customizable parameters)
    - Burning Ship
    - Newton's Fractal
    - Tricorn (Mandelbar)
- **Interactive GUI**:
    - Zoom and pan with mouse controls
    - Real-time parameter adjustment
    - Color palette customization
- **High Performance**: Multi-threaded rendering
- **Cross-platform**: Works on Windows, macOS, and Linux

## Screenshots

| Mandelbrot Set                            | Julia Set                       | Burning Ship                                  |
|-------------------------------------------|---------------------------------|-----------------------------------------------|
| ![Mandelbrot](screenshots/mandelbrot.png) | ![Julia](screenshots/julia.png) | ![Burning Ship](screenshots/burning_ship.png) |

## Installation

### Prerequisites

- Rust 1.85+ (install from [rustup.rs](https://rustup.rs/))
- Git

## Build & Execution

### Running the Application

```bash
cargo run --release
```

## Contributing



### Code Style

This project uses standard Rust formatting. Please run:

```bash
cargo fmt
cargo clippy
```



## Acknowledgments

- [Benoit Mandelbrot](https://en.wikipedia.org/wiki/Benoit_Mandelbrot) for his groundbreaking work on fractal geometry
- The Rust community for excellent mathematical and GUI libraries
- [The Fractal Geometry of Nature](https://en.wikipedia.org/wiki/The_Fractal_Geometry_of_Nature) for inspiration

<p align="center">
  Made with 🦀
</p>