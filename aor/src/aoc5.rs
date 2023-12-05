use std::collections::BTreeSet;

use crate::download;


struct Submap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}


struct Map {
    source: String,
    destination: String,
    mappings: Vec<Submap>,
}

impl Map {
    fn new(source: String, destination: String) -> Map {
        Map { source, destination, mappings: Vec::new() }
    }

    fn add(&mut self, destination_range_start: u64, source_range_start: u64, range_length: u64) {
        self.mappings.push(Submap { destination_range_start, source_range_start, range_length });
    }

    fn map(&self, value: u64) -> u64 {
        for submap in &self.mappings {
            if value >= submap.source_range_start && value < submap.source_range_start + submap.range_length {
                return submap.destination_range_start + value - submap.source_range_start;
            }
        }
        return value;
    }
}


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/5/input").expect("Failed to download input");
    // destination range start, source range start, range length
    // so, 52 50 48 means 91->89
    // if no mapping, don't change
    // split input by double newlines
    let split_input = input.split("\n\n");
    let mut maps: Vec<Map> = Vec::new();
    let mut starts: Vec<u64> = Vec::new();
    let mut ranges: Vec<u64> = Vec::new();
    for chunk in split_input {
        // println!("{}", chunk);
        if chunk.contains("seeds") {
            // seeds: 12 34 45 78
            let mut split_chunk = chunk.split_whitespace();
            // skip "seeds:"
            split_chunk.next();
            // each two items is a start and range, parse and add seeds as such
            while let Some(start) = split_chunk.next() {
                let start: u64 = start.parse().unwrap();
                let range: u64 = split_chunk.next().unwrap().parse().unwrap();
                starts.push(start);
                ranges.push(range);
            }
        } else if chunk.contains("map:") {
            // parse `source-to-destination map:` for source and destination strings
            // split lines, first line should be parsed for source destination, the rest should be iterated over
            let mut lines = chunk.lines();
            let first_line = lines.next().unwrap();
            let mut split_first_line = first_line.split_whitespace().nth(0).unwrap().split("-");
            let source = split_first_line.next().unwrap().to_string();
            // skip "to"
            split_first_line.next();
            let destination = split_first_line.next().unwrap().to_string();
            maps.push(Map::new(source, destination));
            // add submaps
            for line in lines {
                let mut split_line = line.split_whitespace();
                let destination_range_start: u64 = split_line.next().unwrap().parse().unwrap();
                let source_range_start: u64 = split_line.next().unwrap().parse().unwrap();
                let range_length: u64 = split_line.next().unwrap().parse().unwrap();
                maps.last_mut().unwrap().add(destination_range_start, source_range_start, range_length);
            }
        }
    }
    let mut values: Vec<u64> = Vec::new();
    let mut value: u64;
    let mut counter: u64 = 0;
    println!("total range {} million", ranges.iter().sum::<u64>() / 1000000);
    for (start, range) in starts.iter().zip(ranges.iter()) {
        for seed in *start..*start + *range {
            counter += 1;
            if counter % 1000000 == 0 {
                println!("{} million", counter / 1000000);
            }
            value = seed;
            for map in &maps {
                value = map.map(value);
            }
            values.push(value);
            // println!("{} -> {}", seed, value);
        }
    }
    println!("lowest value: {}", values.iter().min().unwrap());
}