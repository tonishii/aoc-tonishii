use std::fs::File;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
  let day = "day4";
  let input =
    File::open(format!("./inputs/{day}.txt"))
    .expect("Path not found.");

  let res = match day {
    "day1" => day1::solve(input),
    "day2" => day2::solve(input),
    "day3" => day3::solve(input),
    "day4" => day4::solve(input),
    _ => panic!("Day does not exist!"),
  };

  println!("Result: {res}");
}
