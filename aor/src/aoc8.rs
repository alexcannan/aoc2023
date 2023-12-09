use crate::download;


fn gcd(mut a: u64, mut b: u64) -> u64 {
    println!("finding gcd of {} and {}", a, b);
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a // When b is zero, a is the GCD
}


fn lcm(a: u64, b: u64) -> u64 {
    return (a / gcd(a, b)) * b;
}


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
    let mut steps: u64 = 0;
    // begins on array of nodes that end in A
    let mut current: Vec<&str> = map.keys().filter(|&x| x.ends_with("A")).map(|x| *x).collect();
    let mut looped_sequence = sequence.chars().cycle();
    // postulate that each ghost will return to an end after a consistent number of steps
    // if we identify that period for each ghost, we can figure out the least common multiple when they align
    let mut steps_to_ends: Vec<Vec<u64>> = Vec::from_iter((0..current.len()).map(|_| Vec::new()));
    let mut steps_by_ghost: Vec<u64> = Vec::from_iter((0..current.len()).map(|_| 0));
    while !current.iter().all(|&x| x.ends_with("Z")) {
        if steps % 1000000 == 0 {
            println!("steps: {}", steps);
        }
        // if we have a consistent period for each ghost, break
        if steps_to_ends.iter().all(|x| x.len() > 0) {
            break;
        }
        // println!("{:?}", current);
        let next_direction = looped_sequence.next().unwrap(); // L or R
        for i in 0..current.len() {
            if current[i].ends_with("Z") {
                steps_to_ends[i].push(steps_by_ghost[i]);
                steps_by_ghost[i] = 0;
            }
            // get next fork
            let next_fork = map.get(current[i]).unwrap();
            if next_direction == 'L' {
                current[i] = next_fork.0
            } else {
                current[i] = next_fork.1
            };
            steps_by_ghost[i] += 1;
        }
        steps += 1;
    };
    println!("broke loop");
    let periods: Vec<&u64> = steps_to_ends.iter().map(|x| x.last().unwrap()).collect::<Vec<_>>();
    // 17141, 12083, 22199, 18827, 20513, 19951
    // find least common multiple of the periods
    let mut lcm_val: u64 = 1;
    for i in 0..periods.len() {
        lcm_val = lcm(lcm_val, *periods[i]);
    }
    println!("steps: {}", lcm_val);
}