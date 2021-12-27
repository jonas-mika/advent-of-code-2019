use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};

const DAY : usize = 3;

fn manhatten(p : &(i32, i32) ) -> i32 {
    p.0.abs() + p.1.abs()
}

fn main() {
    let file;
    if env::args().len() > 1 {
        file = format!("{}", env::args().nth(1).unwrap());
    } else {
        file = format!("./inputs/day{}.in", DAY);
    }

    let file = File::open(file).expect("");
    let reader = BufReader::new(file);

    let coord1 : HashMap<(i32, i32), i32> = HashMap::new();
    let coord2 : HashMap<(i32, i32), i32> = HashMap::new();
    let mut coords : Vec<HashMap<(i32, i32), i32>> = vec![coord1, coord2];

    let mut i : usize = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut curr : (i32, i32) = (0, 0);
        let mut steps : i32 = 0;

        for inst in line.split(',') {
            let dir = inst.chars().nth(0).unwrap();
            let num = &inst[1..].parse::<i32>().unwrap();

            for _ in 0..*num {
                steps += 1;
                if dir == 'U' {
                    curr = (curr.0, curr.1+1);
                } else if dir == 'D' {
                    curr = (curr.0, curr.1-1);
                } else if dir == 'L' {
                    curr = (curr.0-1, curr.1);
                } else if dir == 'R' {
                    curr = (curr.0+1, curr.1);
                } else {
                    assert!(false);
                }

                if !coords[i as usize].contains_key(&curr) {
                    coords[i as usize].insert(curr, steps);
                }
            }
        }
        i+=1;
    }
    // println!("List 1: {:?}", coords[0]);
    // println!("List 2: {:?}", coords[1]);

    let mut final_coords : Vec<(i32, i32)> = Vec::new();
    for coord in coords[0].keys() {
        if coords[1].contains_key(&coord) {
            final_coords.push(*coord);
        }
    }

    // part 1
    let mut distances : Vec<i32> = Vec::new();
    let mut total_steps : Vec<i32> = Vec::new();
    for coord in final_coords.clone() {
        distances.push(manhatten(&coord));
    }
    // part 2
    for coord in final_coords.clone() {
        total_steps.push(coords[0].get(&coord).unwrap() + coords[1].get(&coord).unwrap());
    }

    println!("Part 1: {}", distances.iter().min().unwrap()); 
    println!("Part 2: {}", total_steps.iter().min().unwrap()); 
}
