use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Default)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<u32>,
    hcl: Option<usize>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn set_byr(&mut self, val: &str) {
        self.byr = val.parse::<u32>().ok().filter(|y| *y >= 1920 && *y <= 2002)
    }

    fn set_iyr(&mut self, val: &str) {
        self.iyr = val.parse::<u32>().ok().filter(|y| *y >= 2010 && *y <= 2020)
    }

    fn set_eyr(&mut self, val: &str) {
        self.eyr = val.parse::<u32>().ok().filter(|y| *y >= 2020 && *y <= 2030)
    }

    fn set_hgt(&mut self, val: &str) {
        self.hgt =
            val[..(val.len() - 2)]
                .parse::<u32>()
                .ok()
                .filter(|h| match &val[(val.len() - 2)..] {
                    "cm" => *h >= 150 && *h <= 193,
                    "in" => *h >= 59 && *h <= 76,
                    _ => false,
                })
    }

    fn set_hcl(&mut self, val: &str) {
        self.hcl = Some(val)
            .filter(|s| s.len() == 7 && s.find('#').unwrap_or(1) == 0)
            .and_then(|_| usize::from_str_radix(&val[1..], 16).ok())
    }

    fn set_ecl(&mut self, val: &str) {
        self.ecl = match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(val.to_owned()),
            _ => None,
        }
    }

    fn set_pid(&mut self, val: &str) {
        self.pid =
            Some(val.to_owned()).filter(|v| v.len() == 9 && v.chars().all(|c| c.is_numeric()))
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

fn parse(p: &mut Passport, attr: &str) {
    let parts: Vec<&str> = attr.split(':').collect();
    if parts.len() != 2 {
        return;
    }
    match parts[0] {
        "byr" => p.set_byr(parts[1]),
        "iyr" => p.set_iyr(parts[1]),
        "eyr" => p.set_eyr(parts[1]),
        "hgt" => p.set_hgt(parts[1]),
        "hcl" => p.set_hcl(parts[1]),
        "ecl" => p.set_ecl(parts[1]),
        "pid" => p.set_pid(parts[1]),
        _ => {}
    }
}

fn main() {
    let file = File::open("aoc2020/inputs/day04.input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut passports = Vec::new();
    let mut this_passport = Passport::default();
    for l in lines {
        if l.is_empty() {
            passports.push(this_passport);
            this_passport = Passport::default();
            continue;
        }
        for attr in l.split_whitespace() {
            parse(&mut this_passport, attr);
        }
    }
    passports.push(this_passport);
    let valid = passports.iter().filter(|&p| p.is_valid()).count();
    println!("{:?}", valid);
}
