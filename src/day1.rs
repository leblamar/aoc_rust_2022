use std::fs::read_to_string;

pub fn main() {
  println!("It's day 1 !!!");
  let parsed_list = parse_file_to_elves();

  part1(&parsed_list);
  part2(&parsed_list);
}

fn parse_file_to_elves() -> Vec<Vec<i32>> {
  let file = read_to_string("src/day1_input.txt").unwrap();
  let mut parsed_list: Vec<Vec<i32>> = Vec::new();
  
  let first_elf: Vec<i32> = Vec::new();
  parsed_list.push(first_elf);

  for line in file.lines() {
    if line.is_empty() {
      let new_elf: Vec<i32> = Vec::new();
      parsed_list.push(new_elf);
    } else {
      let last_index = parsed_list.len() - 1;
      let parsed_line: i32 = line.parse().unwrap();
      parsed_list[last_index].push(parsed_line);
    }
  }

  parsed_list
}

fn part1(parsed_list: &Vec<Vec<i32>>) {
  let result = parsed_list.iter()
    .map(|elf| elf.iter().sum::<i32>())
    .max()
    .unwrap();

  println!("Part 1 result : {}", result)
}

fn part2(parsed_list: &Vec<Vec<i32>>) {
  let elves: Vec<i32> = parsed_list.iter()
    .map(|elf| elf.iter().sum::<i32>())
    .collect();

  let mut top1_elf = 0;
  let mut top2_elf = 0;
  let mut top3_elf = 0;

  for elf in elves {
    if elf > top1_elf {
      top3_elf = top2_elf;
      top2_elf = top1_elf;
      top1_elf = elf;
    } else if elf > top2_elf {
      top3_elf = top2_elf;
      top2_elf = elf;
    } else if elf > top3_elf {
      top3_elf = elf;
    }
  }

  let result = top1_elf + top2_elf + top3_elf;

  println!("Part 2 result : {}", result)
}
