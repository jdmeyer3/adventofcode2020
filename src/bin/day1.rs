use itertools::{Itertools, Combinations};
use std::borrow::Borrow;

fn main() {
    let input = std::fs::read_to_string("input/day1").unwrap();
    let expenses = input.lines().map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("{}", expenses.len());
    let res = expenses.iter().combinations(2);
    for r in res {
        if r[0] + r[1] == 2020 {
            println!("the expenses are {} and {}", r[0], r[1]);
            println!("multiplied together they are {}", r[0] * r[1])
        }
    }
    println!("{}", expenses.len());
    let res = expenses.iter().combinations(3);
    for r in res {
        if r[0] + r[1] + r[2] == 2020 {
            println!("the expenses are {} and {} and {}", r[0], r[1], r[2]);
            println!("multiplied together they are {}", r[0] * r[1] * r[2])
        }
    }
}