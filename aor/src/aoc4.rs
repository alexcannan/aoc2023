use crate::download;


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/4/input").expect("Failed to download input");
    let mut score: i32 = 0;
    let mut card_score: i32;
    for line in input.lines() {
        println!("{}", line);
        // take everything after colon
        let all_numbers: &str = line.split(":").nth(1).unwrap();
        // split str into a 2-sized tuple split on pipe
        let split_numbers: Vec<&str> = all_numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<&str> = split_numbers[0].split_whitespace().collect::<Vec<&str>>();
        let my_numbers: Vec<&str> = split_numbers[1].split_whitespace().collect::<Vec<&str>>();
        let mut n_wins: i32 = 0;
        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                n_wins += 1;
            }
        }
        if n_wins == 0 {
            card_score = 0;
        } else {
            card_score = 2_i32.pow((n_wins-1) as u32);
        }
        println!("{} wins, score {}", n_wins, card_score);
        score += card_score;
    }
    println!("Score: {}", score);
}