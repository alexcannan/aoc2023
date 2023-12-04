use std::collections::HashMap;

use crate::download;


struct Stack {
    counts: Vec<i32>,
}

impl Stack {
    fn new() -> Stack {
        Stack { counts: Vec::new() }
    }

    // add 1 to the first n items of stack, create if they don't exist
    fn add(&mut self, n: usize) {
        for i in 0..n {
            if i < self.counts.len() {
                self.counts[i] += 1;
            } else {
                self.counts.push(1);
            }
        }
    }

    // pop first item from stack off
    fn pop(&mut self) -> i32 {
        if self.counts.len() == 0 {
            return 0;
        }
        let count = self.counts[0];
        self.counts.remove(0);
        return count;
    }
}


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/4/input").expect("Failed to download input");
    let mut score: i32 = 0;
    let mut card_score: i32;
    let mut count_stack: Stack = Stack::new();
    let mut n_cards: i32 = 0;
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
        let extra_cards: i32 = count_stack.pop();
        if n_wins == 0 {
            card_score = 0;
        } else {
            card_score = 2_i32.pow((n_wins-1) as u32);
            for _ in 0..extra_cards+1 {
                count_stack.add(n_wins as usize);
            }
        }
        println!("{} wins, score {}", n_wins, card_score);
        // print count stack
        println!("{} extra; count stack: {:?}", extra_cards, count_stack.counts);
        score += card_score;
        n_cards += 1 + extra_cards;
    }
    println!("Score: {}", score);
    println!("Cards: {}", n_cards);
}