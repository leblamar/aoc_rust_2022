use std::fs::read_to_string;
use std::fmt;

pub fn main() {
  println!("It's day 5 !!!");

  let mut env_with_inst = parse_file();
  part1(&mut env_with_inst);

  let mut env_with_inst_part2 = parse_file();
  part2(&mut env_with_inst_part2);
}

#[derive(Debug, Clone)]
struct Instruction {
  qty: usize,
  src: usize,
  dst: usize
}

impl Instruction {
  fn create_inst_from_line(line: &str) -> Instruction {
    let split: Vec<&str> = line.split(' ').collect();
    Instruction { 
      qty: split[1].parse::<usize>().unwrap(), 
      src: split[3].parse::<usize>().unwrap() - 1, 
      dst: split[5].parse::<usize>().unwrap() - 1
    }
  }
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(qty: {}, src: {}, dst: {})", self.qty, self.src, self.dst)
  }
}

#[derive(Debug)]
struct Env {
  columns: Vec<Vec<char>>
}

impl Env {
  fn create_env_from_lines(lines: Vec<&str>) -> Env {
    let mut columns: Vec<Vec<char>> = Vec::new();
    let columns_len = lines[0].len() / 4;
    (0..=columns_len).for_each(|_| columns.push(Vec::new()));

    for line in lines {
      for position in (0..line.len()).filter(|pos| pos % 4 == 1) {
        let column = &mut columns[position / 4];
        let char = line.chars().nth(position).unwrap();
        if !char.is_whitespace() {
          column.insert(0, char);
        }
      }      
    }

    Env { columns }
  }

  fn apply(&mut self, inst: Instruction) {
    let src_column = &mut self.columns[inst.src];
    let mut tmp_column: Vec<char> = Vec::new();

    for _ in 0..inst.qty {
      tmp_column.insert(0, src_column.pop().unwrap());
    }

    let dst_column = &mut self.columns[inst.dst];
    for _ in 0..inst.qty {
      dst_column.push(tmp_column.pop().unwrap());
    }
  }

  fn apply_part2(&mut self, inst: Instruction) {
    let src_column = self.columns.get_mut(inst.src).unwrap();
    let mut tmp_column: Vec<char> = Vec::new();

    for _ in 0..inst.qty {
      tmp_column.push(src_column.pop().unwrap());
    }

    let dst_column = self.columns.get_mut(inst.dst).unwrap();
    for _ in 0..inst.qty {
      dst_column.push(tmp_column.pop().unwrap());
    }
  }
}

struct EnvWithInst {
  env: Env,
  insts: Vec<Instruction>
}

impl EnvWithInst {
  fn resolve(&mut self) {
    self.insts
      .clone()
      .into_iter()
      .for_each(|inst| self.env.apply(inst))
  }

  fn resolve_part2(&mut self) {
    self.insts
      .clone()
      .into_iter()
      .for_each(|inst| self.env.apply_part2(inst))
  }

  fn get_result(&self) -> String {
    self.env
      .columns
      .clone()
      .into_iter()
      .map(|column| column[column.len() - 1].clone())
      .collect()
  }
}

fn parse_file() -> EnvWithInst {
  let file = read_to_string("src/day5_input.txt").unwrap();

  let env_part: Env = Env::create_env_from_lines(file.lines()
    .take_while(|line| !line.contains('1'))
    .collect()
  );

  let instr_part: Vec<Instruction> = file.lines()
    .skip_while(|line| !line.is_empty())
    .skip(1)
    .map(|string| Instruction::create_inst_from_line(string))
    .collect();

  EnvWithInst {
    env: env_part,
    insts: instr_part
  }
}

fn part1(env_with_inst: &mut EnvWithInst) {
  env_with_inst.resolve();
  println!("Part 1 result : {}", env_with_inst.get_result());
}

fn part2(env_with_inst: &mut EnvWithInst) {
  env_with_inst.resolve_part2();
  println!("Part 2 result : {}", env_with_inst.get_result());
}
