use std::fs::read_to_string;

pub fn main() {
  println!("It's day 4 !!!");
  let parsed_lines = parse_lines();
  part1(&parsed_lines);
  part2(&parsed_lines);
}

fn parse_lines() -> Vec<Vec<Section>> {
  let file = read_to_string("src/day4_input.txt").unwrap();
  file.lines()
    .map(|line| line.split(',')
      .map(|section_str| from_section_str(section_str))
      .map(|section_list| Section::create_section_from_list(section_list))
      .collect::<Vec<Section>>()
    )
    .collect::<Vec<Vec<Section>>>()
}

struct Section {
  start: i32,
  end: i32
}

impl Section {
  fn create_section_from_list(section_list: Vec<i32>) -> Section {
    Section {
      start: *section_list.get(0).unwrap(),
      end: *section_list.get(1).unwrap()
    }
  }

  fn contain_section(&self, other: &Section) -> bool {
    self.start <= other.start && self.end >= other.end
  }

  fn one_contain_other(&self, other: &Section) -> bool {
    self.contain_section(other) || other.contain_section(self)
  }

  fn sections_overlap(&self, other: &Section) -> bool {
    (self.start <= other.start && other.start <= self.end)
      || (self.start <= other.end && other.end <= self.end)
      || other.contain_section(self)
  }
}

fn from_section_str(section_str: &str) -> Vec<i32> {
  section_str.split('-')
    .map(|string| string.parse::<i32>().unwrap())
    .collect::<Vec<i32>>()
}

fn part1(parsed_lines: &Vec<Vec<Section>>) {
  let result = parsed_lines.into_iter()
    .filter(|sections| sections[0].one_contain_other(&sections[1]))
    .count();

  println!("Part 1 result : {}", result);
}

fn part2(parsed_lines: &Vec<Vec<Section>>) {
let result = parsed_lines.into_iter()
    .filter(|sections| sections[0].sections_overlap(&sections[1]))
    .count();

  println!("Part 2 result : {}", result);
}