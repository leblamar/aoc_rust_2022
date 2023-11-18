use std::fs::read_to_string;

pub fn main() {
  println!("It's day 11 !!!");

  let monkey_list = read_monkeys();

  part1(&monkey_list);
  part2(&monkey_list);
}

#[derive(Debug, Clone)]
struct Monkey {
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
      items: parse_items(chunk[1]), 
      operation: Operation::create_operation_from_line(chunk[2]), 
      div_test_nb: parse_div_test_nb(chunk[3]), 
      monkey_on_true_id: parse_monkey_condition_id(chunk[4]), 
      monkey_on_false_id: parse_monkey_condition_id(chunk[5]),
      inspect_item_counter: 0
    }
  }

  fn get_monkey_and_next_monkeys(monkey_slice: &mut [Monkey], idx: usize) -> (&mut Monkey, &mut Monkey, &mut Monkey) {
    let true_monkey_id = monkey_slice[idx].monkey_on_true_id;
    let false_monkey_id = monkey_slice[idx].monkey_on_false_id;
    let (first_monkeys, last_monkeys) = monkey_slice.split_at_mut(idx);
    if true_monkey_id < idx && idx < false_monkey_id {
      let (cur_monkey, last_last_monkeys) = last_monkeys.split_at_mut(1);
      return (&mut cur_monkey[0], &mut first_monkeys[true_monkey_id], &mut last_last_monkeys[false_monkey_id - idx - 1]);
    } else if false_monkey_id < idx && idx < true_monkey_id {
      let (cur_monkey, last_last_monkeys) = last_monkeys.split_at_mut(1);
      return (&mut cur_monkey[0], &mut last_last_monkeys[true_monkey_id - idx + 1], &mut first_monkeys[false_monkey_id]);
    } else if true_monkey_id < idx && false_monkey_id < idx {
      if true_monkey_id < false_monkey_id {
        let (true_slice, false_slice) = first_monkeys.split_at_mut(false_monkey_id);
        return (&mut last_monkeys[0], &mut true_slice[true_monkey_id], &mut false_slice[0]);
      } else if true_monkey_id > false_monkey_id {
        let (false_slice, true_slice) = first_monkeys.split_at_mut(true_monkey_id);
        return (&mut last_monkeys[0], &mut true_slice[0], &mut false_slice[false_monkey_id]);
      } else {
        panic!("On devrait pas arriver ici, ça veut dire que true == false: idx: {}, true: {}, false: {}", idx, true_monkey_id, false_monkey_id)
      }
    } else if idx < true_monkey_id && idx < false_monkey_id {
      if true_monkey_id < false_monkey_id {
        let (last_first_slice, last_last_slice) = last_monkeys.split_at_mut(false_monkey_id - idx);
        let (cur_monkey, true_monkey) = last_first_slice.split_at_mut(true_monkey_id - idx);
        return (&mut cur_monkey[0], &mut true_monkey[0], &mut last_last_slice[0]);
      } else if true_monkey_id > false_monkey_id {
        let (last_first_slice, last_last_slice) = last_monkeys.split_at_mut(true_monkey_id - idx);
        let (cur_monkey, false_monkey) = last_first_slice.split_at_mut(false_monkey_id - idx);
        return (&mut cur_monkey[0], &mut last_last_slice[0], &mut false_monkey[0]);
      } else {
        panic!("On devrait pas arriver ici, ça veut dire que true == false: idx: {}, true: {}, false: {}", idx, true_monkey_id, false_monkey_id)
      }
    } else {
      panic!("On devrait pas arriver ici !!! Ça veut dire qu'un true ou false monkey vaut idx !!! idx: {}, true: {}, false: {}", idx, true_monkey_id, false_monkey_id);
    }
  }

  fn play_round(monkey_list: &mut Vec<Monkey>, reduce_worry: &impl Fn(u64) -> u64) {
    let length = monkey_list.len();
    let monkey_slice = &mut monkey_list[..];
    for idx in 0..length {
      let (cur_monkey, true_monkey, false_monkey) = Monkey::get_monkey_and_next_monkeys(monkey_slice, idx);
      cur_monkey.play_turn(true_monkey, false_monkey, reduce_worry);
    }
  }

  fn play_turn(&mut self, true_monkey: &mut Monkey, false_monkey: &mut Monkey, reduce_worry: &impl Fn(u64) -> u64) {
    let length = self.items.len();
    for &item in &self.items {
      let new_worry_level= reduce_worry(self.operation.apply(item));
      if new_worry_level % self.div_test_nb == 0 {
        true_monkey.items.push(new_worry_level);
      } else {
        false_monkey.items.push(new_worry_level);
      }
    }

    self.inspect_item_counter += length;
    self.items.clear();
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
      .nth(1)
      .unwrap()
      .parse()
      .unwrap()
  }

  fn create_operation_from_line(line: &str) -> Operation {
    let filtered_line = line.split("=")
      .nth(1)
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

fn parse_items(line: &str) -> Vec<u64> {
  line.split(":")
    .nth(1)
    .unwrap()
    .replace(" ", "")
    .split(",")
    .map(|item| item.parse().unwrap())
    .collect()
}

fn parse_div_test_nb(line: &str) -> u64 {
  line.replace(" ", "")
    .split("by")
    .nth(1)
    .unwrap()
    .parse()
    .unwrap()
}

fn parse_monkey_condition_id(line: &str) -> usize {
  line.replace(" ", "")
    .split("monkey")
    .nth(1)
    .unwrap()
    .parse()
    .unwrap()
}

fn part1(monkey_list: &Vec<Monkey>) {
  let reduce_worry = |worry_level: u64| worry_level / 3;
  let mut new_monkey_list = monkey_list.clone();
  for _ in 0..20 {
    Monkey::play_round(&mut new_monkey_list, &reduce_worry);
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
  let max_value: u64 = monkey_list.iter()
      .map(|monkey| monkey.div_test_nb)
      .product();
  let reduce_worry = |worry_level: u64| worry_level % max_value;
  let mut new_monkey_list = monkey_list.clone();
  for _ in 0..10_000 {
    Monkey::play_round(&mut new_monkey_list, &reduce_worry);
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
