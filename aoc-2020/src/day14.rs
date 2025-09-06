use std::{collections::HashMap, fs::File, io::Read};
use regex::Regex;

const PART: &str = "part2";
pub fn solve(mut input: File) -> u64 {
  let mut input_str = String::new();

  input
    .read_to_string(&mut input_str)
    .unwrap();

  // Parse the lines: O(n)
  let input: Vec<(Option<String>, String)> = input_str
    .lines()
    .map(|s| {
      let (key, value) = s
        .split_once(" = ")
        .unwrap();

      let re = Regex::new(r"mem\[([0-9]+)\]").unwrap();
      match key {
        "mask" => (None, value.to_string()),
        _ => {
          let Some(caps) = re.captures(key) else {
            panic!("No match for {s}");
          };

          (Some((&caps[1]).to_string()), value.to_string())
        },
      }
    })
    .collect();

  // println!("{:?}", input);
  return if PART == "part1" { part1(input) } else { part2(&input) };
}

pub fn part2(input: &Vec<(Option<String>, String)>) -> u64 {
  let mut res_map: HashMap<u64, u64> = HashMap::new();
  let mut curr_bitmask = String::new();

  // O(n * m)
  for (key, value) in input {
    match key {
      Some(key_val) => {
        let key_val: u32 = key_val
          .parse()
          .unwrap();

        // O(m)
        let res_bin = format!("{:0>width$}", int_to_binary(key_val), width = curr_bitmask.len());

        // O(m)
        let (mut res_addr, floating_list, n) = mod_mask(&res_bin, &curr_bitmask);

        // O(2^m), worst case bits in bitmask are all floating
        for i in 0..2_i32.pow(n as u32) {
          let floating_bits = format!("{:0>width$}", int_to_binary(i as u32), width = n);

          for (floating_bit, floating_ind) in floating_bits.chars().zip(floating_list.clone()) {
            res_addr[floating_ind] = floating_bit;
          }

          res_map
            .insert(
            u64::from_str_radix(&String::from_iter(res_addr.clone()), 2).unwrap(),
            value
                .parse()
                .unwrap()
            );

          // println!("{} {}", floating_bits, String::from_iter(res_addr.clone()),);
        }

      },
      None => curr_bitmask = value.to_string(),
    }

    // println!("{:?}", res_map);
  }

  // Function completes in O(2^m) if bitmask can consist of all floating bits
  // or just O(n * m) in average
  return res_map.iter().map(|(_, &v)| v).sum();
}

pub fn mod_mask(input: &String, mask: &String) -> (Vec<char>, Vec<usize>, usize) {
  let mut res = String::new();
  let mut floating_list: Vec<usize> = Vec::new();

  for (i, (addr_char, mask_char)) in input.chars().zip(mask.chars()).enumerate() {
    match mask_char {
      'X' => {
        floating_list.push(i);
        res.push('X');
      },
      '1' => res.push('1'),
      '0' => res.push(addr_char),
      _ => panic!("No match address masking."),
    }
  }

  // Function completes in O(m), m: length of the bitmask
  return (
    res
      .chars()
      .collect(),
    floating_list.clone(),
    floating_list.len()
  );
}

pub fn mask(input: &String, mask: &String) -> String {
  let mut res = String::new();
  for (input_char, mask_char) in input.chars().zip(mask.chars()) {
    match mask_char {
      'X' => res.push(input_char),
      _ => res.push(mask_char),
    }
  }

  // println!("{} {} {}", input, mask, res);

  // Function completes in O(m), m: length of the bitmask
  return res;
}

pub fn int_to_binary(mut input: u32) -> String {
  if input == 0 {
    return "0".to_string();
  }

  let mut binary_string = String::new();
  while input > 0 {
    let remainder = input % 2;
    binary_string
      .insert(0, char::from_digit(remainder, 10)
      .unwrap());
    input /= 2;
  }

  // Function completes in O(m), m: magnitude of input
  binary_string
}

pub fn part1(input: Vec<(Option<String>, String)>) -> u64 {
  let mut res_map: HashMap<u32, u64> = HashMap::new();
  let mut curr_bitmask: String = String::new();

  // O(n * m), n: no. of strings, m: length of curr_bitmask
  for (key, value) in input {
    match key {
      Some(key_val) => {
        let value = value
          .parse()
          .unwrap();

        // O(m)
        let res_bin = format!("{:0>width$}", int_to_binary(value), width = curr_bitmask.len());

        // O(m)
        let res_bin = mask(&res_bin, &curr_bitmask);
        res_map
          .insert(
            key_val
              .parse()
              .unwrap(),
          u64::from_str_radix(&res_bin, 2)
              .unwrap()
          );

      },
      None => curr_bitmask = value,
    }
  }

  // println!("{:?}", res_map);

  // O(n * m), n: no. of strings, m: length of curr_bitmask
  return res_map.iter().map(|(_, &v)| v).sum();
}