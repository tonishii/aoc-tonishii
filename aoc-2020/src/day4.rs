use std::{fs::File, io::Read};
use regex::Regex;

const PART: &str = "part2";
const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const MEASUREMENTS: [&str; 2] = ["cm", "in"];
const EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<String> = input_str
    .split("\n\n")
    .map(|s| s.to_string())
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

fn part1(input: Vec<String>) -> u32 {
  // O(n * 7) which is just O(n)
  let mut res: u32 = 0;
  for passport in &input {
    for field in FIELDS {
      if !passport.contains(field) {
        res += 1;
        break;
      }
    }
  }

  // Function completes in O(n)
  return input.len() as u32 - res;
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not. Already known.
fn part2(input: Vec<String>) -> u32 {
  // O(n * 7) which is just O(n)
  let mut res: u32 = 0;
  for passport in &input {
    // let single_line = passport.replace('\n', " ");
    // println!("{}", single_line);

    for field in passport.split(" ") {
      let (curr_field, value) = field.split_once(':').unwrap();

      let eval_res = if curr_field == "byr" {
        eval_yr(value, 1920, 2002)
      } else if curr_field == "iyr" {
        eval_yr(value, 2010, 2020)
      } else if curr_field == "eyr" {
        eval_yr(value, 2020, 2030)
      } else if curr_field == "hgt" {
        eval_hgt(value)
      } else if curr_field == "hcl" {
        eval_hcl(value)
      } else if curr_field == "ecl" {
        eval_ecl(value)
      } else if curr_field == "pid" {
        eval_pid(value)
      } else { false };

      if !eval_res {
        res += 1;
        break;
      }
    }
  }

  // Function completes in O(n)
  return input.len() as u32 - res;
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
pub fn eval_yr(value: &str, start: u32, end: u32) -> bool {
  let value: u32 = match value.parse() {
    Ok(num) => num,
    Err(_) => return false,
  };

  return value >= start && value <= end;
}

// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
pub fn eval_hgt(value: &str) -> bool {
  let re = Regex::new(r"([0-9]+)([a-zA-Z]+)").unwrap();

  let Some(caps) = re.captures(value) else {
    println!("No match");
    return false;
  };

  let num: u32 = match (&caps[1]).parse() {
    Ok(num) => num,
    Err(_) => return false,
  };

  if !MEASUREMENTS.contains(&(&caps[2])) {
    return false;
  }

  let (start, end) = if &caps[2] == "cm" { (150, 193) } else { (59, 76) };

  num >= start && num <= end
}

pub fn eval_hcl(value: &str) -> bool {
  let re = Regex::new(r"#([0-9a-f]+)").unwrap();

  let Some(caps) = re.captures(value) else {
    println!("No match");
    return false;
  };

  (&caps[1]).len() == 6
}

pub fn eval_ecl(value: &str) -> bool {
  if value.len() != 3 {
    return false;
  }

  EYE_COLOR.contains(&value)
}

pub fn eval_pid(value: &str) -> bool {
  value.len() == 9
}
