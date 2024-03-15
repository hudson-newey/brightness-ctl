use std::env;
use std::f64::consts::E;
use std::process::Command;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        println!("Error: No brightness value provided");
        println!("Usage: brightness <brightness>");
        return;
    }

    let brightness: f64 = args[1].parse::<f64>().unwrap();

    if brightness < 0.0 || brightness > 1.0 {
        println!("Error: Brightness value must be between 0.0 and 1.0");
        return;
    }

    let gamma: f64 = calculate_gamma(brightness);

    change_output(brightness, gamma);
}

fn calculate_gamma(brightness: f64) -> f64 {
    // I don't know if "turning point" is the correct term
    // what I'm trying to say is that in exponential function
    // when x = 0.85, y = 0.85
    const TURNING_POINT: f64 = 0.8;
    let m = TURNING_POINT / E.powf(TURNING_POINT);
    let c = 1.0 - (m * E.powf(1.0));

    let gamma = m * E.powf(brightness) + c;

    let clamped_gamma = gamma.clamp(0.6, 1.0);

    #[cfg(debug_assertions)]
    println!("[DEBUG] Gamma: {}", clamped_gamma);

    return clamped_gamma;
}

fn change_output(brightness: f64, gamma: f64) {
    let command: String = format!(
        "xrandr --output eDP-1 --brightness {} --gamma {}",
        brightness, gamma
    );

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
}
