use std::env::var;
use std::io;
use std::process;

use ureq::Error;


fn get_input(url: &str) -> Result<String, io::Error> {
    /*
    download input from advent of code. be sure to set AOC_SESSION environment variable to your session cookie
     */
    let session_cookie = var("AOC_SESSION").expect("AOC_SESSION environment variable not set");
    match ureq::get(url).set("Cookie", &format!("session={}", session_cookie)).call() {
        Ok(response) => {
            response.into_string().map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to read response"))
        },
        Err(Error::Status(code, _response)) => {
            Err(io::Error::new(io::ErrorKind::Other, format!("Server returned status code: {}", code)))
        }
        Err(_) => {
            Err(io::Error::new(io::ErrorKind::Other, "Request failed"))
        }
    }
}

fn aoc1() {
    let calibration_lines = get_input("https://adventofcode.com/2023/day/1/input").expect("Failed to get input");
    let lines: Vec<&str> = calibration_lines.split_whitespace().collect();
    // println!("{:#?}", lines);
    // for each string, find first number going forwards and backwards; these are digits #1 and #2 of the calibration number
    let mut sum: i32 = 0;
    for line in lines {
        // println!("{}", line);
        let digit1 = line.chars().find(|c| c.is_digit(10));
        let digit2 = line.chars().rfind(|c| c.is_digit(10));
        let calibration_number: i32 = (digit1.unwrap().to_string() + &digit2.unwrap().to_string()).parse().expect("not an integer");
        sum += calibration_number;
    }
    println!("{}", sum);
}


fn main() {
    println!("Hello, aoc world!");
    aoc1();
    process::exit(0);
}