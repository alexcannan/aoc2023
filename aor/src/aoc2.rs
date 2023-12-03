use crate::download;


struct BallConfiguration {
    red: i32,
    green: i32,
    blue: i32,
}


pub fn main () {
    let cube_lines = download::get_input("https://adventofcode.com/2023/day/2/input").expect("Failed to get input");
    // expect 12 red, 13 green, and 14 blue balls
    let ball_config: BallConfiguration = BallConfiguration {red: 12, green: 13, blue: 14};
    let mut ball_draw = BallConfiguration {red: 0, green: 0, blue: 0};  // initialize to 0
    let mut ball_min = BallConfiguration {red: 0, green: 0, blue: 0};  // initialize to 0
    let mut good_draw;
    let mut id_total: i32 = 0;
    for line in cube_lines.lines() {
        // println!("{}", line);
        good_draw = true;
        // lines are like Game 65: 7 red, 7 blue; 3 blue, 1 red, 1 green; 3 red, 8 blue
        // parse game id and draws separated by semicolons
        let game_id_and_draws: Vec<&str> = line.split(":").collect();
        let game_id: i32 = game_id_and_draws[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().expect("Failed to parse game id");
        let draws: Vec<&str> = game_id_and_draws[1].split(";").collect();
        // parse draws
        for draw in draws {
            // reset draw count
            ball_draw.red = 0; ball_draw.green = 0; ball_draw.blue = 0;
            // reset min count
            ball_min.red = 0; ball_min.green = 0; ball_min.blue = 0;
            // println!("{}", draw);
            let balls: Vec<&str> = draw.split(',').collect();
            for ball in balls {
                // println!("{}", ball);
                let ball_color_and_count: Vec<&str> = ball.split_whitespace().collect();
                let ball_color: &str = ball_color_and_count[1];
                let ball_count: i32 = ball_color_and_count[0].parse::<i32>().expect("Failed to parse ball count");
                // fill draw counts
                match ball_color {
                    "red" => ball_draw.red += ball_count,
                    "green" => ball_draw.green += ball_count,
                    "blue" => ball_draw.blue += ball_count,
                    _ => panic!("Unknown ball color"),
                }
            }
            // check that draw does not exceed ball configuration
            if ball_draw.red > ball_config.red || ball_draw.green > ball_config.green || ball_draw.blue > ball_config.blue {
                good_draw = false;
                break;
            } else {
                continue;
            }
        }
        // if draw is good, add game id to total
        if good_draw {
            id_total += game_id;
        }
    }
    println!("{}", id_total)
}
