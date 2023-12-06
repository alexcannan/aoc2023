use crate::download;


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/6/input").expect("Failed to download input");
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    let mut potential_wins: Vec<u64> = Vec::new();
    while let Some(line) = lines.next() {
        let mut split_line = line.split_whitespace();
        // skip "Time:" or "Distance:"
        split_line.next();
        // join rest of digits into one number
        let mut number = String::new();
        while let Some(digit) = split_line.next() {
            number.push_str(digit);
        }
        let number: u64 = number.parse().unwrap();
        times.push(number);
        let line = lines.next().unwrap();
        let mut split_line = line.split_whitespace();
        // skip "Distance:"
        split_line.next();
        // join rest of digits into one number
        let mut number = String::new();
        while let Some(digit) = split_line.next() {
            number.push_str(digit);
        }
        let number: u64 = number.parse().unwrap();
        distances.push(number);
    }
    println!("times: {:?}", times);
    println!("dists: {:?}", distances);
    /*
        charge time = x
        target distance = y
        actual time = y
     */
    let mut n_wins: u64;
    for (i, time) in times.iter().enumerate() {
        n_wins = 0;
        for charge_time in 1..*time {
            // solutions are above zero and below target time
            let target_distance = distances[i];
            let travel_time = target_distance / charge_time;
            let total_time = charge_time + travel_time;
            if total_time < *time {
                n_wins += 1;
            }
        }
        potential_wins.push(n_wins);
    }
    println!("potential wins: {:?}", potential_wins);
    println!("power of potential wins: {}", potential_wins.iter().product::<u64>());
}