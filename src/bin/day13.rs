use std::num::ParseIntError;

fn pt1(input: &str) {
    let mut start_time = 0;
    let mut buses: Vec<usize> = Vec::new();
    for line in input.lines() {
        if start_time == 0 {
            start_time = line.parse::<usize>().unwrap();
        }
        let times = line.split(",").collect::<Vec<&str>>();
        for time in times {
            let t = time.parse::<usize>();
            match t {
                Ok(t) => buses.push(t),
                Err(_) => {}
            }
        }
    }
    let mut bus_id: usize = 0;
    let mut end_time = start_time;
    'outer: loop {
        end_time += 1;
        for b in &buses {
            if end_time % *b == 0 {
                bus_id = *b;
                break 'outer
            }
        }
    }
    println!("{} to {}", start_time, end_time);
    println!("{}", (end_time - start_time) * bus_id );
}

fn pt2() {}

fn main() {
    let input = std::fs::read_to_string("input/day13").unwrap();
    pt1(&input)
    // TODO: pt2: just didn't have time to figure out Chinese Remainder Theorum
}