use std::{fs::File, io::Read};

pub fn solve(mut input: File) -> u32 {
  let mut input_str: String = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let mut input: Vec<u32> = input_str
    .lines()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  input.sort(); // O(n * log(n))

  let (mut x, mut y) = (0, input.len() - 1);

  // O(n)
  loop {
    let curr = input[x] + input[y];
    if curr == 2020 {
      break;
    } else if curr < 2020 {
      x += 1;
    } else {
      y -= 1;
    }
  }

  // Function completes in O(n * log(n))
  return input[x] * input[y];
}
