use std::{collections::HashMap, fs::File, io::Read};

const END: usize = 2020;
const END_2: usize = 30000000;
const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  let input: Vec<u32> = input_str
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();

  // println!("{:?}", input);

  return new(input);
}

pub fn new(input: Vec<u32>) -> u32 {
  let mut turn_map: HashMap<u32, (usize, usize)> = HashMap::new();
  let mut last_turn: u32 = 0;

  // Initialize: O(n)
  for (turn_ctr, turn) in input.iter().enumerate() {
    turn_map
      .entry(*turn)
      .and_modify(|v| (*v)=(0, turn_ctr + 1))
      .or_insert((0, turn_ctr + 1));

    last_turn = *turn;
  }

  let end = if PART == "part1" { END } else if PART == "part2" { END_2 } else { panic!("Invalid part.") };
  let mut turn_ctr = input.len();

  while turn_ctr < end {
    turn_ctr += 1;
    // println!("Turn {}: Last Turn: {} {:?}", turn_ctr, last_turn, turn_map.get(&last_turn));
    match turn_map.get(&last_turn) {
      Some(&value) => {
        let age = match value.0 {
          0 => 0,
          _ => (value.1 - value.0) as u32,
        };

        turn_map
          .entry(age)
          .and_modify(|v| *v=(v.1, turn_ctr))
          .or_insert((0, turn_ctr));
        last_turn = age;
      },
      None => {
        turn_map
          .entry(0)
          .and_modify(|v| *v=(0_usize, turn_ctr));

        last_turn = 0;
      },
    }

    // println!("Turn {}: {:?} {}", turn_ctr, turn_map, last_turn);
  }

  // println!("{:?} {}", turn_map, last_turn);
  return last_turn;
}

pub fn _old(input: Vec<u32>) -> u32 {
  let mut turn_map: HashMap<u32, Vec<usize>> = HashMap::new();
  let mut last_turn: u32 = 0;

  // Initialize: O(n)
  for (turn_ctr, turn) in input.iter().enumerate() {
    turn_map
      .entry(*turn)
      .and_modify(|v| (*v).push(turn_ctr))
      .or_insert(vec![turn_ctr]);

    last_turn = *turn;
  }

  let end = if PART == "part1" { END } else if PART == "part2" { END_2 } else { panic!("Invalid part.") };
  let mut turn_ctr = input.len() - 1;
  while turn_ctr < end - 1 {
    let last_turn_spoken = turn_map
      .get(&last_turn);

    // println!("{} {} {:?}", turn_ctr, last_turn, last_turn_spoken);
    match last_turn_spoken {
      Some(value) => {
        let value_last_ind = value.len() - 1;

        let age = match value_last_ind.checked_sub(1) {
          Some(res_value) => (value[value_last_ind] - value[res_value]) as u32,
          None => 0,
        };

        turn_map
          .entry(age)
          .and_modify(|v| (*v).push(turn_ctr + 1))
          .or_insert(vec![turn_ctr + 1]);
        last_turn = age;
      },
      None => {
        turn_map
          .entry(0)
          .and_modify(|v| (*v).push(turn_ctr + 1));

        last_turn = 0;
      },
    }

    turn_ctr += 1;
    // println!("Turn {}: {:?} {}", turn_ctr + 1, turn_map, last_turn);
  }

  // println!("{:?} {}", turn_map, last_turn);
  return last_turn;
}