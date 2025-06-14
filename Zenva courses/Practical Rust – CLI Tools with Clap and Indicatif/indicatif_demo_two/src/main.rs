use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
        .template("{spinner:.green} {msg}");

    let pb = ProgressBar::new_spinner();

    let tick_duration = Duration::from_millis(1000/120);
    
    pb.enable_steady_tick(tick_duration);

    pb.set_style(spinner_style.unwrap_or(ProgressStyle::default_spinner()));

    for _ in 0..50 {
        pb.set_message("Processing...");
        std::thread::sleep(std::time::Duration::from_millis(90));
    }

    pb.finish_with_message("Done!");
}
