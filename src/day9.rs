use std::cmp::Ordering;
use std::fs::read_to_string;
//use std::collections::{BinaryHeap, HashSet};
use std::collections::BinaryHeap;

pub fn main() {
  println!("It's day 9 !!!");

  let moves = get_moves();
  
  part1(&moves);
  part2(&moves);
}

#[derive(Clone, Debug)]
enum Move {
  Up,
  Down,
  Left,
  Right,
  DiagUL,
  DiagUR,
  DiagDL,
  DiagDR,
  None
}

fn get_moves() -> Vec<Move> {
  read_to_string("src/day9_input.txt")
    .unwrap()
    .lines()
    .map(|line| line.split(" ").collect::<Vec<&str>>())
    .flat_map(line_to_move_list)
    .collect()
}

fn line_to_move_list(line_list: Vec<&str>) -> Vec<Move> {
  let line_move = match line_list[0] {
    "U" => Move::Up,
    "D" => Move::Down,
    "L" => Move::Left,
    "R" => Move::Right,
    x => panic!("This letter is unrecognized {}", x)
  };

  vec![line_move; line_list[1].parse().unwrap()]
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn move_from(&mut self, move_elt: Move) {
    match move_elt {
      Move::Up => self.y += 1,
      Move::Down => self.y -= 1,
      Move::Left => self.x -= 1,
      Move::Right => self.x += 1,
      Move::DiagUL => { self.y += 1; self.x -= 1 },
      Move::DiagUR => { self.y += 1; self.x += 1 },
      Move::DiagDL => { self.y -= 1; self.x -= 1 },
      Move::DiagDR => { self.y -= 1; self.x += 1 },
      Move::None => {}
    };
  }

  fn compute_other_move(&self, other: &Point, move_elt: &Move) -> Move {
    match move_elt {
      Move::Up => {
        if self.y <= other.y {
          return Move::None;
        } else if self.x < other.x {
          return Move::DiagUL;
        } else if self.x == other.x {
          return Move::Up;
        } else if self.x > other.x {
          return Move::DiagUR;
        } else {
          panic!("This cannot happend !")
        }
      },
      Move::Down => {
        if self.y >= other.y {
          return Move::None;
        } else if self.x < other.x {
          return Move::DiagDL;
        } else if self.x == other.x {
          return Move::Down;
        } else if self.x > other.x {
          return Move::DiagDR;
        } else {
          panic!("This cannot happend !")
        }
      },
      Move::Left => {
        if self.x >= other.x {
          return Move::None;
        } else if self.y < other.y {
          return Move::DiagDL;
        } else if self.y == other.y {
          return Move::Left;
        } else if self.y > other.y {
          return Move::DiagUL;
        } else {
          panic!("This cannot happend !")
        }
      },
      Move::Right => {
        if self.x <= other.x {
          return Move::None;
        } else if self.y < other.y {
          return Move::DiagDR;
        } else if self.y == other.y {
          return Move::Right;
        } else if self.y > other.y {
          return Move::DiagUR;
        } else {
          panic!("This cannot happend !")
        }
      },
      Move::DiagUL => {
        if self.x >= other.x && self.y <= other.y {
          return Move::None;
        } else if self.x < other.x {
          if self.y < other.y {
            return Move::Left;
          } else {
            return Move::DiagUL;
          }
        } else if self.y > other.y {
          if self.x > other.x {
            return Move::Up;
          } else {
            return Move::DiagUL;
          }
        } else {
          panic!("This cannot happened !");
        }
      },
      Move::DiagUR => {
        if self.x <= other.x && self.y <= other.y {
          return Move::None;
        } else if self.x > other.x {
          if self.y < other.y {
            return Move::Right;
          } else {
            return Move::DiagUR;
          }
        } else if self.y > other.y {
          if self.x < other.x {
            return Move::Up;
          } else {
            return Move::DiagUR;
          }
        } else {
          panic!("This cannot happened !");
        }
      },
      Move::DiagDL => {
        if self.x >= other.x && self.y >= other.y {
          return Move::None;
        } else if self.x < other.x {
          if self.y > other.y {
            return Move::Left;
          } else {
            return Move::DiagDL;
          }
        } else if self.y < other.y {
          if self.x > other.x {
            return Move::Down;
          } else {
            return Move::DiagDL;
          }
        } else {
          panic!("This cannot happened !");
        }
      },
      Move::DiagDR => {
        if self.x <= other.x && self.y >= other.y {
          return Move::None;
        } else if self.x > other.x {
          if self.y > other.y {
            return Move::Right;
          } else {
            return Move::DiagDR;
          }
        } else if self.y < other.y {
          if self.x < other.x {
            return Move::Down;
          } else {
            return Move::DiagDR;
          }
        } else {
          panic!("This cannot happened !");
        }
      },
      Move::None => Move::None
    }
  }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      match self.x.cmp(&other.x) {
        Ordering::Equal => self.y.cmp(&other.y),
        ord => ord
      }
    }
}

fn compute_tail_moves(moves: &Vec<Move>, rope_length: usize) -> usize {
  let last_idx = rope_length - 1;
  let mut rope = vec![Point { x: 0, y: 0 }; rope_length];
  // Excatly same speed than using a hash set :'(
  let mut moves_done: BinaryHeap<Point> = BinaryHeap::new();
  moves_done.push(rope[last_idx].clone());

  for move_elt in moves {
    let mut current_move = move_elt.clone();
    for (cur_idx, next_idx) in (0..last_idx).zip(1..rope_length) {
      let cur_elt = &rope[cur_idx];
      let next_elt = &rope[next_idx];
      let next_elt_move = cur_elt.compute_other_move(next_elt, &current_move);
      let cur_elt_mut = &mut rope[cur_idx];
      cur_elt_mut.move_from(current_move);
      current_move = next_elt_move.clone();
    }
    let tail = &mut rope[last_idx];
    tail.move_from(current_move);
    moves_done.push(tail.clone());
  }

  let mut count = 1;
  let mut last_diff_point = moves_done.pop().unwrap();
  while let Some(point) = moves_done.pop() {
    if point != last_diff_point {
      last_diff_point = point;
      count += 1; 
    }
  }

  count
}

fn part1(moves: &Vec<Move>) {
  let result = compute_tail_moves(moves, 2);
  println!("Part 1 result : {}", result)
}

fn part2(moves: &Vec<Move>) {
  let result = compute_tail_moves(moves, 10);
  println!("Part 2 result 2 : {}", result);
}
