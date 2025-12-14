## PPM File Generator
Draw into a PPM file.

```bash
cargo run
```

```bash
cargo build --release
```

```
Draws into a .ppm file

Usage: ppm-rs [OPTIONS]

Options:
      --width <WIDTH>              Output image width [default: 9600]
      --height <HEIGHT>            Output image height [default: 5400]
  -o, --output-path <OUTPUT_PATH>  Output file path [default: output.ppm]
  -h, --help                       Print help
  -V, --version                    Print version
```

See the PPM specification here: https://netpbm.sourceforge.net/doc/ppm.html

## Usage
To use the mandelbrot set raster generator, build the raster and use the `PPMImageBuffer` to write to a .ppm file.

```rust
use ppm_rs::ppm::PPMImageBuffer;
use ppm_rs::raster::{mandelbrot::Mandelbrot, RasterGenerator};

fn main() {
  let raster_builder = Mandelbrot::new(100).unwrap();
  let raster = raster_builder.generate(16*600, 9*600);

  let ppm_buf = PPMImageBuffer::new(16*600, 9*600, raster)
          .expect("Error creating ppm buffer");
  ppm_buf.write_to_ppm("output.ppm").expect("Error creating ppm file");
}
```

![Mandelbrot set PPM output](docs/output.webp)

