extern crate regex;
use regex::Regex;
use std::borrow::Borrow;
use log::{debug, error, log_enabled, info, Level};
use env_logger;

#[derive(Debug, Default)]
struct PassportCredentials<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> PassportCredentials<'a> {
    fn is_present(&self) -> bool {
        if self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some() {
            return true;
        }
        false
    }
    fn is_valid(&self) -> bool {
        debug!("{:?}", self);
        if self.valid_yr(self.byr, 1920, 2002) &&
            self.valid_yr(self.iyr, 2010, 2020) &&
            self.valid_yr(self.eyr, 2020, 2030) &&
            self.valid_hgt() &&
            self.valid_hcl() &&
            self.valid_ecl() &&
            self.valid_pin() {
            return true;
        };
        false
    }
    fn valid_yr(&self, yr: Option<&str>, beg: i32, end: i32) -> bool {
        if yr.is_none() {
            debug!("invalid: missing yr");
            return false
        };
        let yr = yr.unwrap().parse::<i32>().unwrap();
        if yr >= beg && yr <= end {
            return true
        };
        debug!("invalid yr");
        false
    }
    fn valid_hgt(&self) -> bool{
        if self.hgt.is_none() {
            debug!("invalid: missing hgt");
            return false
        };
        let re = Regex::new(r"([0-9]+)(\w+)").unwrap();
        let cap = re.captures(self.hgt.unwrap()).unwrap();
        let height = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let unit = cap.get(2).unwrap().as_str();
        if unit == "in" {
            if height >= 59 && height <=76 {
                return true
            };
            debug!("invalid height");
            return false
        }

        // assumes cm
        if height >= 150 && height <= 193 {
            return true
        };
        debug!("invalid height");
        false
    }
    fn valid_hcl(&self) -> bool {
        if self.hcl.is_none() {
            debug!("invalid: missing hcl");
            return false
        };

        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        if re.is_match(self.hcl.unwrap()) {
            return true;
        }
        debug!("invalid hcl");
        false
    }

    fn valid_ecl(&self) -> bool {
        if self.ecl.is_none() {
            debug!("invalid: missing ecl");
            return false;
        }
        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if valid_colors.contains(&self.ecl.unwrap()) {
            return true;
        }
        debug!("invalid ecl");
        false
    }
    fn valid_pin(&self) -> bool {
        if self.pid.is_none() {
            debug!("invalid: missing pid");
            return false
        };

        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        if re.is_match(self.pid.unwrap()) {
            return true;
        }
        debug!("invalid pin");
        false
    }


    fn add_credentials(&mut self, creds: &'a str) {
        let mut cred_arr: Vec<&str> = creds.split(" ").collect();
        for cred in cred_arr {
            if cred == "" {
                continue;
            }
            let mut cred_split = cred.split(":");
            let field = cred_split.next().unwrap();
            let data = cred_split.next().unwrap();
            match field {
                "byr" => self.byr = Some(data),
                "iyr" => self.iyr = Some(data),
                "eyr" => self.eyr = Some(data),
                "hgt" => self.hgt = Some(data),
                "hcl" => self.hcl = Some(data),
                "ecl" => self.ecl = Some(data),
                "pid" => self.pid = Some(data),
                "cid" => self.cid = Some(data),
                _ => {}
            }
        }
    }
}

fn register_passport(input: &str) -> Vec<PassportCredentials> {
    let mut passports: Vec<PassportCredentials> = Vec::new();

    passports.push(PassportCredentials::default());
    for i in input.lines() {
        if i.is_empty() {
            passports.push(PassportCredentials::default());
            continue;
        }
        passports.last_mut().unwrap().add_credentials(i);
    }
    passports
}


pub fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("./input/day4").unwrap();
    let passports = register_passport(input.as_str());

    println!("passports with present fields: {:?}", passports.iter().filter(|f| f.is_present()).count());
    println!("passports with valid field: {:?}", passports.iter().filter(|f| f.is_valid()).count());
}
