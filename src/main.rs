use std::thread;
use std::time::Duration;

use gumdrop::Options;

#[derive(Debug, Options)]
struct ConfigOptions {
    #[options(help = "delay in milliseconds between mouse movements")]
    delay: Option<u64>,

    #[options(help = "seconds to sleep before looping the program")]
    sleep: Option<u32>,

    #[options(help = "movement distance in pixels")]
    pixels: Option<f64>,
}

fn mouse_move(delay: u64, pixels: f64) {
    // arbitrary pattern that returns to origin
    let origin = autopilot::mouse::location();
    let mut dest = autopilot::geometry::Point::new(origin.x - pixels, origin.y);
    let _ = autopilot::mouse::move_to(dest);
    dest.x += pixels;
    dest.y += pixels;
    thread::sleep(Duration::from_millis(delay));
    let _ = autopilot::mouse::move_to(dest);
    dest.x += pixels;
    dest.y -= pixels;
    thread::sleep(Duration::from_millis(delay));
    let _ = autopilot::mouse::move_to(dest);
    dest.x -= pixels;
    dest.y -= pixels;
    thread::sleep(Duration::from_millis(delay));
    let _ = autopilot::mouse::move_to(dest);
    thread::sleep(Duration::from_millis(delay));
    let _ = autopilot::mouse::move_to(origin);
}

fn main() {
    let opts = ConfigOptions::parse_args_default_or_exit();

    let delay = match opts.delay {
        Some(x) => x,
        None => 10,
    };
    let pixels = match opts.pixels {
        Some(x) => x,
        None => 1.0,
    };

    // option not set, run single time
    if opts.sleep.is_none() {
        mouse_move(delay, pixels);
    } else {
        let sleep_ms = match opts.sleep {
            Some(x) => x * 1000,
            None => 60 * 1000,
        };
        // loop forever
        loop {
            mouse_move(delay, pixels);
            thread::sleep(Duration::from_millis(sleep_ms as u64))
        }
    }
}
