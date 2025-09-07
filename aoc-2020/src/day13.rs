use std::{collections::HashMap, fs::File, io::Read};

const PART: &str = "part2";

pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  // O(1)
  let (timestamp_start, bus_ids) = input_str
    .split_once('\n')
    .unwrap();

  // O(1)
  let timestamp_start = timestamp_start
    .parse::<u32>()
    .unwrap();

  // O(n)
  let bus_ids: Vec<Option<u32>> = bus_ids
    .split(',')
    .map(|s| if s == "x" { None } else { Some(s.parse().unwrap()) })
    .collect();

  // println!("{:?} {}", bus_ids, timestamp_start);

  return if PART == "part1" { part1((timestamp_start, bus_ids)) } else { part2((timestamp_start, bus_ids)) };
}

pub fn part2(_input: (u32, Vec<Option<u32>>)) -> u32 {
  return 0;
}

pub fn part1(input: (u32, Vec<Option<u32>>)) -> u32 {
  let timestamp_start = input.0;
  let bus_ids = input.1;

  // O(n)
  let mut res_bus_ids: HashMap<u32, f64> = HashMap::new();
  for bus_id in bus_ids {
    match bus_id {
      Some(value) => {res_bus_ids.insert(value, timestamp_start as f64/value as f64); },
      None => continue,
    }
  }

  // println!("{:?}", res_bus_ids);

  let (_, (&min_bus_id, &_)) = res_bus_ids
    .iter()
    .enumerate()
    .min_by(|&(_, (_, val1)), &(_, (_, val2))| val1.total_cmp(val2))
    .unwrap();

  let mut i: u32 = 1;
  let mut timestamp_end: u32;
  // println!("{} {}", value, timestamp_end);

  // O(1)
  loop  {
    timestamp_end = min_bus_id * i;
    i += 1;

    if timestamp_end > timestamp_start {
      break;
    }
  }

  // Function completes in O(n)
  return (timestamp_end - timestamp_start) * min_bus_id;
}