use std::{fs::File, io::Read};

const PART: &str = "part2";
pub fn solve(mut input: File) -> i32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  // Parse the lines: O(n)
  let input: Vec<(&str, i32)> = input_str
    .lines()
    .map(|s| {
      let (op, arg) = s.split_once(" ").unwrap();

      (op, arg.parse().unwrap())
    })
    .collect();

  // println!("{:?}", input);
  return if PART == "part1" { part1(input).0 } else { part2(input) };
}

pub fn part1(input: Vec<(&str, i32)>) -> (i32, bool) {
  let (mut acc, mut i): (i32, i32) = (0, 0);
  let mut visited: Vec<usize> = Vec::new();

  // O(n)
  while i < input.len().try_into().unwrap() {
    let i_comp: usize = i.try_into().expect("Negative i.");

    if visited.contains(&i_comp) {
      return (acc, true);
    }

    let (op, arg) = input[i_comp];
    match op {
      "acc" => acc += arg,
      "jmp" => i += arg - 1,
      "nop" => (),
      _ => (),
    }

    visited.push(i_comp);
    // println!("{:?} i = {} acc = {}", visited, i, acc);
    i += 1;
  }

  // Function completes in O(n)
  return (acc, false);
}

pub fn part2(input: Vec<(&str, i32)>) -> i32 {
  let mut queue: Vec<usize> = Vec::new();

  // O(n)
  for (i, &(op, _)) in input.iter().enumerate() {
    if op == "jmp" || op == "nop" {
      queue.push(i);
    }
  }

  // println!("{:?}", queue);

  // O(n^2)
  for i in queue {
    let mut mod_input = input.clone();
    mod_input[i].0 = match mod_input[i].0 {
      "jmp" => "nop",
      "nop" => "jmp",
      _ => panic!("Error parsing."),
    };

    // O(n)
    let (acc, inf) = part1(mod_input);

    if !inf {
      // Function completes in O(n^2)
      return acc;
    }
  }

  return 0;
}