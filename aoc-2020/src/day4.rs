use std::{collections::HashSet, fs::File, io::Read};
use regex::Regex;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const MEASUREMENTS: [&str; 2] = ["cm", "in"];
const EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const PART: &str = "part2";

pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<String> = input_str
    .split("\n\n")
    .map(|s| {
      let s = s.to_string();
      s.replace('\n', " ")
    })
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

fn part2(input: Vec<String>) -> u32 {
  // O(n * 7) which is just O(n)

  let mut res: u32 = 0;
  for passport in &input {
    // println!("{passport}");

    let mut invalid = false;
    let mut completed_fields: HashSet<&str> = HashSet::with_capacity(8);
    for field in passport.split(" ") {
      // println!("{field}");
      let (curr_field, value) = field
        .split_once(':')
        .unwrap();

      let eval_res = match curr_field {
        "byr" => eval_yr(value, 1920, 2002),
        "iyr" => eval_yr(value, 2010, 2020),
        "eyr" => eval_yr(value, 2020, 2030),
        "hgt" => eval_hgt(value),
        "hcl" => eval_hcl(value),
        "ecl" => eval_ecl(value),
        "pid" => eval_pid(value),
        "cid" => continue,
        _ => false
      };

      // println!("{curr_field} {value} {eval_res}");

      if !eval_res || !completed_fields.insert(curr_field) {
        invalid = true;
        break;
      }
    }

    if invalid {
      res += 1;
      continue;
    }

    if !(completed_fields.len() == 7 || completed_fields.len() == 8) {
      res += 1;
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
    // println!("{value}");
    println!("No match!");
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
    // println!("{value}");
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
  let re = Regex::new(r"([0-9]+)").unwrap();

  let Some(caps) = re.captures(value) else {
    // println!("{value}");
    println!("No match");
    return false;
  };

  (&caps[1]).len() == 9
}
