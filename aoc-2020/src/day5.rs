use std::{cmp::{max, min}, fs::File, io::Read};

const PART: &str = "part2";
pub fn solve(mut input: File) -> u64 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<&str> = input_str
    .lines()
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part1(input: Vec<&str>) -> u64 {
  // O(n)
  let mut res: u64 = 0;
  for ticket in input {
    // This runs in constant time since we only have to parse a set amount of letters
    let row = bsearch(ticket, 'F', 'B', 127);
    let col = bsearch(&ticket[7..ticket.len()], 'L', 'R', 7);

    res = max(res, (row * 8 + col).into());
  }

  // Function completes in O(n)
  return res;
}

pub fn part2(input: Vec<&str>) -> u64 {
  let mut list: Vec<u64> = Vec::new();

  // O(n)
  for ticket in input {
    // This runs in constant time since we only have to parse a set amount of letters
    let row = bsearch(ticket, 'F', 'B', 127);
    let col = bsearch(&ticket[7..ticket.len()], 'L', 'R', 7);

    list.push((row * 8 + col).into());
  }

  // O(n log n)
  list.sort();

  // O(n)
  for (i, &id) in list.iter().enumerate() {
    if id + 1 == list[i + 1] {
      continue;
    }

    // println!("{} {} {}", list[max(0, i - 1)], id, list[i + 1]);
    return id + 1;
  }

  // Function completes in O(n log n)
  return 0;
}

pub fn bsearch(input: &str, lower: char, upper: char, bounds: u32) -> u32 {
  let (mut low, mut high): (u32, u32) = (0, bounds);

  for (i, next) in input.chars().enumerate() {
    let mid: u32 = (low + high) / 2;

    match next {
      x if x == lower => high = mid,
      y if y == upper => low = mid + 1,
      _ => {
        println!("No match");
        return 0;
      }
    }

    if i == 6 {
      break;
    }
  }

  // Function completes in O(log n) or O(log bounds) or O(1) in this case
  return min(low, high);
}
