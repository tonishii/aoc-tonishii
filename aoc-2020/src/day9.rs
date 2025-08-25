use std::{collections::VecDeque, fs::File, io::Read};

const MIN_SET_LEN: u8 = 2;
const PREAMBLE: u8 = 25;
const PART: &str = "part2";
pub fn solve(mut input: File) -> u64 {
  let mut input_str = String::new();

  input.read_to_string(&mut input_str)
    .unwrap();

  let input: Vec<u64> = input_str
    .lines()
    .map(|s| s.parse().expect("Error in parsing."))
    .collect();

  return if PART == "part1" { part1(&input).0 } else { part2(input) };
}

pub fn part1(input: &Vec<u64>) -> (u64, usize) {
  for (i, &num) in input.iter().skip(PREAMBLE as usize).enumerate() {
    // println!("{} {}", i, num);
    let i = i + PREAMBLE as usize;
    let mod_input: Vec<u64> = input[i - PREAMBLE as usize..i].to_vec();
    if !two_sum(mod_input, num) {
      return (num, i);
    }
  }

  return (0, 0);
}

pub fn part2(input: Vec<u64>) -> u64 {
  // O(n log n)
  let (target, ind) = part1(&input);
  // println!("{}", target);

  let mut sum: u64 = 0;
  let mut start: usize = 0;
  let mut state: VecDeque<u64> = VecDeque::new();

  // O(n) SLIDING WINDOW APPROACH
  for end in 0..ind {
    state.push_back(input[end]);
    sum += input[end];

    // Keep adding until up to length
    if state.len() < MIN_SET_LEN as usize {
      continue;
    }

    // println!("Before: {:?} {}", state, sum);

    while sum > target && start <= end {
      let head = state.pop_front().expect("Q empty");
      sum -= head;
      start += 1;

      if sum == target {
        break;
      }
    }

    // println!("After: {:?} {}", state, sum);

    if sum == target {
      break;
    }
  }

  // Function completes in O(n log n), O(n) if two_sum is optimized
  let iter = state.iter();
  return iter.clone().min().unwrap() + iter.max().unwrap();
}

// Not the best implementation
pub fn two_sum(mut input: Vec<u64>, target: u64) -> bool {
  // println!("{:?} {}", input, input.len());
  // O(n log n)
  input.sort();

  let (mut low, mut high): (usize, usize) = (0, input.len() - 1);

  // println!("{:?}", input);

  // O(n)
  while low < high {
    let sum = input[low] + input[high];
    // println!("{}", sum);

    if sum == target {
      return true;
    } else if sum > target {
      high -= 1;
    } else {
      low += 1;
    }
  }

  // O(n log n)
  return false;
}
