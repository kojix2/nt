use notify_rust::Notification;
use std::env;
use std::io::{self, Write};
use std::process::{exit, Command, Stdio};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} <command>", args[0])?;
        exit(1);
    }
    let command = env::args().skip(1).collect::<Vec<String>>().join(" ");

    // Time the command execution
    let start = Instant::now();
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let output = child.wait()?;
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64(); // Convert to float

    // Display notification for the result of command execution
    if output.success() {
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
