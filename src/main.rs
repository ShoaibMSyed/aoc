#![feature(gen_blocks)]

use std::{path::PathBuf, time::Instant};

use anyhow::{Context, Result};
use log::LevelFilter;
use simple_logger::SimpleLogger;

pub mod util;

macro_rules! years {
    ($($year:literal [ $($day:literal),* $(,)? ]),* $(,)?) => {
        paste::paste! {
            $(mod [< y $year >] {
                $(pub mod [< day $day >];)*
            })*
            

            fn call(year: usize, day: usize, part: usize, input: String) -> Result<()> {
                match year {
                    $(
                        $year => match day {
                            $(
                                $day => match part {
                                    1 => self::[< y $year >]::[< day $day >]::part1(input),
                                    2 => self::[< y $year >]::[< day $day >]::part2(input),
                                    _ => Err(anyhow::anyhow!("'{part}' is not a valid part")),
                                }
                            )*
                            _ => Err(anyhow::anyhow!("'{day}' is not a valid day")),
                        }
                    )*
                    _ => Err(anyhow::anyhow!("'{year}' is not a valid year")),
                }
            }
        }
    };
}

const CUR_YEAR: usize = 2024;

years!(2024 [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => eprintln!("{e:?}"),
    }
}

fn run() -> Result<()> {
    SimpleLogger::new()
        .with_module_level("rustls", LevelFilter::Off)
        .with_level(LevelFilter::Info)
        .init()?;

    let mut args = std::env::args().skip(1).peekable();

    let first_arg = args
        .peek()
        .context("first argument not provided")?;

    let year: usize = match first_arg.starts_with('y') {
        true => {
            let first_arg = args.next().unwrap();

            first_arg[1..].parse()
                .context("first argument 'year' is not a number")?
        }
        _ => CUR_YEAR,
    };

    let day: usize = args
        .next()
        .context("first argument 'day' not provided")?
        .parse()
        .context("first argument 'day' is not a number")?;

    let part: usize = args
        .next()
        .context("second argument 'part' not provided")?
        .parse()
        .context("second argument 'part' is not a number")?;

    let input = match args.next() {
        None => load_input_file(day, "")?,
        Some(s) => {
            if &s == "-" {
                args.next().unwrap_or_default()
            } else {
                load_input_file(day, &s)?
            }
        }
    };

    let start = Instant::now();

    call(year, day, part, input)?;

    let end = Instant::now();

    let duration = end - start;

    if duration.as_secs() < 5 {
        println!("ran in {:.3} millis", duration.as_micros() as f64 / 1000.0);
    } else {
        println!("ran in {:.3} seconds", duration.as_secs_f64());
    }    

    Ok(())
}

fn load_input_file(day: usize, suffix: &str) -> Result<String> {
    let path = format!("input/{day}{suffix}");
    let path = PathBuf::from(path);

    match std::fs::read_to_string(&path) {
        Ok(input) => Ok(input),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && suffix.is_empty() => {
            eprintln!("file '{}' not found, downloading...", path.display());

            let input = download_input_file(day).context("error downloading input file")?;

            match std::fs::write(&path, &input) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("could not write input to file '{}':\n{e:?}", path.display());
                }
            }

            Ok(input)
        }
        Err(e) => Err(e).with_context(|| format!("file '{}' not found", path.display())),
    }
}

fn download_input_file(day: usize) -> Result<String> {
    let session =
        std::env::var("AOC_SESSION").context("AOC_SESSION environment variable not found")?;

    let url = format!("https://adventofcode.com/2024/day/{day}/input");

    let resp = ureq::get(&url)
        .set("Cookie", &format!("session={session}"))
        .call()?;

    Ok(resp.into_string()?)
}
