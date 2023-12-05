use crate::download;


struct Submap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}


struct Map {
    mappings: Vec<Submap>,
}

impl Map {
    fn new() -> Map {
        Map { mappings: Vec::new() }
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
            let mut lines = chunk.lines();
            let _first_line = lines.next().unwrap();
            // if maps were not in order, we can unravel them with this line. but, they seem ordered.
            maps.push(Map::new());
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
    let mut min_value: u64 = u64::MAX;
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
            if value < min_value {
                min_value = value;
            }
            // println!("{} -> {}", seed, value);
        }
    }
    println!("lowest value: {}", min_value);
}