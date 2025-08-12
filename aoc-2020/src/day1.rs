use std::{fs::File, io::Read};

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

fn part1(mut input: Vec<u32>) -> u32 {
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

fn part2(mut input: Vec<u32>) -> u32 {
  input.sort(); // O(n * log(n))

  println!("{:?}", input);
  let (mut x, mut y) = (0, input.len() - 1);

  loop {
    let curr = input[x] + input[y];

    if x >= y {
      return 0;
    }

    println!("{}", curr);
    if curr < 2020 {
      println!("LESS THAN {} {} {}", x, y, curr);

      // Iterate through 0 up until y to search for the smaller number
      for i in 0..y {
        let curr = input[x] + input[i] + input[y];
        println!("{} {} {} {}", x, i, y, curr);

        if i == x {
          continue;
        } else if curr == 2020 {
          return input[x] * input[i] * input[y];
        } else if curr > 2020 {
          break;
        }
      }

      x += 1;
    } else {
      y -= 1;
    }
  }
}
