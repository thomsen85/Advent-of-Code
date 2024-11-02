use std::fs;
use std::io::Read;

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
        #[arg(short, long, default_value = "template")]
        template_path: String,

        #[arg(short, long)]
        do_not_fetch_input: bool,
    },
    Submit,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    year: Option<i32>,

    #[arg(short, long)]
    day: Option<u32>,

    #[arg(short, long, default_value = "COOKIE")]
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

    let cookie = std::fs::read_to_string(&args.cookie_file)
        .unwrap_or_else(|_| panic!("Cannot find cookie file {}", args.cookie_file));

    let cookie = format!("session={}", cookie.trim());
    dbg!(&args);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    match args.option {
        Cmd::New {
            template_path,
            do_not_fetch_input,
        } => new(client, year, day, template_path, do_not_fetch_input),
        Cmd::Submit => submit(),
    }
}

fn new(client: Client, year: i32, day: u32, template_path: String, do_not_fetch_input: bool) {
    // Wait for the task to drop with a nice timer :)

    if !do_not_fetch_input {
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
        // loop {
        //     if clipboard.get_text().unwrap() != content {
        //         content = clipboard.get_text().unwrap();
        //         break;
        //     }
        //     std::thread::sleep(std::time::Duration::from_millis(300));
        // }

        println!("Please enter test input for test {}:", test_cases.len() + 1);
        let mut input_buf = String::new();
        std::io::stdin()
            .read_line(&mut input_buf)
            .expect("Cannot read test input from stdin");

        let input = input_buf.trim_end();
        if input.is_empty() {
            break;
        }

        println!(
            "Please enter test output for test {}:",
            test_cases.len() + 1
        );

        loop {
            let mut output_buf = String::new();
            std::io::stdin()
                .read_line(&mut output_buf)
                .expect("Cannot read test output from stdin");

            let output = output_buf.trim_end();
            if output.is_empty() {
                println!("Output cannot be empty, please enter again:");
                continue;
            }

            test_cases.push(Tests {
                input: input.to_string(),
                output: output.to_string(),
            });
            break;
        }
    }

    dbg!(&test_cases);
    let file_path = format!("{}/src/day{}_1.rs", year, day);

    let test_path = "test.rs";
    let template = fs::read_to_string(template_path).expect("Cannot remove template file");
    let code = substitute_template(&template, year, day, &test_cases);
    println!("{}", code);
    fs::write(test_path, code).expect("Cannot write test file");
    // Ask for test cases
    // Create files from template
}

fn substitute_template(template: &str, year: i32, day: u32, tests: &[Tests]) -> String {
    let mut template = template.to_string();

    template = template.replace("***year***", &year.to_string());
    template = template.replace("**day***", &day.to_string());

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

fn submit() {}
