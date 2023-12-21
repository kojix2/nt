use notify_rust::Notification;
use std::env;
use std::process::Command;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    // Grab the command-line arguments and concatenate them into a
    // space-separated string
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }
    let command = env::args().skip(1).collect::<Vec<String>>().join(" ");

    // Measure the time of the command execution
    let start = Instant::now();
    let output = Command::new("sh").arg("-c").arg(&command).output()?;
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64(); // Convert to float

    // Notify the result of command execution
    if output.status.success() {
        Notification::new()
            .summary(&format!("Success: {:.2} seconds", elapsed_secs))
            .body(&command)
            .show()
            .unwrap();
    } else {
        Notification::new()
            .summary(&format!("Failed: {:.2} seconds", elapsed_secs))
            .body(&command)
            .show()
            .unwrap();
    }
    Ok(())
}
