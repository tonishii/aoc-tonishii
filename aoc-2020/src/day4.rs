use std::{fs::File, io::Read};

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
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

  return part2(input);
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

      if curr_field == "byr" {
        eval_year(value, 1920, 2002);
      } else if curr_field == "iyr" {
        eval_year(value, 2010, 2020);
      } else if curr_field == "eyr" {
        eval_year(value, 2020, 2030);
      } else if curr_field == "hgt" {

      } else if curr_field == "hcl" {

      } else if curr_field == "ecl" {

      } else if curr_field == "pid" {

      }
    }
  }

  // Function completes in O(n)
  return input.len() as u32 - res;
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn eval_year(value: &str, start: u32, end: u32) -> bool {
  let value: u32 = match value.parse() {
    Ok(num) => num,
    Err(_) => return false,
  };

  return value >= start && value <= end;
}
