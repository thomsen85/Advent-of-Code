use std::fs;
use std::io::Write;
use std::process::Command;

use arboard::Clipboard;
use chrono::Datelike;
use chrono::TimeZone;
use clap::Parser;
use clap::Subcommand;
use reqwest::blocking::Client;
use reqwest::header::HeaderValue;

#[derive(Subcommand, Debug, Clone)]
enum Cmd {
    New {
        // This should be a templated file with the correct keywords:
        // TODO: Fill here
        #[arg(short, long, default_value = "aoc/template")]
        template_path: String,

        #[arg(short, long)]
        no_fetch_input: bool,
    },
    Submit {
        #[arg(short, long)]
        no_check_tests: bool,
    },
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    year: Option<i32>,

    #[arg(short, long)]
    day: Option<u32>,

    #[arg(short, long, default_value = "1")]
    part: u32,

    #[arg(
        short,
        long,
        default_value = "/home/thomas/Development/Advent-of-Code/aoc/COOKIE"
    )]
    cookie_file: String,

    #[command(subcommand)]
    option: Cmd,
}

#[derive(Debug, Clone)]
struct Tests {
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    if args.year.is_none() != args.day.is_none() {
        eprintln!("Year and day must be specified together.");
        std::process::exit(1);
    }

    let (year, day) = if args.year.is_some() {
        (args.year.unwrap(), args.day.unwrap())
    } else {
        let today = chrono::Utc::now();
        (today.year(), today.day())
    };

    println!("===== Advent of Code =====");
    println!("Year: {}", year);
    println!("Day: {}", day);
    println!("Part {}", args.part);
    println!("==========================");

    let client = create_client(&args.cookie_file);

    match args.option {
        Cmd::New {
            template_path,
            no_fetch_input,
        } => new(client, year, day, template_path, no_fetch_input),
        Cmd::Submit { no_check_tests } => submit(client, year, day, args.part, no_check_tests),
    }
}

fn create_client(cookie_file: &str) -> Client {
    let cookie = std::fs::read_to_string(cookie_file)
        .unwrap_or_else(|_| panic!("Cannot find cookie file {}", cookie_file));

    let cookie = format!("session={}", cookie.trim());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

fn new(client: Client, year: i32, day: u32, template_path: String, no_fetch_input: bool) {
    // Wait for the task to drop with a nice timer :)

    if !no_fetch_input {
        let opening_time = chrono::Utc
            .with_ymd_and_hms(year, 12, day, 5, 0, 1)
            .unwrap();

        if chrono::Utc::now() < opening_time {
            let time_to_wait = opening_time - chrono::Utc::now();

            for i in (0..time_to_wait.num_seconds()).rev() {
                print!("\rWaiting for the task to drop in {:?} secs", i);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        // Send a request to fetch the input
        let res = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let input_path = format!("{}/inputs/day{}.txt", year, day);
        fs::write(input_path, res).unwrap();
    }
    // Read stdin to get the tests:
    let mut test_cases = Vec::new();
    let mut clipboard = Clipboard::new().unwrap();
    let mut content = clipboard.get_text().unwrap();
    loop {
        println!("Please enter test input for test {}:", test_cases.len() + 1);
        loop {
            if clipboard.get_text().unwrap() != content {
                content = clipboard.get_text().unwrap();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }

        let input = content.clone();

        println!(
            "Please enter test output for test {}:",
            test_cases.len() + 1
        );
        loop {
            if clipboard.get_text().unwrap() != content {
                content = clipboard.get_text().unwrap();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }

        let output = content.clone();

        test_cases.push(Tests {
            input: input.to_string(),
            output: output.to_string(),
        });

        println!("Do you want to add another test case? (y/n)");
        let mut input_buf = String::new();
        std::io::stdin()
            .read_line(&mut input_buf)
            .expect("Cannot read test input from stdin");

        if input_buf.trim().is_empty() || input_buf.trim() != "y" {
            break;
        }
    }

    let file_path = format!("{}/src/bin/day{}-1.rs", year, day);

    let template = fs::read_to_string(template_path).expect("Cannot remove template file");
    let code = substitute_template(&template, year, day, &test_cases);
    fs::write(&file_path, code.clone()).expect("Cannot write test file");

    println!("Created {}", file_path);
}

fn substitute_template(template: &str, year: i32, day: u32, tests: &[Tests]) -> String {
    let mut template = template.to_string();

    template = template.replace("***year***", &year.to_string());
    template = template.replace("***day***", &day.to_string());

    let test_start_pattern = "***|begin test|***";
    let test_end_pattern = "***|end test|***";
    dbg!(&template);
    let start_test_index = template
        .find(test_start_pattern)
        .expect("Not found start test pattern");
    let end_test_index = template
        .find(test_end_pattern)
        .expect("Not found end test pattern");

    let test_str =
        template[(start_test_index + test_start_pattern.len())..end_test_index].to_string();

    let tests_strings = tests.iter().fold(String::new(), |mut acc, f| {
        let a = test_str.clone();
        let a = a.replace("***input***", &f.input);
        let a = a.replace("***output***", &f.output);

        acc.push_str(&a);
        acc
    });

    // Create tests
    template.replace_range(
        start_test_index..(end_test_index + test_end_pattern.len()),
        &tests_strings,
    );
    template
}

fn submit(client: Client, year: i32, day: u32, part: u32, no_check_tests: bool) {
    println!();
    if no_check_tests {
        println!("Skipping tests");
    } else {
        println!("Running tests...");

        let output = Command::new("cargo")
            .arg("test")
            .arg("--bin")
            .arg(format!("day{}-{}", day, part))
            .output()
            .expect("Cannot run tests");

        if !output.status.success() {
            println!("Tests failed");
            println!("============= stdout =============");
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            // flush
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::io::Write::flush(&mut std::io::stderr()).unwrap();
            std::process::exit(1);
        }
    }

    println!("Tests passed");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(format!("day{}-{}", day, part))
        .output()
        .unwrap();

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::Write::flush(&mut std::io::stderr()).unwrap();

    let val = String::from_utf8(output.stderr.clone())
        .unwrap()
        .trim()
        .split(" = ")
        .last()
        .unwrap()
        .replace("\"", "");
    println!("Submit the following output? (y/n)");
    println!("{}", val);

    unimplemented!();
}
