use std::fs::read_to_string;

pub fn main() {
  println!("It's day 11 !!!");

  let monkey_list = read_monkeys();

  part1(&monkey_list);
  part2(&monkey_list);
}

#[derive(Debug, Clone)]
struct Monkey {
  id: usize,
  items: Vec<u64>,
  operation: Operation,
  div_test_nb: u64,
  monkey_on_true_id: usize,
  monkey_on_false_id: usize,
  inspect_item_counter: usize
}

impl Monkey {
  fn create_monkey_from_chunk(chunk: &[&str]) -> Monkey {
    Monkey { 
      id: parse_id(chunk[0]), 
      items: parse_items(chunk[1]), 
      operation: Operation::create_operation_from_line(chunk[2]), 
      div_test_nb: parse_div_test_nb(chunk[3]), 
      monkey_on_true_id: parse_monkey_condition_id(chunk[4]), 
      monkey_on_false_id: parse_monkey_condition_id(chunk[5]),
      inspect_item_counter: 0
    }
  }

  fn play_round(monkey_list: &mut Vec<Monkey>, with_div_by_three: bool, max_value: u64) {
    let length = monkey_list.len();
    
    for idx in 0..length {
      let monkey = monkey_list.get(idx).unwrap().clone();
      monkey.play_turn(monkey_list, with_div_by_three, max_value);
    }
  }

  fn play_turn(&self, monkey_list: &mut Vec<Monkey>, with_div_by_three: bool, max_value: u64) {
    let true_items: &mut Vec<u64> = &mut Vec::new();
    let false_items: &mut Vec<u64> = &mut Vec::new();
    for item in &self.items {
      let new_worry_level_without_div = self.operation.apply(*item);
      let new_worry_level = if with_div_by_three { new_worry_level_without_div / 3 } 
        else { new_worry_level_without_div % max_value };
      if new_worry_level % self.div_test_nb == 0 {
        true_items.push(new_worry_level);
      } else {
        false_items.push(new_worry_level);
      }
    }

    for monkey in monkey_list {
      if monkey.id == self.id {
        monkey.inspect_item_counter += self.items.len();
        monkey.items.clear();
      } else if monkey.id == self.monkey_on_true_id {
        monkey.items.append(true_items);
      } else if monkey.id == self.monkey_on_false_id {
        monkey.items.append(false_items);
      }
    }
  }
}

#[derive(Debug, Clone)]
enum Operation {
  Add(u64),
  Mult(u64),
  Square
}

impl Operation {
  fn parse_value(formula: &str, symbol: &str) -> u64 {
    formula.split(symbol)
      .collect::<Vec<&str>>()
      .get(1)
      .unwrap()
      .parse()
      .unwrap()
  }

  fn create_operation_from_line(line: &str) -> Operation {
    let filtered_line = line.split("=")
      .collect::<Vec<&str>>()
      .get(1)
      .unwrap()
      .replace(" ", "");

    match filtered_line.as_str() {
      "old*old" => Operation::Square,
      else_line => if else_line.contains("+") {
        Operation::Add(Operation::parse_value(else_line, "+"))
      } else {
        Operation::Mult(Operation::parse_value(else_line, "*"))
      }
    }
  }

  fn apply(&self, old: u64) -> u64 {
    match self {
      Operation::Add(value) => old + value,
      Operation::Mult(value) => old * value,
      Operation::Square => old * old
    }
  }
}

fn read_monkeys() -> Vec<Monkey> {
  read_to_string("src/day11_input.txt")
    .unwrap()
    .lines()
    .collect::<Vec<&str>>()
    .chunks(7)
    .map(Monkey::create_monkey_from_chunk)
    .collect()
}

fn parse_id(line: &str) -> usize {
  let almost_id = *line.split(" ")
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap();
  almost_id.split_at(almost_id.len() - 1)
    .0
    .parse()
    .unwrap()
}

fn parse_items(line: &str) -> Vec<u64> {
  line.split(":")
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap()
    .replace(" ", "")
    .split(",")
    .map(|item| item.parse().unwrap())
    .collect()
}

fn parse_div_test_nb(line: &str) -> u64 {
  line.replace(" ", "")
    .split("by")
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap()
    .parse()
    .unwrap()
}

fn parse_monkey_condition_id(line: &str) -> usize {
  line.replace(" ", "")
    .split("monkey")
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap()
    .parse()
    .unwrap()
}

fn part1(monkey_list: &Vec<Monkey>) {
  let max_value: u64 = monkey_list.clone()
      .iter()
      .map(|monkey| monkey.div_test_nb)
      .product();
  let mut new_monkey_list = monkey_list.clone();
  for _ in 0..20 {
    Monkey::play_round(&mut new_monkey_list, true, max_value);
  }
  
  let mut activest: usize = 0;
  let mut second_activest: usize = 0;
  for monkey in new_monkey_list {
    if monkey.inspect_item_counter > activest {
      second_activest = activest;
      activest = monkey.inspect_item_counter;
    } else if monkey.inspect_item_counter > second_activest {
      second_activest = monkey.inspect_item_counter;
    }
  }

  let result = activest * second_activest;
  println!("Part 1 result : {}", result)
}

fn part2(monkey_list: &Vec<Monkey>) {
  let max_value: u64 = monkey_list.clone()
      .iter()
      .map(|monkey| monkey.div_test_nb)
      .product();
  let mut new_monkey_list = monkey_list.clone();
  for _ in 0..10_000 {
    Monkey::play_round(&mut new_monkey_list, false, max_value);
  }
  
  let mut activest: usize = 0;
  let mut second_activest: usize = 0;
  for monkey in new_monkey_list {
    if monkey.inspect_item_counter > activest {
      second_activest = activest;
      activest = monkey.inspect_item_counter;
    } else if monkey.inspect_item_counter > second_activest {
      second_activest = monkey.inspect_item_counter;
    }
  }

  let result = activest * second_activest;  
  println!("Part 2 result : {}", result);
}
