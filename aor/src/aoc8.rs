use crate::download;


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/8/input").expect("Error getting input");
    // init map of str to tuple of 2 strings
    let mut map: std::collections::HashMap<&str, (&str, &str)> = std::collections::HashMap::new();
    // sequence is first line
    let sequence = input.lines().next().unwrap().trim();
    for line in input.lines().skip(2) {
        // split line into A = (B, C)
        let mut line = line.split(" = ");
        // get first element
        let first = line.next().unwrap().trim();
        // get second element
        let mut second = line.next().unwrap();
        // strip parenthesis from second
        second = &second[1..second.len()-1];
        // split second element into (B, C)
        let mut second = second.split(", ");
        let second_first = second.next().unwrap();
        let second_second = second.next().unwrap();
        // insert into map
        map.insert(first, (second_first, second_second));
    }
    println!("{}", sequence);
    println!("{:?}", map);
    // traverse map given sequence until you reach ZZZ, count number of steps
    let mut steps = 0;
    let mut current = "AAA";
    let mut looped_sequence = sequence.chars().cycle();
    while current != "ZZZ" {
        // get next element
        let next_direction = looped_sequence.next().unwrap(); // L or R
        let next_fork = map.get(current).unwrap();
        if next_direction == 'L' {
            current = next_fork.0
        } else {
            current = next_fork.1
        };
        steps += 1;
    };
    println!("steps: {}", steps);
}