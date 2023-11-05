use std::fs::read_to_string;

pub fn main() {
  println!("It's day 10 !!!");

  let instruction_list = read_instructions();

  part1(&instruction_list);
  part2(&instruction_list);
}

#[derive(Debug)]
enum Instruction {
  Noop,
  Addx(i32)
}

fn read_instructions() -> Vec<Instruction> {
  read_to_string("src/day10_input.txt")
    .unwrap()
    .lines()
    .map(|line| line.split(" ").collect::<Vec<&str>>())
    .map(line_to_instruction)
    .collect()
}

fn line_to_instruction(line: Vec<&str>) -> Instruction {
  match *line.get(0).unwrap() {
    "noop" => Instruction::Noop,
    "addx" => Instruction::Addx(line.get(1).unwrap().parse().unwrap()),
    err => panic!("This instruction does not exist : {}", err)
  }
}

impl Instruction {
  fn apply_instruction(&self) -> i32 {
    match self {
      Instruction::Noop => 0,
      Instruction::Addx(value) => value.clone()
    }
  }

  fn transform_for_flat(&self) -> Vec<Instruction> {
    match self {
      Instruction::Noop => vec![Instruction::Noop],
      Instruction::Addx(v) => vec![Instruction::Noop, Instruction::Addx(*v)]
    }
  }
}

fn part1(instruction_list: &Vec<Instruction>) {
  let important_idxs = vec![20, 60, 100, 140, 180, 220];
  let mut register_x = 1;
  let mut result = 0;
  for (idx, instruction) in instruction_list.iter()
    .flat_map(Instruction::transform_for_flat)
    .enumerate() {
      if important_idxs.contains(&(idx + 1)) {
        result += register_x * (1 + idx as i32);
      }
      register_x += instruction.apply_instruction();
  }

  println!("Part 1 result : {}", result)
}

fn part2(instruction_list: &Vec<Instruction>) {
  let important_idxs = vec![40, 80, 120, 160, 200, 240];
  let mut register_x = 1;
  let mut result = "".to_string();
  let mut current_line = "".to_string();
  let mut cur_line = 0;
  for (idx, instruction) in instruction_list.iter()
    .flat_map(Instruction::transform_for_flat)
    .enumerate() {
      if register_x <= idx as i32 + 1 - cur_line * 40 
        && idx as i32 + 1 - cur_line * 40 <= register_x + 2 {
        current_line.push_str("#");
      } else {
        current_line.push_str(".");
      }

      if important_idxs.contains(&(idx + 1)) {
        result.push_str("\n");
        result.push_str(&current_line);
        current_line = "".to_string();
        cur_line += 1;
      }
      register_x += instruction.apply_instruction();
  }

  //println!("Part 2 result : \n{}", result);
  println!("Part 2 result : {}", "BZPAJELK")
}
