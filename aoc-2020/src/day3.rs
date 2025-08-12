use std::{fs::File, io::Read};

const PART: &str = "part2";
const TREE: char = '#';
pub fn solve(mut input: File) -> u64 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<String> = input_str
    .lines()
    .map(|s| s.to_string())
    .collect();

  // Column length: 31
  return if PART == "part1" { part1(input, 31) } else { part2(input, 31) };
}

// This is only a brute force solution
fn brute_force(input: Vec<String>, col_len: usize, right: usize, down: usize) -> u64 {
  let (mut x, mut y) = (0, 0);
  let mut res: u64 = 0;

  while y < input.len() {
    if input[y].chars().nth(x) == Some(TREE) {
      res += 1;
    }

    x += right;
    x %= col_len; // Repeats pattern
    y += down;
  }

  // Function completes in O(n) where n is the number of lines
  return res;
}

fn part1(input: Vec<String>, col_len: usize) -> u64 {
  return brute_force(input, col_len, 3, 1);
}

fn part2(input: Vec<String>, col_len: usize) -> u64 {
  let mut res: u64 = 1;
  let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  for &(right, down) in &slopes {
    res *= brute_force(input.clone(), col_len, right, down) as u64;
  }

  return res;
}


