use std::fs::File;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
  let day = "day3";
  let input =
    File::open(format!("./inputs/{day}.txt"))
    .expect("Path not found.");

  enum ResultType {
    U32(u32),
    U64(u64),
  }

  let res  = match day {
    "day1" => ResultType::U32(day1::solve(input)),
    "day2" => ResultType::U32(day2::solve(input)),
    "day3" => ResultType::U64(day3::solve(input)),
    "day4" => ResultType::U32(day4::solve(input)),
    _ => panic!("Day does not exist!"),
  };

  match res {
    ResultType::U32(v) => println!("Result: {}", v),
    ResultType::U64(v) => println!("Result: {}", v),
}
}
