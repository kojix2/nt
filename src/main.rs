use notify_rust::Notification;
use std::error::Error;
use std::process::{exit, Command, Stdio};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: {} [options] <command>", args[0]);
        exit(1);
    }

    let command = match args[1].as_str() {
        "-h" | "--help" => {
            println!("Usage: {} [options] <command>", args[0]);
            return Ok(());
        }
        "-v" | "--version" => {
            println!("version 0.1.0");
            return Ok(());
        }
        _ => args[1..].join(" "),
    };

    // Time the command execution
    let start = Instant::now();
    #[cfg(target_os = "windows")]
    let mut child = Command::new("cmd")
        .arg("/C")
        .arg(&command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    #[cfg(not(target_os = "windows"))]
    let default_shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
    #[cfg(not(target_os = "windows"))]
    let mut child = Command::new(default_shell)
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
            .show()?; // propagate error with `?`
    } else {
        Notification::new()
            .summary(&format!("Failed: {:.2} seconds", elapsed_secs))
            .body(&command)
            .show()?; // propagate error with `?`
    }
    Ok(())
}
