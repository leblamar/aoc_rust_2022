use std::fs::read_to_string;
use std::collections::HashSet;

pub fn main() {
  println!("It's day 6 !!!");
  let file = read_to_string("src/day6_input.txt").unwrap();
  let line = file.lines().nth(0).unwrap();

  part1(line);
  part2(line);
}

fn first_solver(line: &str, k: usize) -> usize {
  let mut first_value: usize = 0;
  for i in 0..line.len() {
    let mut set: HashSet<char> = HashSet::new();

    for j in 0..k {
      let value = line.chars().nth(i + j).unwrap();
      set.insert(value);
    }

    if set.len() == k {
      first_value = i + k;
      break;
    }
  }

  first_value
}

fn part1(line: &str) {
  let first_value = first_solver(line, 4);
  println!("Part 1 result : {}", first_value);
}

fn part2(line: &str) {
  let first_value = first_solver(line, 14);
  println!("Part 2 result : {}", first_value);
}
