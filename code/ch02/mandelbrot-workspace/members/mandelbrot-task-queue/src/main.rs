use std::env;
use std::thread;
use std::sync::Mutex;

use mandelbrot_lib::parse_complex;
use mandelbrot_lib::parse_pair;
use mandelbrot_lib::pixel_to_point;
use mandelbrot_lib::render;
use mandelbrot_lib::write_image;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let band_rows = bounds.1 / threads + 1;
    
    {
        let bands = Mutex::new(pixels.chunks_mut(band_rows * bounds.0).enumerate());
        thread::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|| {
                    loop {
                        match {
                            let mut guard = bands.lock().unwrap();
                            guard.next()
                        }
                        {
                            None => { return; }
                            Some((i, band)) => {
                                let top = band_rows * i;
                                let height = band.len() / bounds.0;
                                let band_bounds = (bounds.0, height);
                                let band_upper_left = pixel_to_point(bounds, (0, top),
                                                                     upper_left, lower_right);
                                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
                                                                      upper_left, lower_right);
                                render(band, band_bounds, band_upper_left, band_lower_right);
                            }
                        }
                    }
                });
            }
        });
        
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
