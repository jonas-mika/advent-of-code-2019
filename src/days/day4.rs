use std::env; use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_at_least_pair(num : i32) -> bool {
    let mut prev = -1;
    let nums : Vec<i32> = num.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    for c in nums.iter() {
        if *c == prev {
            return true;
        }
        prev = *c;
    }
    false
}

fn has_pair(num : i32) -> bool {
    let mut counts : Vec<(i32, i32)> = Vec::new();
    let nums : Vec<i32> = num.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

    for c in nums.iter() {
        if let Some(x) = counts.last() {
            if x.1 == *c {
                let len = counts.len();
                counts[len - 1] = (x.0 + 1, x.1);
            } else {
                counts.push((1, *c));
            }
        } else {
            counts.push((1, *c));
        }
    }

    let has_pair = counts.iter().map(|x| x.0).collect::<Vec<i32>>();

    if has_pair.contains(&2) {
        return true;
    }
    false
}

fn strictly_increasing(num : i32) -> bool {
    let mut prev = -1;
    for c in num.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32) {
        if c < prev {
            return false
        }
        prev = c;
    }
    return true;
}

fn is_valid1(mut num : i32) -> bool {
    if strictly_increasing(num) {
        if has_at_least_pair(num) {
            return true;
        }
    } 
    false
}

fn is_valid2(mut num : i32) -> bool {
    if strictly_increasing(num) {
        if has_pair(num) {
            return true;
        }
    } 
    false
}

pub fn solve() {
    let l = 147981;
    let h = 691423;
    let mut c1 = 0;
    let mut c2= 0;

    for num in l..h+1 {
        if is_valid1(num) {
            c1+=1;
        }
        if is_valid2(num) {
            c2+=1;
        }
    }
    println!("Part 1: {}", c1);
    println!("Part 2: {}", c2);
}
