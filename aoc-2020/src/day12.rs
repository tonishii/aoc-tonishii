use std::{collections::HashMap, fs::File, io::Read};

const DIRECTIONS: [char; 4] = ['E', 'S', 'W', 'N'];
const START_DIR: usize = 0;
const PART: &str = "part2";

pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  // O(n)
  let input: Vec<(char, u32)> = input_str
    .lines()
    .map(|s| {
      let (cmd, arg) = s.split_at(1);

      // println!("{} {}", cmd, arg);
      (cmd.chars().next().unwrap(), arg.parse().unwrap())
    })
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part2(input: Vec<(char, u32)>) -> u32 {
  let mut waypoint_pos: HashMap<char, u32> = HashMap::from([('E', 10), ('N', 1)]);
  let mut ship_pos: HashMap<char, u32> = HashMap::with_capacity(DIRECTIONS.len());

  for (cmd, arg) in input {
    match cmd {
      'F' => {
        for (&waypoint_dir, &waypoint_dist) in &waypoint_pos {
          ship_pos
            .entry(waypoint_dir)
            .and_modify(|e| *e += waypoint_dist * arg)
            .or_insert(waypoint_dist * arg);
        }
      },
      'N' | 'S' | 'E' | 'W' => {
        waypoint_pos
          .entry(cmd)
          .and_modify(|e| *e+=arg)
          .or_insert(arg);
      },
      'L' => {
        let east = *waypoint_pos.get(&'E').unwrap_or(&0);
        let south = *waypoint_pos.get(&'S').unwrap_or(&0);
        let west = *waypoint_pos.get(&'W').unwrap_or(&0);
        let north = *waypoint_pos.get(&'N').unwrap_or(&0);

        match arg % 360 {
          90  => {
            waypoint_pos.insert('E', north);
            waypoint_pos.insert('S', east);
            waypoint_pos.insert('W', south);
            waypoint_pos.insert('N', west);
          },
          180 => {
            waypoint_pos.insert('E', west);
            waypoint_pos.insert('S', north);
            waypoint_pos.insert('W', east);
            waypoint_pos.insert('N', south);
          },
          270 => {
            waypoint_pos.insert('E', south);
            waypoint_pos.insert('S', west);
            waypoint_pos.insert('W', north);
            waypoint_pos.insert('N', east);
          }
          _   => {
            println!("Error");
            break;
          },
        }
      }
      'R' => {
        let east = *waypoint_pos.get(&'E').unwrap_or(&0);
        let south = *waypoint_pos.get(&'S').unwrap_or(&0);
        let west = *waypoint_pos.get(&'W').unwrap_or(&0);
        let north = *waypoint_pos.get(&'N').unwrap_or(&0);

        match arg % 360 {
          90  => {
            waypoint_pos.insert('E', south);
            waypoint_pos.insert('S', west);
            waypoint_pos.insert('W', north);
            waypoint_pos.insert('N', east);
          },
          180 => {
            waypoint_pos.insert('E', west);
            waypoint_pos.insert('S', north);
            waypoint_pos.insert('W', east);
            waypoint_pos.insert('N', south);
          },
          270 => {
            waypoint_pos.insert('E', north);
            waypoint_pos.insert('S', east);
            waypoint_pos.insert('W', south);
            waypoint_pos.insert('N', west);
          }
          _   => {
            println!("Error");
            break;
          },
        }
      }
      _ => {
        println!("No match! Error in parsing.");
        break;
      }
    }
    println!("{:?} {:?} Command: {}{}", waypoint_pos, ship_pos, cmd, arg);
  }

  return m_dist(ship_pos);
}

pub fn part1(input: Vec<(char, u32)>) -> u32 {
  let mut ship_pos: HashMap<char, u32> = HashMap::with_capacity(DIRECTIONS.len());
  let mut curr_dir = START_DIR;

  // O(n)
  for (cmd, mut arg) in input {
    match cmd {
      'F' => {
        ship_pos
          .entry(DIRECTIONS[curr_dir])
          .and_modify(|e| *e+=arg)
          .or_insert(arg);
      },
      'N' | 'S' | 'E' | 'W' => {
        ship_pos
          .entry(cmd)
          .and_modify(|e| *e+=arg)
          .or_insert(arg);
      },
      'L' | 'R' => {
        arg %= 360;
        let units  = arg / 90;

        if cmd == 'L' {
          curr_dir = curr_dir + DIRECTIONS.len() - (units as usize % DIRECTIONS.len());
        } else {
          curr_dir += units as usize;
        }

        curr_dir %= DIRECTIONS.len();
      },
      _ => {
        println!("No match! Error in parsing.");
        break;
      }
    }

    // println!("{:?} Heading: {}, Command: {}{}", dist, dirs[curr_dir], cmd, arg);
  }

  // println!("{:?}", dist);

  // Function completes in O(n) time
  return m_dist(ship_pos);
}

fn m_dist(pos: HashMap<char, u32>) -> u32 {
  return pos.get(&'N').unwrap_or(&0)
          .abs_diff(*pos.get(&'S').unwrap_or(&0)) +
        pos.get(&'E').unwrap_or(&0)
          .abs_diff(*pos.get(&'W').unwrap_or(&0));
}