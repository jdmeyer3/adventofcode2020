use std::time::{Duration, Instant};

const ROWS: u8 = 128;
const COLUMNS: u8 = 8;

struct Ticket {
    row: Range,
    column: Range,
}

struct TicketDecoder {
    ticket_code: Vec<i32>,
    ticket: Ticket,
}

struct Range(u8, u8);  // should be the starting value of the range

// decoder will return "1" for upper half, and "0" for lower half
fn decoder(c: &str) -> Vec<i32> {
    let mut v = Vec::new();
    for i in c.as_bytes() {
        if *i == 70 || *i == 76 { // if F or L
            v.push(0);
        } else {
            v.push(1);
        }
    };
    v
}

impl Range {
    fn get_val(&mut self, mut code_index: usize, code: Vec<i32>) -> u8 {
        if code_index == code.len() - 1 {
            return if code[code_index] == 1 {
                self.1 - 1
            } else {
                self.0
            };
        } else {
            // find half
            let half = ((self.0 + self.1) / 2);
            if code[code_index] as i32 == 1 {
                self.0 = half;
                code_index += 1;
                self.get_val(code_index, code)
            } else {
                self.1 = half;
                code_index += 1;
                self.get_val(code_index, code)
            }
        }
    }
}

impl TicketDecoder {
    fn new(ticket_code: &str) -> TicketDecoder {
        TicketDecoder {
            ticket_code: decoder(ticket_code),
            ticket: Ticket {
                column: Range(0, COLUMNS),
                row: Range(0, ROWS),
            },
        }
    }
    fn get_id(&mut self) -> usize {
        let row = self.ticket.row.get_val(0, self.row_code());
        let column = self.ticket.column.get_val(0, self.column_code());
        (row as usize * 8) + column as usize
    }
    fn row_code(&self) -> Vec<i32> {
        self.ticket_code[0..7].to_owned()
    }
    fn column_code(&self) -> Vec<i32> {
        self.ticket_code[7..10].to_owned()
    }
}

fn get_passport(passport: &str) -> usize {
    let mut dec = TicketDecoder::new(passport);
    dec.get_id()

}

fn main() {
    // gracious enhanced code from https://github.com/baszalmstra/adventofcode2020/blob/main/src/bin/day5.rs

    let start = Instant::now();

    let input = std::fs::read_to_string("./src/input").unwrap();
    let passports = input.lines().map(get_passport).collect::<Vec<_>>();

    let max = passports.iter().max().unwrap();
    println!("the maximum ticket id is {:?}", max);
    let min = passports.iter().min().unwrap();
    println!("the min ticket id is {:?}", min);


    let mut available_seats = vec![false; *max];
    for idx in passports.iter() {
        available_seats[idx - min] = true;
    }
    let mut entry = available_seats.into_iter().enumerate().filter(|p| !p.1);
    println!("your ticket number is: {}", entry.next().unwrap().0 + min);


    println!("{:?} seconds for whatever you did.", start.elapsed());

    // part2
}
