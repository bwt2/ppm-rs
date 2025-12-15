//! ppm-rs
//!
//! A small ppm library for drawing Mandelbrot and Multibrot fractals.
//!
//! # Example
//! ```rust
//! use multibrot_ppm_rs::raster::{multibrot::Multibrot, ViewWindow, RasterGenerator};
//!
//! let mb = Multibrot::new_with_view(2.0, 100, ViewWindow::default()).unwrap();
//! let raster = mb.generate(800, 600);
//! ```

pub mod ppm;
pub mod raster;
