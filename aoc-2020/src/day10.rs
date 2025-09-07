use std::{collections::HashMap, fs::File, io::Read};

const START: u32 = 0;
const PART: &str = "part1";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();

  input.read_to_string(&mut input_str)
    .unwrap();

  // Parse the lines: O(n)
  let input: Vec<u32> = input_str
    .lines()
    .map(|s| s.parse().expect("Error in parsing."))
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part2(_input: Vec<u32>) -> u32 {
  return 0;
}

pub fn part1(input: Vec<u32>) -> u32 {
  let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
  let (mut jolt_1, mut jolt_3): (u32, u32) = (0, 0);

  let mut extended = input.clone();
  extended.push(0);

  // O(n)
  for &joltage in &extended {
    map.insert(joltage, Vec::new());
  }


  // O(n)
  for &joltage in &extended {
    // O(1)
    let mut valids = Vec::new();

    for valid in joltage+1..joltage+3+1 {
      if map.contains_key(&valid) {
        valids.push(valid);
      }
    }

    map
      .entry(joltage)
      .and_modify(|prev| *prev=valids);
  }

  // println!("{:?}", map);

  // O(n)
  let mut curr = START;
  loop {
    let curr_val = map.get(&curr).unwrap();

    // println!("{} {:?} {} {}", curr, *curr_val, jolt_1, jolt_3);

    let next = match curr_val.first() {
      Some(&val) => val,
      None => {
        break;
      }
    };

    let dist = next - curr;

    if dist == 1 {
      jolt_1 += 1;
    } else if dist == 3 {
      jolt_3 += 1;
    }

    curr = next;
  }

  // Function completes in O(n) time
  return jolt_1 * (jolt_3 + 1);
}

