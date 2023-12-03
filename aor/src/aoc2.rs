use crate::download;


struct BallConfiguration {
    red: i32,
    green: i32,
    blue: i32,
}


pub fn main () {
    let cube_lines: String = download::get_input("https://adventofcode.com/2023/day/2/input").expect("Failed to get input");
    let mut ball_min = BallConfiguration {red: 0, green: 0, blue: 0};  // initialize to 0
    let mut draw_power: i32;
    let mut min_power_total = 0;
    for line in cube_lines.lines() {
        println!("{}", line);
        // lines are like Game 65: 7 red, 7 blue; 3 blue, 1 red, 1 green; 3 red, 8 blue
        // parse game id and draws separated by semicolons
        let game_id_and_draws: Vec<&str> = line.split(":").collect();
        let draws: Vec<&str> = game_id_and_draws[1].split(";").collect();
        // reset min count
        ball_min.red = 0; ball_min.green = 0; ball_min.blue = 0;
        // parse draws
        for draw in draws {
            // println!("{}", draw);
            let balls: Vec<&str> = draw.split(',').collect();
            for ball in balls {
                // println!("{}", ball);
                let ball_color_and_count: Vec<&str> = ball.split_whitespace().collect();
                let ball_color: &str = ball_color_and_count[1];
                let ball_count: i32 = ball_color_and_count[0].parse::<i32>().expect("Failed to parse ball count");
                // fill min counts
                match ball_color {
                    "red" => if ball_min.red < ball_count {ball_min.red = ball_count},
                    "green" => if ball_min.green < ball_count {ball_min.green = ball_count},
                    "blue" => if ball_min.blue < ball_count {ball_min.blue = ball_count},
                    _ => panic!("Unknown ball color"),
                }
            }
            // print ball min
            println!("mins r{} g{} b{}", ball_min.red, ball_min.green, ball_min.blue);
        }
        // calculate min power (product of each min value)
        draw_power = ball_min.red * ball_min.green * ball_min.blue;
        println!("draw power {}", draw_power);
        min_power_total += draw_power;
    }
    println!("{}", min_power_total);
}
