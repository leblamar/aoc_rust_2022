use std::cmp::Ordering;
use std::fs::read_to_string;
use std::boxed::Box;

pub fn main() {
  println!("It's day 13 !!!");

  let packet_pairs = read_and_parse_file();

  part1(&packet_pairs);
  part2(&packet_pairs);
}

fn read_and_parse_file() -> Vec<PacketPair> {
    read_to_string("src/day13_input.txt")
      .unwrap()
      .lines()
      .collect::<Vec<&str>>()
      .chunks(3)
      .map(PacketPair::parse_packet_pair)
      .collect()
}

#[derive(Debug, Clone)]
enum Value {
  List(Box<Vec<Value>>),
  Int(u32)
}

impl Value {
  fn parse_value(line: &str) -> Value {
    if line == "[]" {
      return Value::List(Box::new(Vec::new()));
    }
    let try_parse = line.parse::<u32>();
    if try_parse.is_ok() {
      return Value::Int(try_parse.unwrap());
    }
    let mut line_to_parse = &line[1..(line.len()-1)];
    let mut values: Vec<Value> = Vec::new();
    while !line_to_parse.is_empty() {
      if line_to_parse.starts_with("[") {
        let mut count = 0;
        let mut split_idx = 0;
        for (idx, c) in line_to_parse.char_indices() {
          count += if c == '[' { 1 } else if c == ']' { -1 } else { 0 };
          if count == 0 {
            split_idx = idx;
            break;
          }
        }
        let first = &line_to_parse[0..=split_idx];
        values.push(Value::parse_value(first));
        line_to_parse = &line_to_parse[(split_idx+1)..(line_to_parse.len())];
        if line_to_parse.starts_with(',') {
          line_to_parse = &line_to_parse[1..(line_to_parse.len())];
        }
      } else {
        match line_to_parse.split_once(",") {
          Some((first, second)) => {
            values.push(Value::parse_value(first));
            line_to_parse = second;
          },
          None => {
            values.push(Value::parse_value(line_to_parse));
            line_to_parse = "";
          }
        };
      }
    }
    Value::List(Box::new(values))
  }

  fn encapsulate(&self) -> Value {
    match self {
      Value::List(_) => panic!("N'est pas censé arrivé"),
      Value::Int(value) => Value::List(Box::new(vec![Value::Int(*value)]))
    }
  }

  fn is_equal_to(&self, other: &Value) -> Ordering {
    match (self, other) {
      (Value::Int(left), Value::Int(right)) => left.cmp(right),
      (Value::List(left_box), Value::List(right_box)) => {
        let values_in_order = left_box.iter()
          .zip(right_box.iter())
          .map(|(l_val, r_val)| l_val.is_equal_to(r_val))
          .reduce(|acc, next| match acc {
            Ordering::Equal => next,
            order => order
          })
          .unwrap_or(Ordering::Equal);

        match values_in_order {
          Ordering::Equal => left_box.len().cmp(&right_box.len()),
          order => order
        }
      },
      (Value::Int(_), Value::List(_)) 
        => {self.encapsulate().is_equal_to(other)},
      (Value::List(_), Value::Int(_)) 
        => {self.is_equal_to(&other.encapsulate())}
    }
  }
}

#[derive(Debug, Clone)]
struct PacketPair {
  left: Value,
  right: Value
}

impl PacketPair {
  fn parse_packet_pair(lines: &[&str]) -> PacketPair {
    PacketPair {
      left: Value::parse_value(lines[0]),
      right: Value::parse_value(lines[1])
    }
  }

  fn is_valid(&self) -> bool {
    match self.left.is_equal_to(&self.right) {
      Ordering::Greater => false,
      _ => true
    }
  }
}

fn part1(packet_pairs: &Vec<PacketPair>) {
  let result: usize = packet_pairs.iter()
    .enumerate()
    .filter(|(_, packet_pair)| packet_pair.is_valid())
    .map(|(idx, _)| idx + 1)
    .sum();
  println!("Part 1 result : {}", result)
}

fn part2(packet_pairs: &Vec<PacketPair>) {
  let mut res = packet_pairs.iter()
    .flat_map(|packet_pairs| vec![packet_pairs.left.clone(), packet_pairs.right.clone()])
    .collect::<Vec<Value>>();
  res.push(Value::List(Box::new(vec![Value::Int(2)])));
  res.push(Value::List(Box::new(vec![Value::Int(6)])));
  res.sort_by(|left, right| left.is_equal_to(right));

  let idx_2 = res.iter()
    .position(|value| match value {
      Value::List(val) => if let Some(first) = val.get(0) { 
        matches!(first, Value::Int(v_i) if *v_i == 2)
      } else { false },
      _ => false
    })
    .map(|val| val + 1)
    .unwrap();
  let idx_6 = res.iter()
    .position(|value| match value {
      Value::List(val) => if let Some(first) = val.get(0) { 
        matches!(first, Value::Int(v_i) if *v_i == 6)
      } else { false },
      _ => false
    })
    .map(|val| val + 1)
    .unwrap();

  let result = idx_2 * idx_6;
  println!("Part 2 result : {}", result);
}
