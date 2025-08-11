use std::{fs::File, io::Read};

const TREE: char = '#';
pub fn solve(mut input: File) -> u32 {
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
  return brute_force(input, 31);
}

// This is only a brute force solution
fn brute_force(input: Vec<String>, col_len: usize) -> u32 {
  let (mut x, mut y) = (0, 0);
  let mut res: u32 = 0;

  while y < input.len() {
    if input[y].chars().nth(x) == Some(TREE) {
      res += 1;
    }

    x += 3;
    x %= col_len; // Repeats pattern
    y += 1;
  }

  // Function completes in O(n) where n is the number of lines
  return res;
}
