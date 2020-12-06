use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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
fn decoder(c: String) -> Vec<i32> {
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
            let half = (self.0 + self.1) / 2;
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
    fn new(ticket_code: String) -> TicketDecoder {
        TicketDecoder {
            ticket_code: decoder(ticket_code),
            ticket: Ticket {
                column: Range(0, COLUMNS),
                row: Range(0, ROWS),
            },
        }
    }
    fn get_id(&mut self) -> i32 {
        let row = self.ticket.row.get_val(0, self.row_code());
        let column = self.ticket.column.get_val(0, self.column_code());
        // println!("row: {:?}", row);
        // println!("column: {:?}", column);
        (row as i32 * 8) + column as i32
    }
    fn row_code(&self) -> Vec<i32> {
        self.ticket_code[0..7].to_owned()
    }
    fn column_code(&self) -> Vec<i32> {
        self.ticket_code[7..10].to_owned()
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn not_in_seq(arr: Vec<i32>) -> i32 {
    let mut last_num = 0;
    let mut index = 0;
    let len = arr.len();
    for i in arr {
        if index == 0 {
            last_num = i;
            index+= 1;
            continue
        }
        if index < len as i32 {
            if i == last_num+1 {
                last_num = i;
                index += 1;
            } else {
                return i
            }
        }
    }
    0
}

fn main() {
    let mut largest_id = 0;

    let start = Instant::now();
    let mut input = HashMap::new();
    if let Ok(lines) = read_lines("./src/input") {
        for line in lines {
            if let Ok(code) = line {
                let mut dec = TicketDecoder::new(code.clone());
                let ticket_id = dec.get_id();
                if ticket_id > largest_id {
                    largest_id = ticket_id;
                }
                input.insert(ticket_id, code.clone());
            }
        }
    }



    let mut available_seats = Vec::new();
    let max_range = 128 * 8 + 8;
    for i in 0..max_range {
        if !input.contains_key(&i) {
            available_seats.push(i);
        }
    }
    println!("available seats: {:?}", available_seats);
    println!("your seat sir: {:?}", not_in_seq(available_seats));
    println!("largest ticket id is: {}", largest_id);
    println!("{:?} seconds for whatever you did.", start.elapsed());

    // part2
}
