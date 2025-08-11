use std::{fs::File, io::Read};

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
