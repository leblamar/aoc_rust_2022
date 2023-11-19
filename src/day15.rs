use std::fs::read_to_string;
use std::collections::HashSet;

pub fn main() {
  println!("It's day 15 !!!");

  let matrix = &mut Matrix::read_matrix();

  part1(matrix);
  part2(&matrix);
}

#[derive(Debug)]
struct Sensor {
  loc: (i32, i32),
  beacon: (i32, i32),
  range: i32
}

impl Sensor {
  fn parse_sensor(line: &str) -> Sensor {
    let (loc_line, beacon_line) = line.split_once(":").unwrap();
    let loc_arr: Vec<i32> = loc_line.split(",")
      .map(|loc| loc.split_once("=").unwrap().1.parse().unwrap())
      .collect();
    let beacon_arr: Vec<i32> = beacon_line.split(",")
      .map(|beacon| beacon.split_once("=").unwrap().1.parse().unwrap())
      .collect();
    Sensor {
      loc: (loc_arr[0], loc_arr[1]),
      beacon: (beacon_arr[0], beacon_arr[1]),
      range: (loc_arr[0] - beacon_arr[0]).abs()
        + (loc_arr[1] - beacon_arr[1]).abs()
    }
  }

  fn dist_from(&self, x: i32, y: i32) -> i32 {
    (self.loc.0 - x).abs()
      + (self.loc.1 - y).abs()
  }

  fn x_dist_from(&self, x: i32) -> i32 {
    (self.loc.0 - x).abs()
  }

  fn y_dist_from(&self, y: i32) -> i32 {
    (self.loc.1 - y).abs()
  }
}

#[derive(Debug)]
struct Matrix {
  sensors: Vec<Sensor>
}

impl Matrix {
  fn read_matrix() -> Matrix {
    let sensors: Vec<Sensor> = read_to_string("src/day15_input.txt")
      .unwrap()
      .lines()
      .map(Sensor::parse_sensor)
      .collect();

    Matrix {
      sensors
    }
  }

  fn get_pos_info_1(&self, x: i32, y: i32, max_x: i32) -> (bool, i32, i32) {
    let mut is_at_dist = false;
    let mut max_next_cur_x = x + 1;
    let mut closest_dist_to_range = max_x - x;
    for sensor in &self.sensors {
      let dist = sensor.dist_from(x, y);
      if sensor.range >= dist {
        let cur_next_x = sensor.loc.0 + sensor.range - (y - sensor.loc.1).abs() + 1;
        is_at_dist = true;
        max_next_cur_x = max_next_cur_x.max(cur_next_x);
      }
      if !is_at_dist && x < sensor.loc.0 {
        let dist_to_range = dist - sensor.range;
        closest_dist_to_range = closest_dist_to_range.min(dist_to_range);
      }
    }
    (is_at_dist, max_next_cur_x, closest_dist_to_range)
  }

  fn is_at_dist(&self, (x, y): (i32, i32)) -> bool {
    for sensor in &self.sensors {
      let x_dist = sensor.x_dist_from(x);
      if sensor.range >= x_dist {
        if sensor.range >= x_dist + sensor.y_dist_from(y) {
          return true;
        }
      }
    }
    return false;
  }
}

fn part1(matrix: &mut Matrix) {
  let min_x = matrix.sensors.iter()
    .map(|sensor| sensor.loc.0 - sensor.range)
    .min()
    .unwrap();
  let max_x = matrix.sensors.iter()
    .map(|sensor| sensor.loc.1 + sensor.range)
    .max()
    .unwrap();

  let test_y = 2_000_000;
  let mut sensors_and_beacons: HashSet<i32> = HashSet::new();
  for sensor in &matrix.sensors {
    if sensor.loc.1 == test_y {
      sensors_and_beacons.insert(sensor.loc.0);
    }
    if sensor.beacon.1 == test_y {
      sensors_and_beacons.insert(sensor.beacon.0);
    }
  }

  let mut cur_x = min_x;
  let mut count = 0;
  let mut iter = 0;
  while cur_x < max_x {
    iter += 1;
    let (is_at_dist, max_next_cur_x, closest_dist_to_range) = matrix.get_pos_info_1(cur_x, test_y, max_x);
    if is_at_dist {
      cur_x = max_next_cur_x;
    } else {
      count += closest_dist_to_range;
      cur_x += closest_dist_to_range;
    }
  }
  println!("Iter: {}", iter);
  let result = (max_x - min_x) - count - sensors_and_beacons.len() as i32;
  println!("Part 1 result : {}", result)
}

fn part2(matrix: &Matrix) {
  let real_max_x = 4_000_000;
  let real_max_y = 4_000_000;

  let directions = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
  let mut beacon = (0, 0);
  'outer: for sensor in &matrix.sensors {
    let mut cur_x = sensor.loc.0 - sensor.range - 1;
    let mut cur_y = sensor.loc.1;
    for (dx, dy) in &directions {
      for _ in 0..=sensor.range {
        if 0 <= cur_x && cur_x <= real_max_x 
          && 0 <= cur_y && cur_y <= real_max_y
          && !matrix.is_at_dist((cur_x, cur_y)) {
          beacon = (cur_x, cur_y);
          break 'outer;
        }
        cur_x += dx;
        cur_y += dy;
      }
    }
  }

  let result = beacon.0 as i64 * 4_000_000 + beacon.1 as i64;
  println!("Part 2 result : {}", result);
}
