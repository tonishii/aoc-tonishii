use std::{fs::File, io::Read};

const PART: &str = "part1";
pub fn solve(mut input: File) -> i32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read the string.");

  // Parsing each line: O(n)
  let input: Vec<(&str, i32)> = input_str
    .lines()
    .map(|s| {
      let (command, unit) = s
        .split_once(" ")
        .unwrap();

      let unit: i32 = unit.parse().unwrap();

      (command, unit)
    })
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part1(input: Vec<(&str, i32)>) -> i32 {
  // O(n)
  let (mut pos, mut depth): (i32, i32) = (0, 0);
  for (command, unit) in input {
    match command {
      "forward" => pos += unit,
      "up" => depth -= unit,
      "down" => depth += unit,
      _ => {
        println!("No match!");
        break;
      }
    }

    // println!("{} {} {} {}", command, unit, pos, depth);
  }

  // Function completes in O(n)
  return pos * depth;
}

pub fn part2(input: Vec<(&str, i32)>) -> i32 {
  // O(3n) which is just O(n)
  let mut res: i32 = 0;
  // let mut prev: u32 = 0;
  // for i in 0..(input.len() - 2) {
  //   let mut curr: u32 = 0;
  //   for j in i..i+3 {
  //     // print!("{} ", input[j]);
  //     curr += input[j];
  //   }
  //   // println!();

  //   if i != 0 {
  //     if curr > prev {
  //       res += 1;
  //     }
  //   }

  //   prev = curr;
  // }

  // Function completes in O(n)
  return res;
}