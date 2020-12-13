use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor,
    OutOfBounds,
}

#[derive(Clone)]
struct Seating {
    seats: Vec<Seat>,
    width: usize,
}

impl fmt::Display for Seating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut output: String = String::new();
        let mut row = Vec::new();
        for s in self.seats.iter() {
            if row.len() == self.width {
                output += &format!("{:?}\n", row);
                row = Vec::new();
            }
            match s {
                Seat::Empty => {
                    row.push('L');
                }
                Seat::Occupied => {
                    row.push('#')
                }
                Seat::Floor => {
                    row.push('.')
                }
                Seat::OutOfBounds => {
                    row.push('?')
                }
            }
        }
        output += &format!("{:?}\n", row);
        write!(f, "{}", output)
    }
}

impl Seating {
    fn occupied(&mut self) -> bool {
        let mut changed = false;
        let mut new_arrangement: Vec<Seat> = Vec::new();
        for (i, mut s) in self.seats.clone().iter_mut().enumerate() {
            // println!("evalutating idx {} seat {:?}", i, s);
            // short circuit floor
            if *s == Seat::Floor {
                new_arrangement.push(Seat::Floor);
                continue;
            }

            let neighbors: Vec<Seat> = vec![
                self.left(i),
                self.right(i),
                self.top(i),
                self.top_left(i),
                self.top_right(i),
                self.bottom(i),
                self.bottom_left(i),
                self.bottom_right(i)
            ];
            if i == 30 {
                println!("neighbors for idx {}:  {:?}", i,  neighbors);
            }
            if neighbors.iter().filter(|&f| *f == Seat::Occupied).count() >= 4 &&
                *s == Seat::Occupied {
                new_arrangement.push(Seat::Empty);
                changed = true;
                continue;
            }
            if neighbors.iter().filter(|&f| *f == Seat::Occupied).count() == 0 &&
                *s == Seat::Empty {
                new_arrangement.push(Seat::Occupied);
                changed = true;
                continue;
            }
            new_arrangement.push(s.clone());
        }
        self.seats = new_arrangement;
        changed
    }
    fn is_occupied(&self, check_pos: usize) -> Seat {
        if let Some(i) = self.seats.get(check_pos) {
            return i.clone();
        }
        return Seat::OutOfBounds;
    }
    // returns true if the next position is within the boundary
    fn arr_boundary_check(&self, next_pos: i32) -> bool {
        // check if past the array, or outside of their row length
        if next_pos > self.seats.len() as i32 ||
            next_pos < 0 as i32 {
            return false;
        }
        true
    }
    fn row_boundary_check(&self, cur_pos: usize, next_pos: i32) -> bool {
        if next_pos < 0 {
            return false;
        }
        let row = cur_pos / self.width;
        let right = row * self.width + self.width;
        let left = row * self.width;
        if next_pos >= right as i32 ||
            next_pos < left as i32 {
            return false;
        }
        self.arr_boundary_check(next_pos)
    }
    fn left(&self, cur_pos: usize) -> Seat {
        if self.row_boundary_check(cur_pos, cur_pos as i32 - 1) {
            return self.is_occupied(cur_pos - 1);
        }
        Seat::OutOfBounds
    }
    fn right(&self, cur_pos: usize) -> Seat {
        if self.row_boundary_check(cur_pos, cur_pos as i32 + 1) {
            return self.is_occupied(cur_pos + 1);
        }
        Seat::OutOfBounds
    }
    fn top(&self, cur_pos: usize) -> Seat {
        if self.arr_boundary_check(cur_pos as i32 - self.width as i32) {
            return self.is_occupied(cur_pos - self.width);
        }
        Seat::OutOfBounds
    }
    fn top_left(&self, cur_pos: usize) -> Seat {
        if self.row_boundary_check((cur_pos as i32 - self.width as i32) as usize, (cur_pos as i32 - self.width as i32 - 1)) {
            return self.is_occupied(cur_pos - self.width - 1);
        }
        Seat::OutOfBounds
    }
    fn top_right(&self, cur_pos: usize) -> Seat {
        if self.arr_boundary_check(cur_pos as i32  - self.width as i32) &&
            self.row_boundary_check((cur_pos - self.width) as usize, (cur_pos as i32 - self.width as i32 + 1)) {
            return self.is_occupied((cur_pos as i32 - self.width as i32 + 1)as usize);
        }
        Seat::OutOfBounds
    }
    fn bottom(&self, cur_pos: usize) -> Seat {
        if self.arr_boundary_check((cur_pos + self.width) as i32) {
            return self.is_occupied(cur_pos + self.width);
        }
        Seat::OutOfBounds
    }
    fn bottom_left(&self, cur_pos: usize) -> Seat {
        if self.arr_boundary_check((cur_pos  + self.width) as i32) &&
            self.row_boundary_check((cur_pos  + self.width) as usize, (cur_pos + self.width - 1) as i32) {
            return self.is_occupied(cur_pos + self.width - 1);
        }
        Seat::OutOfBounds
    }
    fn bottom_right(&self, cur_pos: usize) -> Seat {
        if self.arr_boundary_check((cur_pos + self.width) as i32) &&
            self.row_boundary_check((cur_pos + self.width) as usize, (cur_pos + self.width + 1) as i32) {
            return self.is_occupied(cur_pos + self.width + 1);
        }
        Seat::OutOfBounds
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day11").unwrap();
    let mut width = 0;
    let mut seats: Vec<Seat> = Vec::new();
    for l in input.lines() {
        if width == 0 {
            width = l.len();
        }
        seats.append(&mut l.chars().map(|c| match c {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '#' => Seat::Occupied,
            _ => panic!("unknown char")
        }).collect::<Vec<_>>());
    }
    let mut seating = Seating {
        seats,
        width,
    };
    println!("starting seating\n{}", seating);
    let mut cycles = 0;
    loop {
        cycles += 1;
        println!("\n\n\nstarting cycle {}", cycles);

        let has_changed = seating.occupied();
        println!("{}", seating);
        println!("has changed: {:?}", has_changed);
        if !has_changed {
            break
        }
    }
    println!("{:?}", seating.seats.iter().filter(|&f| *f == Seat::Occupied).count())
}