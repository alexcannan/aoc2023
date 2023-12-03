use std::cmp::{min, max};

use crate::download;

use regex;


pub fn main() {
    let engine_schematic = download::get_input("https://adventofcode.com/2023/day/3/input").expect("Failed to get input");
    let engine_schematic_lines: Vec<&str> = engine_schematic.lines().collect();
    // find numbers in each line
    let gear_re = regex::Regex::new(r"\*").unwrap();
    let numbers_re = regex::Regex::new(r"\d+").unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    let mut gear_numbers: Vec<u32> = Vec::new();
    let mut expanded_start: usize;
    let mut expanded_end: usize;
    for (i, line) in engine_schematic_lines.iter().enumerate() {
        println!("line {}", line);
        for m in gear_re.find_iter(line) {
            // println!("line {}: number {}", i, m.as_str());
            // check if expanded range has chars besides numbers and `.`
            expanded_start = if m.start() > 0 { m.start() - 1 } else { 0 };
            expanded_end = if m.end() < line.len() { m.end() + 1 } else { line.len() };
            let expanded_match = &line[expanded_start..expanded_end];
            let prev_line = if i > 0 { engine_schematic_lines[i - 1] } else { "" };
            let next_line = if i < engine_schematic_lines.len() - 1 { engine_schematic_lines[i + 1] } else { "" };
            let prev_line_match = if i > 0 { &engine_schematic_lines[i - 1][expanded_start..expanded_end] } else { "" };
            let next_line_match = if i < engine_schematic_lines.len() - 1 { &engine_schematic_lines[i + 1][expanded_start..expanded_end] } else { "" };
            // prev/next matches `...` -> 0; `d..`|`dd.`|`.dd`|`..d`|`ddd` -> 1; `d.d` -> 2
            // expanded match `.*.` -> 0; `d*.`|`.*d` -> 1; `d*d` -> 2
            let n_numbers = numbers_re.find_iter(expanded_match).count() +
                numbers_re.find_iter(prev_line_match).count() +
                numbers_re.find_iter(next_line_match).count();
            if n_numbers == 2 {
                // find numbers that are inside range from prev/current/next line
                for line in [prev_line, line, next_line].iter() {
                    // println!("{} {}", line, expanded_match);
                    for nm in numbers_re.find_iter(line) {
                        if nm.end() >= m.start() && nm.start() <= m.end() {
                            // println!("{} {} {}", nm.start(), nm.end(), nm.as_str());
                            gear_numbers.push(nm.as_str().parse::<u32>().unwrap());
                        }
                    }
                }
                // add product of gear numbers to numbers
                if gear_numbers.len() == 2 {
                    numbers.push(gear_numbers[0] * gear_numbers[1]);
                    gear_numbers.clear();
                } else {
                    panic!("gear_numbers: {:?}", gear_numbers)
                }
            }
            // println!("matches: {}, {}, {}; n_numbers {}", expanded_match, prev_line_match, next_line_match, n_numbers);
        }
    }
    // print sum of numbers
    let mut sum: u32 = 0;
    for number in numbers {
        sum += number;
    }
    println!("sum: {}", sum);
}