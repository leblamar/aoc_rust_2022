use std::fs::read_to_string;
use std::str::Lines;

pub fn main() {
  println!("It's day 3 !!!");
  let file = read_to_string("src/day3_input.txt")
    .unwrap();
  part1(file.lines());
  part2(file.lines());
}

fn to_value(string: char) -> i32 {
  let to_sub_low = 'a' as i32 - 1;
  let to_sub_up = 'A' as i32 - 27;

  string as i32 - if string.is_lowercase() {to_sub_low} else {to_sub_up}
}

fn cut_in_two(line: &str) -> (&str, &str) {
  let mid = line.len() / 2;
  let first = &line[0..mid];
  let second = &line[mid..line.len()];
  (first, second)
}

fn find_common_letter2((first, second): (&str, &str)) -> char {
  first.chars()
    .find(|&char| second.contains(char))
    .unwrap()
}

fn part1(parsed_lines: Lines) {
  let result: i32 = parsed_lines.into_iter()
    .map(cut_in_two)
    .map(find_common_letter2)
    .map(to_value)
    .sum();
  println!("Part 1 result : {}", result)
}

fn find_common_letter3(elves: &[&str]) -> char {
  elves[0].chars()
    .filter(|&char| elves[1].contains(char))
    .find(|&char| elves[2].contains(char))
    .unwrap()
}

fn part2(parsed_lines: Lines) {
  let result: i32 = parsed_lines.collect::<Vec<&str>>()
    .chunks(3)
    .map(find_common_letter3)
    .map(to_value)
    .sum();
  println!("Part 2 result : {}", result);
}
