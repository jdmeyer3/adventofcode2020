use std::str::Chars;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::fmt::Display;

use regex::{Match, Regex};
use regex::internal::Inst;
use std::fmt::{Debug, Formatter};
use std::fmt;

lazy_static! {
    static ref Direction: Vec<Cardinal> = vec![Cardinal::East, Cardinal::South, Cardinal::West, Cardinal::North];
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Debug, Clone)]
enum Relative {
    Left,
    Right,
    Forward,
}

#[derive(PartialEq, Debug, Clone)]
enum Navigation {
    Cardinal(Cardinal),
    Relative(Relative),
}

trait ShipMovement {
    fn new() -> Self;
    fn navigate(&mut self);
    fn turn_left(&mut self, degrees: usize);
    fn turn_right(&mut self, degrees: usize);
    fn forward(&mut self, units: usize);
}

#[derive(Debug, Clone)]
struct Instructions {
    inst: Vec<(Navigation, usize)>,
    heading: usize,
    route: HashMap<Cardinal, usize>,
}

#[derive(Debug, Clone)]
struct WaypointInstructions {
    inst: Vec<(Navigation, usize)>,
    ship_route: HashMap<Cardinal, usize>,
    waypoint: HashMap<Cardinal, usize>
}

impl Instructions {
    fn _move(&mut self, direction1: &Cardinal, direction2: &Cardinal, units: usize) {
        let s = *self.route.get_mut(direction2).unwrap();
        if s > units {
            return *self.route.get_mut(direction2).unwrap() -= units;
        }
        *self.route.get_mut(direction2).unwrap() = 0;
        let units = units - s;
        return *self.route.get_mut(direction1).unwrap() += units;
    }
    fn move_cardinal(&mut self, dir: Cardinal, units: usize) {
        *self.route.get_mut(&dir).unwrap() += units;
    }
}

impl ShipMovement for Instructions {

    fn new() -> Self {
        let mut route = HashMap::new();
        route.insert(Cardinal::North, 0);
        route.insert(Cardinal::South, 0);
        route.insert(Cardinal::East, 0);
        route.insert(Cardinal::West, 0);
        Instructions{
            inst: Vec::new(),
            heading: 0,
            route,
        }
    }
    fn navigate(&mut self) {
        for (dir, unit) in self.inst.clone().iter() {
            match dir {
                Navigation::Cardinal(val) => {
                    match val {
                        Cardinal::North => {
                            self.move_cardinal( Cardinal::North, *unit)
                        }
                        Cardinal::South => {
                            self.move_cardinal(Cardinal::South, *unit)
                        }
                        Cardinal::East => {
                            self.move_cardinal(Cardinal::East, *unit)
                        }
                        Cardinal::West => {
                            self.move_cardinal(Cardinal::West, *unit)
                        }
                    }
                }
                Navigation::Relative(val) => {
                    match val {
                        Relative::Left => {
                            self.turn_left(*unit)
                        }
                        Relative::Right => {
                            self.turn_right(*unit)
                        }
                        Relative::Forward => {
                            self.forward(*unit)
                        }
                    }
                }
            }
        }
    }
    fn turn_left(&mut self, degrees: usize) {
        let turns: i32 = ((degrees as i32 / 90) / -1) as i32;
        self.heading = ((turns + Direction.len() as i32 + self.heading as i32).abs() % 4) as usize;
    }
    fn turn_right(&mut self, degrees: usize) {
        let turns: i32 = (degrees as i32 / 90) as i32;
        self.heading = ((turns + Direction.len() as i32 + self.heading as i32).abs() % 4) as usize;
    }



    fn forward(&mut self, units: usize) {
        let dir = &Direction[self.heading];
        match dir {
            Cardinal::North => {
                self._move(&Cardinal::North, &Cardinal::South, units)
            }
            Cardinal::South => {
                self._move(&Cardinal::South, &Cardinal::North, units)
            }
            Cardinal::East => {
                self._move(&Cardinal::East, &Cardinal::West, units)
            }
            Cardinal::West => {
                self._move(&Cardinal::West, &Cardinal::East, units)
            }
        }
    }
}


fn pt2(input: &str) -> usize {
    let mut ship: [usize; 4] = [0; 4];
    let mut waypoint: [usize; 4] = [1, 10, 0, 0];
    println!("starting\n\nwaypoint {:?}", waypoint);
    println!("ship {:?}", ship);
    for line in input.lines() {
        let (action, val) = line.split_at(1);
        let num = val.parse::<usize>().unwrap();
        println!("action {:?}{:?}", action, num);
        match action {
            "N" => waypoint[0] += num,
            "E" => waypoint[1] += num,
            "S" => waypoint[2] += num,
            "W" => waypoint[3] += num,
            "R" => {
                let turn = (num / 90);
                waypoint.rotate_right(turn);
            }
            "L" => {
                let turn = (num / 90);
                waypoint.rotate_left(turn);
            }
            "F" => {
                for (i, v) in waypoint.iter().enumerate() {
                    ship[i] += *v * num;
                }
                // let n = waypoint[0] as i32 - waypoint[2] as i32;
                // println!("n {:?}", n);
                // if n > 0 {
                //     ship[0] += n as usize * num;
                // } else {
                //     ship[2] += n.abs() as usize * num;
                // }
                // let e = waypoint[1] as i32 - waypoint[3] as i32;
                // println!("e {:?}", e);
                // if e > 0 {
                //     ship[1] += e as usize * num;
                // } else {
                //     ship[3] += e.abs() as usize * num;
                // }

            }
            &_ => {}
        }
        println!("\n\nwaypoint {:?}", waypoint);
        println!("ship {:?}", ship);
    }
    return ((ship[0] as i32 - ship[2] as i32).abs() +
        (ship[1] as i32 - ship[3] as i32).abs()) as usize
}


fn main() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    let inst_re = Regex::new(r"(\w)(\d+)").unwrap();
    let inst_cap = input
        .lines()
        .map(|l| inst_re.captures(l).unwrap())
        .collect::<Vec<_>>();

    let mut nav = Instructions::new();
    for i in inst_cap {
        let direction = i.get(1).unwrap().as_str().chars().last().unwrap();
        let units: usize = String::from(i.get(2).unwrap().as_str()).parse::<usize>().unwrap();
        match direction {
            'F' => {
                nav.inst.push((Navigation::Relative(Relative::Forward), units))
            }
            'L' => {
                nav.inst.push((Navigation::Relative(Relative::Left), units))
            }
            'R' => {
                nav.inst.push((Navigation::Relative(Relative::Right), units))
            }
            'N' => {
                nav.inst.push((Navigation::Cardinal(Cardinal::North), units))
            }
            'S' => {
                nav.inst.push((Navigation::Cardinal(Cardinal::South), units))
            }
            'E' => {
                nav.inst.push((Navigation::Cardinal(Cardinal::East), units))
            }
            'W' => {
                nav.inst.push((Navigation::Cardinal(Cardinal::West), units))
            }
            _ => {}
        }
    }
    nav.navigate();

    // pt1
    println!("{:?}", nav.route);
    println!("{:?}", (
        (*nav.route.get(&Cardinal::North).unwrap() as i32 -
        *nav.route.get(&Cardinal::South).unwrap() as i32).abs() +
        (*nav.route.get(&Cardinal::East).unwrap() as i32 -
            *nav.route.get(&Cardinal::West).unwrap() as i32).abs()));

    // pt2.
    println!("{:?}", pt2(&input));

}
