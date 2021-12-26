use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY : usize = 2;

fn computer(v : &Vec<i64>, noun : i64, verb : i64) -> i64 {
    let mut nums = v.clone();
    nums[1] = noun;
    nums[2] = verb;

    let mut i = 0;
    loop {
        let op = nums[i];
        if op == 99 {
            return nums[0];
        }
        let in1 = nums[i+1] as usize;
        let in2 = nums[i+2] as usize;
        let out = nums[i+3] as usize;

        if op == 1 {
            nums[out] = nums[in1] + nums[in2];
        } else if op == 2 {
            nums[out] = nums[in1] * nums[in2];
        } else {
            assert!(false);
        }
        i+=4;
    }
}

fn main() {
    let file;
    if env::args().len() > 1 {
        file = format!("{}", env::args().nth(1).unwrap());
    } else {
        file = format!("./inputs/day{}.in", DAY);
    }

    let file = File::open(file).expect("");
    let nums : Vec<i64> = BufReader::new(file).lines().nth(0).unwrap().unwrap().
                            split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    let ans1 = computer(&nums, 12, 2);
    println!("Part 1: {}", ans1);

    let goal = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let res = computer(&nums, noun, verb);
            // println!("noun={}, verb={}, res={}", noun, verb, res);
            if res == goal {
                println!("Part 2: {}", 100 * noun + verb);
                break;
            }
        }
    }
}
