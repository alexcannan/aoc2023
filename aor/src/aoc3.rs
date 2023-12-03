use std::cmp::{min, max};

use crate::download;

use regex;


pub fn main() {
    let engine_schematic = download::get_input("https://adventofcode.com/2023/day/3/input").expect("Failed to get input");
    let engine_schematic_lines: Vec<&str> = engine_schematic.lines().collect();
    // find numbers in each line
    let number_re = regex::Regex::new(r"\d+").unwrap();
    let symbol_re = regex::Regex::new(r"[^0-9.]").unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    let mut expanded_start: usize;
    let mut expanded_end: usize;
    for (i, line) in engine_schematic_lines.iter().enumerate() {
        println!("line {}", line);
        for m in number_re.find_iter(line) {
            // println!("line {}: number {}", i, m.as_str());
            // check if expanded range has chars besides numbers and `.`
            expanded_start = if m.start() > 0 { m.start() - 1 } else { 0 };
            expanded_end = if m.end() < line.len() { m.end() + 1 } else { line.len() };
            let expanded_match = &line[expanded_start..expanded_end];
            let prev_line_match = if i > 0 { &engine_schematic_lines[i - 1][expanded_start..expanded_end] } else { "" };
            let next_line_match = if i < engine_schematic_lines.len() - 1 { &engine_schematic_lines[i + 1][expanded_start..expanded_end] } else { "" };
            let prev_has_symbols = symbol_re.is_match(prev_line_match);
            let next_has_symbols = symbol_re.is_match(next_line_match);
            let expanded_has_symbols = symbol_re.is_match(expanded_match);
            println!("match lines: {} {}, {} {}, {} {}", prev_line_match, prev_has_symbols, expanded_match, expanded_has_symbols, next_line_match, next_has_symbols);
            if prev_has_symbols || expanded_has_symbols || next_has_symbols {
                println!("line {}: number {} has symbols", i, m.as_str());
                numbers.push(m.as_str().parse::<u32>().unwrap());
            }
        }
    }
    // print sum of numbers
    let mut sum: u32 = 0;
    for number in numbers {
        sum += number;
    }
    println!("sum: {}", sum);
}