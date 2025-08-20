use std::{fs::File, io::Read};

const BIN_LEN: usize = 12;
const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .expect("Failed to read the string.");

  // Parsing each line: O(n)
  let input: Vec<&str> = input_str
    .lines()
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part1(input: Vec<&str>) -> u32 {
  let mut list: [i32; BIN_LEN] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  // O(n)
  for bin_num in input {
    // O(m) where m (aka BIN_LEN) is the length of each binary number
    for (i, curr_char) in bin_num.chars().enumerate() {
      list[i] += match curr_char {
        '1' => 1,
        '0' => -1,
        _ => {
          println!("No match!");
          break;
        }
      }
    }
  }

  // println!("{:?}", list);

  // O(BIN_LEN) which is just O(1) since BIN_LEN is a constant
  let mut gamma: String = String::new();
  for dig in list {
    let gamma_bit: char;
    if dig > 0 {
      gamma_bit = '1';
    } else if dig < 0 {
      gamma_bit = '0';
    } else {
      println!("ERROR! No least or most bit.");
      return 99999;
    }

    gamma.push(gamma_bit);
  }

  // println!("{}", gamma);

  // Since epsilon is just a one's complement of gamma we can just invert the bits to get epsilon
  // O(1)
  let mut epsilon: String = String::new();
  for dig in gamma.chars() {
    epsilon.push(match dig {
      '1' => '0',
      '0' => '1',
      _ => {
        println!("No match!");
        break;
      }
    });
  };

  let (gamma, epsilon): (u32, u32) = (
    u32::from_str_radix(&gamma, 2).unwrap(),
    u32::from_str_radix(&epsilon, 2).unwrap()
  );

  return gamma * epsilon;
}

pub fn part2(input: Vec<&str>) -> u32 {
  // Better to build a 2D vector for this problem

  let input: Vec<Vec<char>> = input
    .iter()
    .map(|&bin_num| {
      bin_num.chars().collect()
    }).collect();

  let (mut o2, mut co2): (String, String) = (String::new(), String::new());
  let (mut o2_flag, mut co2_flag) = (true, true);

  // O(1)
  for i in 0..BIN_LEN {
    // O(n)
    let mut bin_num: String = String::new();
    let (mut o2_ctr, mut co2_ctr): (i32, i32) = (0, 0);
    let (mut o2_rec_ctr, mut co2_rec_ctr) = (0, 0);

    for j in 0..input.len() {
      // print!("{} ", input[j][i]); By column access
      bin_num = input[j].iter().collect();

      if bin_num.starts_with(&o2) {
        print!("{} ", bin_num);
        o2_rec_ctr += 1;
        o2_ctr += match input[j][i] {
          '1' => 1,
          '0' => -1,
          _ => {
            println!("No match!");
            break;
          }
        }
      }

      if bin_num.starts_with(&co2) {
        println!("{}", bin_num);
        co2_rec_ctr += 1;
        co2_ctr += match input[j][i] {
          '1' => 1,
          '0' => -1,
          _ => {
            println!("No match!");
            break;
          }
        }
      }
    }

    // println!("{} {} {} {}", o2_ctr, co2_ctr, o2_flag, co2_flag);

    if o2_flag {
      if o2_rec_ctr == 1 {
        o2 = bin_num.clone();
        o2_flag = false;
      } else if o2_ctr > 0 {
        o2.push('1');
      } else if o2_ctr < 0 {
        o2.push('0');
      } else {
        o2.push('1');
      }
    }

    if co2_flag {
      if co2_rec_ctr == 1  {
        co2 = bin_num.clone();
        co2_flag = false;
      } else if co2_ctr > 0 {
        co2.push('0');
      } else if co2_ctr < 0 {
        co2.push('1');
      } else {
        co2.push('0');
      }
    }

    // println!("{} {}", o2, co2);
  }

  // println!("{} {}", o2, co2);
  let (o2, co2): (u32, u32) = (u32::from_str_radix(&o2, 2).unwrap(), u32::from_str_radix(&co2, 2).unwrap());
  return o2 * co2;
}