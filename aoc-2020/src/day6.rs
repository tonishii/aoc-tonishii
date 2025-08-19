use std::{collections::HashMap, fs::File, io::Read};

const PART: &str = "part2";
pub fn solve(mut input: File) -> u64 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Same parsing as day 4

  // Parse the lines: O(n)
  let input: Vec<&str> = input_str
    .split("\n\n")
    .collect();

  // println!("{:?}", input);
  return if PART == "part1" { part1(input) } else { part2(input) };
}

fn part1(input: Vec<&str>) -> u64 {
  let input: Vec<String> = input
    .iter()
    .map(|s| s.replace('\n', ""))
    .collect();

  let mut res: u64 = 0;
  for group in input {
    res += count(group) as u64;
  }

  return res;
}

fn part2(input: Vec<&str>) -> u64 {
  let mut res: u64 = 0;

  for group in input {
    res += mod_count(group);
  }

  return res;
}

pub fn mod_count(input: &str) -> u64 {
  let mut qs: HashMap<char, u64> = HashMap::from([
    ('a', 0),
    ('b', 0),
    ('c', 0),
    ('d', 0),
    ('e', 0),
    ('f', 0),
    ('g', 0),
    ('h', 0),
    ('i', 0),
    ('j', 0),
    ('k', 0),
    ('l', 0),
    ('m', 0),
    ('n', 0),
    ('o', 0),
    ('p', 0),
    ('q', 0),
    ('r', 0),
    ('s', 0),
    ('t', 0),
    ('u', 0),
    ('v', 0),
    ('w', 0),
    ('x', 0),
    ('y', 0),
    ('z', 0)
  ]);

  for q in input.chars() {
    qs.entry(q).and_modify(|value| *value += 1);
  }

  let people_count: u64 = input.chars().filter(|c| *c == '\n').count() as u64 + 1;

  // println!("{:?}", qs);

  let mut count: u64 = 0;
  for &yes_count in qs.values() {
    // println!("{} {}", yes_count, people_count);
    if yes_count == people_count {
      count += 1;
    }
  }

  return count;
}

pub fn count(input: String) -> usize {
  let mut qs: HashMap<char, bool> = HashMap::from([
    ('a', false),
    ('b', false),
    ('c', false),
    ('d', false),
    ('e', false),
    ('f', false),
    ('g', false),
    ('h', false),
    ('i', false),
    ('j', false),
    ('k', false),
    ('l', false),
    ('m', false),
    ('n', false),
    ('o', false),
    ('p', false),
    ('q', false),
    ('r', false),
    ('s', false),
    ('t', false),
    ('u', false),
    ('v', false),
    ('w', false),
    ('x', false),
    ('y', false),
    ('z', false)
  ]);

  for q in input.chars() {
    qs.entry(q).and_modify(|value| *value = true);
  }

  return qs.values().filter(|&&b| b).count();
}
