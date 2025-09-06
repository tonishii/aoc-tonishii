#[allow(dead_code)]
use core::fmt;
use std::fs::File;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

enum ResultType {
  U32(u32),
  U64(u64),
  I32(i32)
}

impl fmt::Display for ResultType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ResultType::U32(v) => write!(f, "{v}"),
      ResultType::U64(v) => write!(f, "{v}"),
      ResultType::I32(v) => write!(f, "{v}"),
    }
  }
}

fn main() {
  let day = "day14";
  let input =
    File::open(format!("./inputs/{day}.txt"))
    .expect("Path not found.");

  // let input =
  //   File::open(format!("./tests/{day}.txt"))
  //   .expect("Path not found.");

  let res  = match day {
    "day1" => ResultType::U32(day1::solve(input)),
    "day2" => ResultType::U32(day2::solve(input)),
    "day3" => ResultType::U64(day3::solve(input)),
    "day4" => ResultType::U32(day4::solve(input)),
    "day5" => ResultType::U64(day5::solve(input)),
    "day6" => ResultType::U64(day6::solve(input)),
    "day7" => ResultType::U32(day7::solve(input)),
    "day8" => ResultType::I32(day8::solve(input)),
    "day9" => ResultType::U64(day9::solve(input)),
    "day10" => ResultType::U32(day10::solve(input)),
    "day11" => ResultType::U32(day11::solve(input)),
    "day12" => ResultType::U32(day12::solve(input)),
    "day13" => ResultType::U32(day13::solve(input)),
    "day14" => ResultType::U64(day14::solve(input)),
    _ => panic!("Day does not exist!"),
  };

  println!("Result: {}", res);
}
