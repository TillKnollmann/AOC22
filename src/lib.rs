/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 * Prefer `./helpers.rs` if you want to extract code from your solutions.
 */
use std::env;
use std::fs;
use std::process::Command;
use std::io;
use std::io::Write;

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! solve {
    ($day:expr, $part:expr, $solver:ident, $input:expr) => {{
        use aoc::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET, submit_result};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) -> Option<T> {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Some(result) => {
                    println!(
                        "{} {}(elapsed: {:.2?}){}",
                        result, ANSI_ITALIC, elapsed, ANSI_RESET
                    );
                    return Some(result);
                }
                None => {
                    println!("not solved.");
                    return None;
                }
            }
        }

        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part, ANSI_RESET);
        let result = print_result($solver, $input);
        match result {
            Some(value) => submit_result($day, $part, value),
            None => {},
        }
    }};
}

pub fn submit_result(day: u8, part: i32, result: u32) {
    println!("\nSubmitting day {}, part {}, result {}", day, part, result);

    let cmd_args: Vec<String> = vec!["submit".to_string(), part.to_string(), result.to_string(), "-d".to_string(), day.to_string()];
    match Command::new("aoc").args(cmd_args).output() {
        Ok(cmd_output) => {
            io::stdout()
                .write_all(&cmd_output.stdout)
                .expect("could not write cmd stdout to pipe.");
            io::stderr()
                .write_all(&cmd_output.stderr)
                .expect("could not write cmd stderr to pipe.");
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
        }
    }

    println!("\n");
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("elapsed:") {
            acc
        } else {
            let timing = l.split("(elapsed: ").last().unwrap();
            // use `contains` istd. of `ends_with`: string may contain ANSI escape sequences.
            // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
            if timing.contains("ns)") {
                acc // range below rounding precision.
            } else if timing.contains("µs)") {
                acc + parse_time(timing, "µs") / 1000_f64
            } else if timing.contains("ms)") {
                acc + parse_time(timing, "ms")
            } else if timing.contains("s)") {
                acc + parse_time(timing, "s") * 1000_f64
            } else {
                acc
            }
        }
    })
}

pub mod aoc_cli {
    use std::{
        fmt::Display,
        fs::create_dir_all,
        process::{Command, Output, Stdio},
    };

    pub enum AocCliError {
        CommandNotFound,
        CommandNotCallable,
        BadExitStatus(Output),
        IoError,
    }

    impl Display for AocCliError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AocCliError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
                AocCliError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
                AocCliError::BadExitStatus(_) => write!(f, "aoc-cli exited with a non-zero status."),
                AocCliError::IoError => write!(f, "could not write output files to file system."),
            }
        }
    }

    pub fn check() -> Result<(), AocCliError> {
        Command::new("aoc")
            .arg("-V")
            .output()
            .map_err(|_| AocCliError::CommandNotFound)?;
        Ok(())
    }

    pub fn read(day: u8, year: Option<u16>) -> Result<Output, AocCliError> {
        // TODO: output local puzzle if present.
        let args = build_args("read", &[], day, year);
        call_aoc_cli(&args)
    }

    pub fn download(day: u8, year: Option<u16>) -> Result<Output, AocCliError> {
        let input_path = get_input_path(day);

        let puzzle_path = get_puzzle_path(day);
        create_dir_all("src/puzzles").map_err(|_| AocCliError::IoError)?;

        let args = build_args(
            "download",
            &[
                "--overwrite".into(),
                "--input-file".into(),
                input_path.to_string(),
                "--puzzle-file".into(),
                puzzle_path.to_string(),
            ],
            day,
            year,
        );

        let output = call_aoc_cli(&args)?;

        if output.status.success() {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &input_path);
            println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzle_path);
            Ok(output)
        } else {
            Err(AocCliError::BadExitStatus(output))
        }

    }

    fn get_input_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/inputs/{}.txt", day_padded)
    }

    fn get_puzzle_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/puzzles/{}.md", day_padded)
    }

    fn build_args(command: &str, args: &[String], day: u8, year: Option<u16>) -> Vec<String> {
        let mut cmd_args = args.to_vec();

        if let Some(year) = year {
            cmd_args.push("--year".into());
            cmd_args.push(year.to_string());
        }

        cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

        cmd_args
    }

    fn call_aoc_cli(args: &[String]) -> Result<Output, AocCliError> {
        if cfg!(debug_assertions) {
            println!("Calling >aoc with: {}", args.join(" "));
        }

        Command::new("aoc")
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|_| AocCliError::CommandNotCallable)
    }
}

/// copied from: https://github.com/rust-lang/rust/blob/1.64.0/library/std/src/macros.rs#L328-L333
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exec_time() {
        assert_approx_eq!(
            parse_exec_time(&format!(
                "🎄 Part 1 🎄\n0 (elapsed: 74.13ns){}\n🎄 Part 2 🎄\n0 (elapsed: 50.00ns){}",
                ANSI_RESET, ANSI_RESET
            )),
            0_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 755µs)\n🎄 Part 2 🎄\n0 (elapsed: 700µs)"),
            1.455_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (elapsed: 70µs)\n🎄 Part 2 🎄\n0 (elapsed: 1.45ms)"),
            1.52_f64
        );

        assert_approx_eq!(
            parse_exec_time(
                "🎄 Part 1 🎄\n0 (elapsed: 10.3s)\n🎄 Part 2 🎄\n0 (elapsed: 100.50ms)"
            ),
            10400.50_f64
        );
    }
}
