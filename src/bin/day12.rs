use std::str::Chars;
use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::{Match, Regex};
use regex::internal::Inst;

lazy_static! {
    static ref Direction: Vec<CardinalDirections> = vec![CardinalDirections::East, CardinalDirections::South, CardinalDirections::West, CardinalDirections::North];
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum CardinalDirections {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Debug, Clone)]
enum RelativeDirections {
    Left,
    Right,
    Forward,
}

#[derive(PartialEq, Debug, Clone)]
enum Navigation {
    CardinalDirections(CardinalDirections),
    RelativeDirections(RelativeDirections),
}

#[derive(Debug, Clone)]
struct Instructions {
    inst: Vec<(Navigation, usize)>,
    heading: usize,
    route: HashMap<CardinalDirections, usize>,
    waypoint: Vec<(Navigation, usize)>,
}

trait Nav {
    fn turn_down_for_what(&mut self, degrees: usize);
}

impl Instructions {

    fn new() -> Self {
        let mut route = HashMap::new();
        route.insert(CardinalDirections::North, 0);
        route.insert(CardinalDirections::South, 0);
        route.insert(CardinalDirections::East, 0);
        route.insert(CardinalDirections::West, 0);
        Instructions{
            inst: Vec::new(),
            heading: 0,
            route,
            waypoint: Vec::new(),
        }
    }
    fn navigate(&mut self) {
        for (dir, unit) in self.inst.clone().iter() {
            match dir {
                Navigation::CardinalDirections(val) => {
                    match val {
                        CardinalDirections::North => {
                            self.move_cardinal(CardinalDirections::North, *unit)
                        }
                        CardinalDirections::South => {
                            self.move_cardinal(CardinalDirections::South, *unit)
                        }
                        CardinalDirections::East => {
                            self.move_cardinal(CardinalDirections::East, *unit)
                        }
                        CardinalDirections::West => {
                            self.move_cardinal(CardinalDirections::West, *unit)
                        }
                    }
                }
                Navigation::RelativeDirections(val) => {
                    match val {
                        RelativeDirections::Left => {
                            self.turn_left(*unit)
                        }
                        RelativeDirections::Right => {
                            self.turn_right(*unit)
                        }
                        RelativeDirections::Forward => {
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

    fn _move(&mut self, direction1: &CardinalDirections, direction2: &CardinalDirections, units: usize) {
        let s = *self.route.get_mut(direction2).unwrap();
        if s > units {
            return *self.route.get_mut(direction2).unwrap() -= units;
        }
        *self.route.get_mut(direction2).unwrap() = 0;
        let units = units - s;
        return *self.route.get_mut(direction1).unwrap() += units;
    }

    fn forward(&mut self, units: usize) {
        let dir = &Direction[self.heading];
        match dir {
            CardinalDirections::North => {
                self._move(&CardinalDirections::North, &CardinalDirections::South, units)
            }
            CardinalDirections::South => {
                self._move(&CardinalDirections::South, &CardinalDirections::North, units)
            }
            CardinalDirections::East => {
                self._move(&CardinalDirections::East, &CardinalDirections::West, units)
            }
            CardinalDirections::West => {
                self._move(&CardinalDirections::West, &CardinalDirections::East, units)
            }
        }
    }
    fn move_cardinal(&mut self, dir: CardinalDirections, units: usize) {
        *self.route.get_mut(&dir).unwrap() += units;
    }
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
                nav.inst.push((Navigation::RelativeDirections(RelativeDirections::Forward), units))
            }
            'L' => {
                nav.inst.push((Navigation::RelativeDirections(RelativeDirections::Left), units))
            }
            'R' => {
                nav.inst.push((Navigation::RelativeDirections(RelativeDirections::Right), units))
            }
            'N' => {
                nav.inst.push((Navigation::CardinalDirections(CardinalDirections::North), units))
            }
            'S' => {
                nav.inst.push((Navigation::CardinalDirections(CardinalDirections::South), units))
            }
            'E' => {
                nav.inst.push((Navigation::CardinalDirections(CardinalDirections::East), units))
            }
            'W' => {
                nav.inst.push((Navigation::CardinalDirections(CardinalDirections::West), units))
            }
            _ => {}
        }
    }
    nav.navigate();
    nav.turn_down_for_what();
    // pt1
    println!("{:?}", nav.route);
    println!("{:?}", (
        (*nav.route.get(&CardinalDirections::North).unwrap() as i32 -
        *nav.route.get(&CardinalDirections::South).unwrap() as i32).abs() +
        (*nav.route.get(&CardinalDirections::East).unwrap() as i32 -
            *nav.route.get(&CardinalDirections::West).unwrap() as i32).abs()))

    // pt2.

}
