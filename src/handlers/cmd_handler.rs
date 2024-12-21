use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn run_cmd(cmd: &str, display: bool, print: Option<&bool>) {
    let &print = print.unwrap_or(&false);
    let expanded_cmd = shellexpand::full(cmd).unwrap(); // Expand environment variables

    let mut parts = expanded_cmd.split_whitespace();
    let command = parts.next().unwrap_or("");
    let args = parts.collect::<Vec<_>>();

    let output = Command::new(command)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    if print {
        println!("command execute: {}", cmd);
    }

    if output.status.success() {
        if display {
            let result = String::from_utf8_lossy(&output.stdout);
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout
                .set_color(&ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            println!("{}", result);
            stdout.reset().unwrap();
        }
    } else {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr
            .set_color(&ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        let error = String::from_utf8_lossy(&output.stderr);
        writeln!(&mut stderr, "Command failed with error:\n{}", error).unwrap();
        stderr.reset().unwrap();
    }
}
