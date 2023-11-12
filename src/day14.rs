use std::fs::read_to_string;
use std::fmt::{self};
use std::thread;
use std::time;

pub fn main() {
  println!("It's day 13 !!!");

  let mut matrix = read_and_parse_file();

  part1(&mut matrix.clone());
  part2(&mut matrix);
}

fn read_and_parse_file() -> Matrix {
    let points: Vec<(Point, Point)> = read_to_string("src/day14_input.txt")
      .unwrap()
      .lines()
      .flat_map(Point::parse_points)
      .collect();
    Matrix::create_matrix(points)
}

#[derive(Debug, Clone)]
struct Point {
  x: i32,
  y: i32
}

#[derive(Debug, Clone)]
enum Cell {
  Block,
  Sand,
  Empty
}

#[derive(Debug, Clone)]
struct Matrix {
  matrix: Vec<Vec<Cell>>,
  min_x: usize
}

impl Point {
  fn parse_point(line_point: &str) -> Point {
    let splited_point: Vec<&str> = line_point.split(",").collect();
    Point {
      x: splited_point[0].parse().unwrap(),
      y: splited_point[1].parse().unwrap()
    }
  }

  fn parse_points(line: &str) -> Vec<(Point, Point)> {
    let points: Vec<Point> = line.split(" -> ")
      .map(Point::parse_point)
      .collect();

    points[0..(points.len()-1)]
      .iter()
      .zip(points[1..points.len()].iter())
      .map(|(p1, p2)| (p1.clone(), p2.clone()))
      .collect()
  }
}

impl Matrix {
  fn get_mins_and_maxs(lines: &Vec<(Point, Point)>) -> (usize, usize, usize) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for (p1, p2) in lines {
      min_x = p1.x.min(p2.x).min(min_x);
      max_x = p1.x.max(p2.x).max(max_x);
      max_y = p1.y.max(p2.y).max(max_y);
    }

    (min_x as usize, max_x as usize, max_y as usize)
  }

  fn fill_matrix_with_line(&mut self, line: &(Point, Point), min_x: usize, min_y: usize) {
    if line.0.x == line.1.x {
      let x = line.0.x as usize - min_x;
      let y0 = line.0.y.min(line.1.y) as usize - min_y;
      let y1 = line.0.y.max(line.1.y) as usize - min_y;
      (y0..=y1)
        .into_iter()
        .for_each(|y| self.matrix[y][x] = Cell::Block);
    } else if  line.0.y == line.1.y {
      let y = line.0.y as usize - min_y;
      let x0 = line.0.x.min(line.1.x) as usize - min_x;
      let x1 = line.0.x.max(line.1.x) as usize - min_x;
      (x0..=x1)
        .into_iter()
        .for_each(|x| self.matrix[y][x] = Cell::Block);
    } else {
      panic!("This should not happend")
    }
  }

  fn create_matrix(lines: Vec<(Point, Point)>) -> Matrix {
    let (min_x, max_x, max_y) = Matrix::get_mins_and_maxs(&lines);
    let final_min_y = 0;
    let final_min_x = min_x - min_x/2 + 99;
    let final_max_x = max_x + min_x/2 - 61;

    let mut matrix = Matrix { 
      matrix: vec![vec![Cell::Empty; final_max_x-final_min_x+1]; max_y-final_min_y+1],
      min_x: final_min_x
    };
    lines.iter()
      .for_each(|line| matrix.fill_matrix_with_line(&line, final_min_x, final_min_y));

    matrix
  }

  fn is_inside(&self, point: &Point, is_final: bool) -> bool{
    0 <= point.x 
      && 0 <= point.y
      && point.x < self.matrix[0].len() as i32 
      && point.y < self.matrix.len() as i32
      && (!is_final || !(point.y == 0 && point.x == 500 - self.min_x as i32))
  }

  fn get(&self, point: &Point) -> &Cell {
    &self.matrix[point.y as usize][point.x as usize]
  }

  fn make_sand_fall(&mut self, sand_point: &Point) -> bool {
    let mut cur_sand_point = sand_point.clone();
    loop {
      let next_y = (cur_sand_point.y + 1) as usize;
      let under_sand_point = Point {
        x: cur_sand_point.x,
        y: next_y as i32
      };
      if !self.is_inside(&under_sand_point, false) {
        cur_sand_point = under_sand_point;
        break;
      }
      match self.get(&under_sand_point) {
        Cell::Empty => {
          cur_sand_point = Point {
            x: cur_sand_point.x,
            y: cur_sand_point.y + 1
          }
        },
        _ => {
          let left_sand_point = Point {
            x: cur_sand_point.x - 1,
            y: next_y as i32
          };
          if !self.is_inside(&left_sand_point, false) {
            cur_sand_point = left_sand_point;
            break;
          }
          match self.get(&left_sand_point) {
            Cell::Empty => cur_sand_point = left_sand_point,
            _ => {
              let right_sand_point = Point {
                x: cur_sand_point.x + 1,
                y: next_y as i32
              };
              if !self.is_inside(&right_sand_point, false) {
                cur_sand_point = right_sand_point;
                break;
              }
              match self.get(&right_sand_point) {
                Cell::Empty => cur_sand_point = right_sand_point,
                _ => break
              };
            }
          };
        }
      }
    }
    if self.is_inside(&cur_sand_point, true) {
      self.matrix[cur_sand_point.y as usize][cur_sand_point.x as usize] = Cell::Sand;
      return true;
    } else {
      return false;
    }
  }
}

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.matrix.len() == 0 || self.matrix[0].len() == 0 {
      return writeln!(f, "No elements");
    }
    let result = writeln!(f, "Matrix:");
    if let Err(_) = result {
      return result;
    }
    for row in self.matrix.iter() {
      for cell in row {
        let result = match cell {
          Cell::Block => write!(f, "#"),
          Cell::Sand => write!(f, "o"),
          Cell::Empty => write!(f, ".")
        };
        if let Err(_) = result {
          return result;
        }
      }
      let result = writeln!(f);
      if let Err(_) = result {
        return result;
      }
    }

    Ok(())
  }
}

fn part1(matrix: &mut Matrix) {
  let debug = false;
  if debug {
    println!("{}", matrix);
  }
  let sand_pouring_point = Point {
    x: 500 - matrix.min_x as i32,
    y: 0
  };
  let waiting_time = time::Duration::from_millis(50);
  let mut is_inside = matrix.is_inside(&sand_pouring_point, false);
  let mut result = 0;
  while is_inside {
    is_inside = matrix.make_sand_fall(&sand_pouring_point);
    if debug && result%500 == 0 {
      println!("Iteration nb {}", result);
      println!("{}", matrix);
      thread::sleep(waiting_time);
    }
    result += 1;
  }
  result -= 1;
  if debug {
    println!("{}", matrix);
  }
  println!("Part 1 result : {}", result)
}

fn part2(matrix: &mut Matrix) {
  let debug = false;
  if debug {
    println!("{}", matrix);
  }
  matrix.matrix.push(vec![Cell::Empty; matrix.matrix[0].len()]);
  matrix.matrix.push(vec![Cell::Block; matrix.matrix[0].len()]);
  let sand_pouring_point = Point {
    x: 500 - matrix.min_x as i32,
    y: 0
  };
  let waiting_time = time::Duration::from_millis(250);
  let mut is_inside = matrix.is_inside(&sand_pouring_point, false);
  let mut result = 0;
  while is_inside {
    is_inside = matrix.make_sand_fall(&sand_pouring_point);
    if debug && result%500 == 0 {
      println!("Iteration nb {}", result);
      println!("{}", matrix);
      thread::sleep(waiting_time);
    }
    result += 1;
  }
  if debug {
    println!("{}", matrix);
  }
  println!("Part 2 result : {}", result);
}
