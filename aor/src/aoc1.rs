use regex;


fn aoc1() {
    fn parse_digit(input: &str) -> Option<i32> {
        println!("{}", input);
        match input {
            "one" | "1" => Some(1),
            "two" | "2" => Some(2),
            "three" | "3" => Some(3),
            "four" | "4" => Some(4),
            "five" | "5" => Some(5),
            "six" | "6" => Some(6),
            "seven" | "7" => Some(7),
            "eight" | "8" => Some(8),
            "nine" | "9" => Some(9),
            "zero" | "0" => Some(0),
            _ => None,
        }
    }

    let calibration_lines = get_input("https://adventofcode.com/2023/day/1/input").expect("Failed to get input");
    let lines: Vec<&str> = calibration_lines.split_whitespace().collect();
    // println!("{:#?}", lines);
    // for each string, find first number going forwards and backwards; these are digits #1 and #2 of the calibration number
    let mut sum: i32 = 0;
    for line in lines {
        println!("{}", line);
        // build regex that matches digits or spelled out digits `one` through `nine`
        let search: &str = r"(\d|zero|one|two|three|four|five|six|seven|eight|nine)";
        let re = regex::Regex::new(search).unwrap();
        // find array of matches
        let mut all_matches: Vec<&str> = Vec::new();
        let mut start = 0;
        while start < line.len() {
            if let Some(match_) = re.find(&line[start..]) {
                all_matches.push(match_.as_str());
                start += match_.start() + 1;
            } else {
                break;
            }
        }
        // create vector of matches as strs
        let digit1 = parse_digit(all_matches[0]).expect("failed to parse digit1");
        let digit2 = parse_digit(all_matches[all_matches.len()-1]).expect("failed to parse digit2");
        sum += digit1 * 10 + digit2;
    }
    println!("{}", sum);
}
