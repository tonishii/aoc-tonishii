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

  let (mut o2_list, mut co2_list): (Vec<Vec<char>>, Vec<Vec<char>>) = (input.clone(), input.clone());

  // O(1)
  for i in 0..BIN_LEN {
    // Total of O(4n) which is just O(n)
    if o2_list.len() > 1 {
      // O(n)
      let o2_ctr = count_bits(&o2_list, i);

      // O(n)
      if o2_ctr >= 0 {
        o2_list = o2_list
          .drain(..)
          .filter(|o2| {
            o2[i] == '1'
          })
          .collect();
      } else {
        o2_list = o2_list
          .drain(..)
          .filter(|o2| {
            o2[i] == '0'
          })
          .collect();
      }
    }

    if co2_list.len() > 1 {
      // O(n)
      let co2_ctr = count_bits(&co2_list, i);

      // O(n)
      if co2_ctr >= 0 {
        co2_list = co2_list
          .drain(..)
          .filter(|co2| {
            co2[i] == '0'
          })
          .collect();
      } else {
        co2_list = co2_list
          .drain(..)
          .filter(|co2| {
            co2[i] == '1'
          })
          .collect();
      }
    }
  }

  let (o2, co2): (String, String) = (
    o2_list.last().unwrap().into_iter().collect(),
    co2_list.last().unwrap().into_iter().collect()
  );

  // println!("O2: {:?}\n Co2: {:?}", o2_list.iter().collect(), co2_list);

  let (o2, co2): (u32, u32) = (
    u32::from_str_radix(&o2, 2).unwrap(),
    u32::from_str_radix(&co2, 2).unwrap()
  );

  // Function completes in O(n)
  return o2 * co2;
}

pub fn count_bits(input: &Vec<Vec<char>>, pos: usize) -> i32 {
  // O(n)
  let mut ctr: i32 = 0;
  for bin_num in input {
    ctr += match bin_num[pos] {
      '1' => 1,
      '0' => -1,
      _ => {
        println!("No match!");
        break;
      }
    };
  }

  // Function completes in O(n)
  ctr
}
