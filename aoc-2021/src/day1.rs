use std::{fs::File, io::Read};

const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read the string.");

  let input: Vec<u32> = input_str
    .lines()
    .map(|v| v.parse().unwrap())
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part1(input: Vec<u32>) -> u32 {
  let mut res: u32 = 0;
  let mut prev: u32 = 0;
  for (i, &depths) in input.iter().enumerate() {
    if i != 0 {
      if depths > prev {
        res += 1;
      }
    }

    prev = depths;
  }

  return res;
}

pub fn part2(input: Vec<u32>) -> u32 {
  let mut res: u32 = 0;
  let mut prev: u32 = 0;

  for i in 0..(input.len() - 2) {
    let mut curr: u32 = 0;
    for j in i..i+3 {
      // print!("{} ", input[j]);
      curr += input[j];
    }
    // println!();

    if i != 0 {
      if curr > prev {
        res += 1;
      }
    }

    prev = curr;
  }

  return res;
}