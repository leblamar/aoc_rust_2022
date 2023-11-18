use std::fs::read_to_string;

pub fn main() {
  println!("It's day 6 !!!");
  let file = read_to_string("src/day6_input.txt").unwrap();
  let line = file.lines().nth(0).unwrap();

  part1(line);
  part2(line);
}

fn first_solver(line: &str, k: usize) -> usize {
  return (0..line.len()).into_iter()
    .position(|i| line[i..(i+k)].chars()
      .enumerate()
      .all(|(idx, c)| !line[(i+idx+1)..(i+k)].contains(c))
    )
    .unwrap() + k;
}

fn part1(line: &str) {
  let first_value = first_solver(line, 4);
  println!("Part 1 result : {}", first_value);
}

fn part2(line: &str) {
  let first_value = first_solver(line, 14);
  println!("Part 2 result : {}", first_value);
}
