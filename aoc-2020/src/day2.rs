use std::{fs::File, io::Read};

const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<(u32, u32, char, String)> = input_str
    .lines()
    .map(|s| {
        let (policy, password) = s
          .split_once(": ")
          .expect("Invalid input format");

        let (range, letter) = policy
          .split_once(" ")
          .expect("Invalid input format");

        let (start, end) = range
          .split_once("-")
          .expect("Invalid input format");

        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();
        let letter = letter.chars().next().unwrap();

        (start, end, letter, password.to_string())
    })
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

fn part1(input: Vec<(u32, u32, char, String)>) -> u32 {
  // O(n * k) where k is the length of a password
  let mut res: u32 = 0;
  for (start, end, letter, password) in input {
    let ctr: u32 = password
      .chars()
      .filter(|&c| c == letter)
      .count() as u32;

    // start: letter must exist >= start in password
    // end: letter must exist <= end in password
    if ctr <= end && ctr >= start {
      res += 1;
    }
  }

  // Function completes in O(n * k) where k is the length of a password
  return res;
}

fn part2(input: Vec<(u32, u32, char, String)>) -> u32 {
  // O(n)
  let mut res: u32 = 0;
  for (start, end, letter, password) in input {
    let chars: Vec<char> = password.chars().collect();
    let start_letter = chars[(start - 1) as usize];
    let end_letter = chars[(end - 1) as usize];

    if (start_letter == letter || end_letter == letter) && start_letter != end_letter {
      res += 1;
    }
  }

  // Function completes in O(n)
  return res;
}
