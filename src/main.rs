use getopts::Options;
use notify_rust::Notification;
use std::env;
use std::error::Error;
use std::process::{exit, Command, Stdio};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "show version info");

    if args.len() <= 1 {
        print_usage(&args[0], &opts, true);
        exit(1);
    }

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_usage(&args[0], &opts, false);
        return Ok(());
    }

    if matches.opt_present("v") {
        println!("version 0.1.0");
        return Ok(());
    }

    let command = matches.free.join(" ");

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
            .show()?; // propagate error with `?`
    } else {
        Notification::new()
            .summary(&format!("Failed: {:.2} seconds", elapsed_secs))
            .body(&command)
            .show()?; // propagate error with `?`
    }
    Ok(())
}

fn print_usage(program: &str, opts: &Options, eprint: bool) {
    let brief = format!("Usage: {} [options] <command>", program);
    let usage_string = opts.usage(&brief);
    if eprint {
        eprintln!("{}", usage_string);
    } else {
        println!("{}", usage_string);
    }
}
