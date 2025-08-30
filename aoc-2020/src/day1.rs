use std::{collections::HashSet, fs::File, io::Read};

const TARGET: u32 = 2020;
const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str: String = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read string");

  // Parse the lines: O(n)
  let input: Vec<u32> = input_str
    .lines()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

// fn old_part1(mut input: Vec<u32>) -> u32 {
//   input.sort(); // O(n * log(n))

//   let (mut x, mut y) = (0, input.len() - 1);

//   // O(n)
//   loop {
//     let curr = input[x] + input[y];
//     if curr == TARGET {
//       break;
//     } else if curr < TARGET {
//       x += 1;
//     } else {
//       y -= 1;
//     }
//   }

//   // Function completes in O(n * log(n))
//   println!("{} {}", input[x], input[y]);
//   return input[x] * input[y];
// }

fn part1(input: Vec<u32>) -> u32 {
  let mut set: HashSet<u32> = HashSet::new();

  for entry in input {
    // println!("{} {}", TARGET, entry);
    let comp: i32 = TARGET as i32 - entry as i32;

    if comp < 0 {
      continue;
    }

    if set.contains(&(comp as u32)) {
      // println!("{} {}", entry, set.get(&(comp as u32)).unwrap());
      return entry * set.get(&(comp as u32)).unwrap();
    }

    set.insert(entry);
  }

  return 0;
}

fn part2(mut input: Vec<u32>) -> u32 {
  // O(n log n)
  input.sort();

  // println!("{:?}", input);

  // O(n ^ 2)
  for i in 0..input.len() {
    let (mut low, mut high): (usize, usize) = (i + 1, input.len() - 1);

    // Basically just a two sum so O(n)
    while low < high {
      let sum = input[i] + input[low] + input[high];

      if sum == TARGET {
        return input[i] * input[low] * input[high];
      } else if sum > TARGET {
        high -= 1;
      } else {
        low += 1;

      }
    }
  }

  // Function completes in O(n ^ 2) time
  return 0;
}
