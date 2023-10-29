use std::fs::read_to_string;
use std::collections::HashSet;
use std::str::Lines;

pub fn main() {
  println!("It's day 3 !!!");
  let file = read_to_string("src/day3_input.txt").unwrap();
  part1(file.lines());
  part2(file.lines());
}

fn to_value(string: char) -> i32 {
  let to_sub_low = 'a' as i32 - 1;
  let to_sub_up = 'A' as i32 - 27;

  string as i32 - if string.is_lowercase() {to_sub_low} else {to_sub_up}
}

fn part1(parsed_lines: Lines) {
  let mut count = 0;
  for line in parsed_lines {
    let mid = line.len() / 2;
    let (first, second) = line.split_at(mid);
    let mut first_set = HashSet::new();
    first.chars().for_each(|char| {first_set.insert(char);});
    let same_letter = second.chars()
      .find(|char| first_set.contains(char))
      .unwrap();
    count += to_value(same_letter);
  }

  println!("Part 1 result : {}", count)
}

fn part2(parsed_lines: Lines) {
  let mut common_badge: HashSet<char> = HashSet::new();
  let mut second_common_badge: HashSet<char> = HashSet::new();

  let mut which_elf = 0;
  let mut count = 0;
  for line in parsed_lines {
    match which_elf {
      0 => {
        common_badge = HashSet::new();
        line.chars().for_each(|char| {common_badge.insert(char);});
      },
      1 => {
        second_common_badge = HashSet::new();
        line.chars()
          .filter(|char| common_badge.contains(char))
          .for_each(|char| {second_common_badge.insert(char);});
      },
      2 => {
        count += to_value(line.chars().find(|char| second_common_badge.contains(char)).unwrap());
      },
      _ => {}
    };

    which_elf = (which_elf + 1) % 3;
  }

  println!("Part 2 result : {}", count);
}
