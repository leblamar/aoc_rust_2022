use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum RockPaperScissor {
  Rock,
  Paper,
  Scissor,
  Unknown
}

pub fn main() {
  println!("It's day 2 !!!");
  let parsed_list = parse_file();
  part1(&parsed_list);
  part2(parsed_list);
}

fn parse_rockpaperscisor(shape: &str) -> RockPaperScissor {
  match shape {
    "A" | "X" => RockPaperScissor::Rock,
    "B" | "Y" => RockPaperScissor::Paper,
    "C" | "Z" => RockPaperScissor::Scissor,
    _ => RockPaperScissor::Unknown
  }
}

fn parse_file() -> Vec<Vec<RockPaperScissor>> {
  let file = read_to_string("src/day2_input.txt").unwrap();

  file.lines()
    .map(|line|
      line.split_ascii_whitespace()
        .map(parse_rockpaperscisor)
        .collect()
    ).collect()
}

fn part1(parsed_list: &Vec<Vec<RockPaperScissor>>) {
  let result: i32 = parsed_list.into_iter()
    .map(|shapes| point_for_shape(shapes.get(1).unwrap()) 
      + point_for_duel(*shapes.get(0).unwrap(), *shapes.get(1).unwrap())
    ).sum();

  println!("Part 1 result : {}", result)
}

fn part2(parsed_list: Vec<Vec<RockPaperScissor>>) {
  let mut count = 0;
  for shapes in parsed_list {
    let enemy_shape = shapes.get(0).unwrap();
    let my_shape = shapes.get(1).unwrap();
    let points_for_part2 = point_for_part2(*enemy_shape, *my_shape);
    count += points_for_part2;
  }

  println!("Part 2 result : {}", count)
}

fn point_for_shape(my_shape: &RockPaperScissor) -> i32 {
  match my_shape {
    RockPaperScissor::Rock => 1,
    RockPaperScissor::Paper => 2,
    RockPaperScissor::Scissor => 3,
    _ => 0
  }
}

fn point_for_duel(enemy_shape: RockPaperScissor, my_shape: RockPaperScissor) -> i32 {
  match enemy_shape {
    RockPaperScissor::Rock => if let RockPaperScissor::Paper = my_shape { 6 } else 
      if let RockPaperScissor::Rock = my_shape { 3 } else { 0 },
    RockPaperScissor::Paper => if let RockPaperScissor::Scissor = my_shape { 6 } else 
      if let RockPaperScissor::Paper = my_shape { 3 } else { 0 },
    RockPaperScissor::Scissor => if let RockPaperScissor::Rock = my_shape { 6 } else 
      if let RockPaperScissor::Scissor = my_shape { 3 } else { 0 },
    _ => 0
  }
}

fn point_for_part2(enemy_shape: RockPaperScissor, my_shape: RockPaperScissor) -> i32 {
  match my_shape {
    RockPaperScissor::Rock => if let RockPaperScissor::Paper = enemy_shape { 1 + 0 } else 
      if let RockPaperScissor::Rock = enemy_shape { 3 + 0 } else { 2 + 0 },
    RockPaperScissor::Paper => if let RockPaperScissor::Scissor = enemy_shape { 3 + 3 } else 
      if let RockPaperScissor::Paper = enemy_shape { 2 + 3 } else { 1 + 3 },
    RockPaperScissor::Scissor => if let RockPaperScissor::Rock = enemy_shape { 2 + 6 } else 
      if let RockPaperScissor::Scissor = enemy_shape { 1 + 6 } else { 3 + 6 },
    _ => 0
  }
}
