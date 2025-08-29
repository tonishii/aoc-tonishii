use std::{fs::File, io::Read};

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const PART: &str = "part2";
pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  // Parse the lines: O(n)
  let input: Vec<Vec<char>> = input_str
    .lines()
    .map(|s| s.chars().collect())
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

// Literally the same as part 1 just change the condition and the get_adj function
pub fn part2(input: Vec<Vec<char>>) -> u32 {
  let mut res_input: Vec<Vec<char>> = input.clone();
  let (mut res, mut change_ctr): (u32, u32) = (0, 0);

  let (row, col): (usize, usize) = (input.len(), input[0].len());

  loop {
    let mut next = res_input.clone();

    // O(n * m) n: no. of rows, m: no. of cols
    for (i, rows) in res_input.iter().enumerate() {
      for (j, &seat) in rows.iter().enumerate() {
        match seat {
          FLOOR => (),
          EMPTY => {
            let occ_ctr = search_adj_occ(&res_input, (i, j), row, col);

            if occ_ctr <= 0 {
              next[i][j] = OCCUPIED;
              res += 1;
              change_ctr += 1;
            }
          },
          OCCUPIED => {
            let occ_ctr = search_adj_occ(&res_input, (i, j), row, col);

            if occ_ctr >= 5 {
              next[i][j] = EMPTY;
              change_ctr += 1;
              res -= 1; // Minus to remain neutral
            }

            res += 1;
          },
          _ => {
            panic!("No match. Error in parsing.");
          }
        }
      }
    }

    res_input = next;
    // print_2d(&res_input);

    if change_ctr == 0 {
      break;
    }

    change_ctr = 0;
    res = 0;
  }

  return res;
}

pub fn part1(input: Vec<Vec<char>>) -> u32 {
  let mut res_input: Vec<Vec<char>> = input.clone();
  let mut res = 0;

  let mut change_ctr: u32 = 0;
  loop {
    let mut next = res_input.clone();

    // O(n * m) n: no. of rows, m: no. of cols
    for (i, row) in res_input.clone().iter().enumerate() {
      for (j, &seat) in row.iter().enumerate() {
        match seat {
          FLOOR => (),
          EMPTY => {
            let occ_ctr = get_adj_occ(&res_input, (i, j));

            if occ_ctr <= 0 {
              next[i][j] = OCCUPIED;
              res += 1;
              change_ctr += 1;
            }
          },
          OCCUPIED => {
            let occ_ctr = get_adj_occ(&res_input, (i, j));

            if occ_ctr >= 4 {
              next[i][j] = EMPTY;
              change_ctr += 1;
              res -= 1; // Minus to remain neutral
            }

            res += 1;
          },
          _ => {
            panic!("No match. Error in parsing.");
          }
        }
      }
    }
    res_input = next;
    // print_2d(&res_input);

    if change_ctr == 0 {
      break;
    }

    change_ctr = 0;
    res = 0;
  }

  return res;
}

pub fn print_2d(input: &Vec<Vec<char>>) {
  input.iter().for_each(|row| {
    for &char in row {
      print!("{char}");
    }
    println!();
  });
  println!();
}

pub fn search_adj(input: &Vec<Vec<char>>, pos: (usize, usize), row: usize, col: usize) -> Vec<Option<char>> {
  let adjacent_pos: [(i32, i32); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0),         (1, 0),
    (-1, -1), (0, -1), (1, -1),
  ];

  let mut res: Vec<Option<char>> = Vec::with_capacity(8);
  for (x, y) in adjacent_pos {
    let (mut new_x, mut new_y) = (pos.0 as i32 + x, pos.1 as i32 + y);

    while new_x >= 0 && new_y >= 0 && new_x < row as i32 && new_y < col as i32 {
      let curr_pos: char = input[new_x as usize][new_y as usize];

      if curr_pos != FLOOR {
        res.push(Some(curr_pos));
        break;
      }

      new_x += x;
      new_y += y;
    }

    if res.len() < res.capacity() {
      res.push(None);
    }
  }

  return res;
}

pub fn search_adj_occ(input: &Vec<Vec<char>>, pos: (usize, usize), row: usize, col: usize) -> u32 {
  let adjacents = search_adj(&input, pos, row, col);

  let mut occ_ctr: u32 = 0;
  for adj in adjacents {
    match adj {
      Some(v) => {
        if v == OCCUPIED {
          occ_ctr += 1;
        }
      },
      None => (),
    }
  }

  return occ_ctr;
}

pub fn get_adj(input: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<Option<char>> {
  let adjacent_pos: [(i32, i32); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0),         (1, 0),
    (-1, -1), (0, -1), (1, -1),
  ];

  // Function completes in O(1) time
  adjacent_pos
    .iter()
    .map(|(x, y)| {
    let (new_x, new_y) = (pos.0 as i32 + x, pos.1 as i32 + y);

    if new_x < 0 || new_y < 0 {
      return None;
    }

    input
      .get(new_x as usize)
      .and_then(|row| row.get(new_y as usize))
      .copied()
  })
  .collect()
}

pub fn get_adj_occ(input: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
  let adjacents = get_adj(&input, pos);

  let mut occ_ctr: u32 = 0;
  for adj in adjacents {
    match adj {
      Some(v) => {
        if v == OCCUPIED {
          occ_ctr += 1;
        }
      },
      None => (),
    }
  }

  return occ_ctr;
}