use std::fs::File;

mod day1;

#[allow(dead_code)]
fn main() {
  let day = "day1";
  let input =
    File::open(format!("./inputs/{day}.txt"))
    .expect("Path not found.");

  enum ResultType {
    U32(u32),
    U64(u64),
  }

  let res  = match day {
    "day1" => ResultType::U32(day1::solve(input)),
    _ => panic!("Day does not exist!"),
  };

  match res {
    ResultType::U32(v) => println!("Result: {}", v),
    ResultType::U64(v) => println!("Result: {}", v),
  }
}
