use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY : usize = 1;

fn fuel(mut fuel : f32) -> f32 {
    let mut total : f32 = 0.0;

    while (fuel / 3.0).floor() - 2.0  > 0.0 {
        fuel = (fuel / 3.0).floor() - 2.0;
        total += fuel;
    }
    total
}

fn main() {
    let workfile : String = format!("./inputs/day{}.in", DAY);

    let file = File::open(workfile).expect("");
    let reader = BufReader::new(file);

    let mut ans1 : f32 = 0.0;
    let mut ans2 : f32 = 0.0;
    //let tests : Vec<f32> = vec![14.0, 1969.0];
    for val in reader.lines().map(|line| line.unwrap().parse::<f32>().unwrap()) {
        ans1 += (val / 3 as f32).floor() - 2 as f32;
        ans2 += fuel(val); 
    }
    println!("Part 1: {}", ans1);
    println!("Part 1: {}", ans2);
}
