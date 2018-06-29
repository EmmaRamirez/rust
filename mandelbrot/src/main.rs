extern crate num;
use num::Complex;
use std::str::FromStr;

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

extern crate crossbeam;

use std::io::Write;

/**
 * Parse the string `s` as a coordinate par, like "400x600" or "1.0, 0.5"
 * Sepcifically, `s` should have the form <left><sep><right>, where <left> and <right are parsed
 * from T::from_str
 * 
 * If `s` has proper form, reutrn `Some<(x, y)>`. If it doesn't parse correctly, return None
 */
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex{ re, im }),
        None => None
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option< u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/*
    Given the row and column of a pixel in the output image,
    return the corresponding point on the complex plane

    `bounds` is a apair giving the width and height of the image in pixels.
    `pixel` is a column, row pair indicating a particular pixel in that image

    The plane designating the area our image covers
*/
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64> 
{
    let (width, height) = (lower_right.re - upper_left.re,
                            upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}


/**
    Render a rectangle of the Mandelbrot set into a buffer of pixels.

    The `bounds` argument gives the width height of the buffer `pixels`
    which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
    arguments specify points on the complex plain corresponding to the upper left 
    and lower-right corners of the pixel buffer
**/
fn render(pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.1 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

// Write the buffer `pixels` whose dimensions are given by bounds to the file named filename
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    
    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("Error parsing upper left");
    let lower_right = parse_complex(&args[4])
        .expect("Error parsing lower right");
    
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // render(&mut pixels, bounds, upper_left, lower_right);

    // TODO: use num_cpus crate
    {
        let threads = 1;
        let rows_per_band = bounds.1 / threads + 1;

        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}