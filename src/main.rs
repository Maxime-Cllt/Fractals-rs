mod complex;
mod fractal;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITERATIONS: u32 = 100;

struct MandelbrotApp {
    center_x: f64,
    center_y: f64,
    zoom: f64,
}

impl MandelbrotApp {
    fn new() -> Self {
        Self {
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
        }
    }

    fn update(&mut self, input: &WinitInputHelper) {
        // Handle zoom
        if input.key_held(VirtualKeyCode::Equals) || input.key_held(VirtualKeyCode::NumpadAdd) {
            self.zoom *= 1.1;
        }
        if input.key_held(VirtualKeyCode::Minus) || input.key_held(VirtualKeyCode::NumpadSubtract) {
            self.zoom /= 1.1;
        }

        // Handle movement
        let move_speed = 0.1 / self.zoom;
        if input.key_held(VirtualKeyCode::Left) || input.key_held(VirtualKeyCode::A) {
            self.center_x -= move_speed;
        }
        if input.key_held(VirtualKeyCode::Right) || input.key_held(VirtualKeyCode::D) {
            self.center_x += move_speed;
        }
        if input.key_held(VirtualKeyCode::Up) || input.key_held(VirtualKeyCode::W) {
            self.center_y -= move_speed;
        }
        if input.key_held(VirtualKeyCode::Down) || input.key_held(VirtualKeyCode::S) {
            self.center_y += move_speed;
        }

        // Reset view
        if input.key_pressed(VirtualKeyCode::R) {
            self.center_x = -0.5;
            self.center_y = 0.0;
            self.zoom = 1.0;
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        // Calculate the bounds of the complex plane to display
        let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
        let width_range = 4.0 / self.zoom;
        let height_range = width_range / aspect_ratio;

        let min_x = self.center_x - width_range / 2.0;
        let max_x = self.center_x + width_range / 2.0;
        let min_y = self.center_y - height_range / 2.0;
        let max_y = self.center_y + height_range / 2.0;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as f64;
            let y = (i / WIDTH as usize) as f64;

            // Map pixel coordinates to complex plane
            let real = min_x + (max_x - min_x) * x / WIDTH as f64;
            let imag = min_y + (max_y - min_y) * y / HEIGHT as f64;

            let iterations = mandelbrot_iterations(real, imag, MAX_ITERATIONS);
            let color = get_color(iterations, MAX_ITERATIONS);

            pixel[0] = color.0; // Red
            pixel[1] = color.1; // Green
            pixel[2] = color.2; // Blue
            pixel[3] = 0xff;    // Alpha
        }
    }
}

fn mandelbrot_iterations(c_real: f64, c_imag: f64, max_iterations: u32) -> u32 {
    let mut z_real = 0.0;
    let mut z_imag = 0.0;
    let mut iterations = 0;

    while iterations < max_iterations {
        let z_real_squared = z_real * z_real;
        let z_imag_squared = z_imag * z_imag;

        // Check if point has escaped (|z|^2 > 4)
        if z_real_squared + z_imag_squared > 4.0 {
            break;
        }

        // z = z^2 + c
        let new_z_real = z_real_squared - z_imag_squared + c_real;
        let new_z_imag = 2.0 * z_real * z_imag + c_imag;

        z_real = new_z_real;
        z_imag = new_z_imag;
        iterations += 1;
    }

    iterations
}

fn get_color(iterations: u32, max_iterations: u32) -> (u8, u8, u8) {
    if iterations == max_iterations {
        // Point is in the set (black)
        (0, 0, 0)
    } else {
        // Color based on how quickly the point escaped
        let t = iterations as f64 / max_iterations as f64;

        // Create a smooth color gradient
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

        (r, g, b)
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Mandelbrot Fractal - Use WASD/Arrow keys to move, +/- to zoom, R to reset")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut app = MandelbrotApp::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            app.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            app.update(&input);
            window.request_redraw();
        }
    });
}