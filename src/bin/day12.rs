use std::str::Chars;
use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::{Match, Regex};
use regex::internal::Inst;

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

impl WaypointInstructions{
    fn get_directions(&self) -> Vec<usize> {
        let north = self.waypoint.get(&Cardinal::North).unwrap().to_owned();
        let east = self.waypoint.get(&Cardinal::East).unwrap().to_owned();
        let south = self.waypoint.get(&Cardinal::South).unwrap().to_owned();
        let west = self.waypoint.get(&Cardinal::West).unwrap().to_owned();

        vec![north, east, south, west]
    }
    fn set_direction(&mut self, directions: Vec<usize>) {
        *self.waypoint.get_mut(&Cardinal::North).unwrap() = directions[0];
        *self.waypoint.get_mut(&Cardinal::East).unwrap() = directions[1];
        *self.waypoint.get_mut(&Cardinal::South).unwrap()= directions[2];
        *self.waypoint.get_mut(&Cardinal::West).unwrap()= directions[3];
    }
    fn _move(&mut self, mut direction1: (Cardinal, usize), mut direction2: (Cardinal, usize), units: usize) -> ((Cardinal, usize), (Cardinal, usize)) {
        if direction2.1 > units {
            return (direction1, (direction2.0, direction2.1 - units));
        }
        return ((direction1.0, direction1.1 + units), (direction2.0, 0));
    }
    fn move_cardinal(&mut self, dir: Cardinal, units: usize) {
        match dir {
            Cardinal::North => {
                let n = self.waypoint
                    .get_key_value(&Cardinal::North)
                    .unwrap();
                let s = self.waypoint
                    .get_key_value(&Cardinal::North)
                    .unwrap();
                let dirs = ((n.0.to_owned(), n.1.to_owned()), (
                    n.1.to_owned(), n.1.to_owned()));
                *self.waypoint.
            }
            Cardinal::South => {}
            Cardinal::East => {}
            Cardinal::West => {}
        }
    }
}

impl ShipMovement for WaypointInstructions {
    fn new() -> Self {
        let mut route = HashMap::new();
        route.insert(Cardinal::North, 0);
        route.insert(Cardinal::South, 0);
        route.insert(Cardinal::East, 0);
        route.insert(Cardinal::West, 0);
        let mut waypoint = route.clone();
        WaypointInstructions{
            inst: Vec::new(),
            ship_route: route,
            waypoint,
        }
    }

    fn navigate(&mut self) {
        unimplemented!()
    }

    fn turn_left(&mut self, degrees: usize) {
        let turns: i32 = ((degrees as i32 / 90) / -1) as i32;
        let mut directions = self.get_directions().clone();
        directions.rotate_left(turns as usize);
        self.set_direction(directions);
    }
    fn turn_right(&mut self, degrees: usize) {
        let turns: i32 = ((degrees as i32 / 90) / -1) as i32;
        let mut directions = self.get_directions();
        directions.rotate_right(turns as usize);
        self.set_direction(directions);
    }

    // moves the ship to the waypoint 10 times, keeps waypoint at location
    fn forward(&mut self, units: usize) {
        for (waypoint_dir, waypoint_units) in self.waypoint.clone().iter() {
            match waypoint_dir {
                Cardinal::North => {
                    *self.ship_route.get_mut(&Cardinal::North).unwrap() += (waypoint_units * units);
                }
                Cardinal::South => {
                    *self.ship_route.get_mut(&Cardinal::South).unwrap() += (waypoint_units * units);
                }
                Cardinal::East => {
                    *self.ship_route.get_mut(&Cardinal::East).unwrap() += (waypoint_units * units);
                }
                Cardinal::West => {
                    *self.ship_route.get_mut(&Cardinal::West).unwrap() += (waypoint_units * units);
                }
            }
        }
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
            *nav.route.get(&Cardinal::West).unwrap() as i32).abs()))

    // pt2.

}
