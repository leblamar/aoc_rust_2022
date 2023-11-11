use std::{fs::read_to_string, collections::VecDeque};

pub fn main() {
  println!("It's day 12 !!!");

  let matrix = Matrix::read_matrix();

  part1(&matrix);
  part2(&matrix);
}

#[derive(Debug, Clone)]
struct Cell {
  value: u32,
  cost: u32,
  is_start: bool,
  is_end: bool
}

#[derive(Debug, Clone)]
struct Matrix {
  cells: Vec<Vec<Cell>>,
  sizes: (usize, usize),
  pos_s: (usize, usize),
  pos_e: (usize, usize)
}

impl Matrix {
  fn read_value(letter: char) -> Cell {
    let is_start = letter == 'S';
    let is_end = letter == 'E';
    Cell {
      value: if is_end { 'z' } else if is_start { 'a' } else { letter } as u32,
      cost: if is_end { 0 } else { u32::MAX },
      is_start,
      is_end
    }
  }

  fn read_matrix_row(line: &str) -> Vec<Cell> {
    line.chars()
      .map(Matrix::read_value)
      .collect()
  }

  fn read_matrix() -> Matrix {
    let cells: Vec<Vec<Cell>> = read_to_string("src/day12_input.txt")
      .unwrap()
      .lines()
      .map(Matrix::read_matrix_row)
      .collect();
    let nb_row = cells.len();
    let nb_col = cells.get(0).unwrap().len();

    let mut matrix = Matrix {
      cells,
      sizes: (nb_row, nb_col),
      pos_s: (0, 0),
      pos_e: (0, 0)
    };

    matrix.pos_s = matrix.find_start_idx();
    matrix.pos_e = matrix.find_end_idx();
    matrix
  }

  fn find_by_value(&self, search_start: bool) -> (usize, usize) {
    let row_idx = self.cells
      .iter()
      .position(|row| row.iter()
        .find(|&cell| if search_start { cell.is_start } else { cell.is_end })
        .is_some()
      )
      .unwrap();

    let col_idx = self.cells
      .get(row_idx)
      .unwrap()
      .iter()
      .position(|cell| if search_start { cell.is_start } else { cell.is_end })
      .unwrap();

    (row_idx, col_idx)
  }

  fn find_start_idx(&self) -> (usize, usize) {
    self.find_by_value(true)
  }

  fn find_end_idx(&self) -> (usize, usize) {
    self.find_by_value(false)
  }

  fn get_directions() -> [(i32, i32); 4] {
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
  }

  fn is_inside(&self, (pos_y, pos_x): (i32, i32)) -> bool {
    0 <= pos_y && pos_y < self.sizes.0 as i32
      && 0 <= pos_x && pos_x < self.sizes.1 as i32
  }

  fn get_cell_i(&self, (y, x): (i32, i32)) -> &Cell {
    self.cells.get(y as usize).unwrap()
      .get(x as usize).unwrap()
  }

  fn get_cell(&self, (y, x): (usize, usize)) -> &Cell {
    self.cells.get(y).unwrap()
      .get(x).unwrap()
  }

  fn get_friends(&self, pos: (usize, usize)) -> Vec<(i32, i32, Cell)> {
    let val = self.get_cell(pos).value;
    Matrix::get_directions()
      .iter()
      .map(|(dx, dy)| (pos.0 as i32 + dy, pos.1 as i32 + dx))
      .filter(|&pos| self.is_inside(pos))
      .map(|pos| (pos.0, pos.1, self.get_cell_i(pos).clone()))
      .filter(|(_, _, cell)| cell.value >= val - 1)
      .collect()
  }

  fn resolve_costs(&mut self) {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; self.sizes.1]; self.sizes.0];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front(self.pos_e);

    while let Some((row_y, col_x)) = queue.pop_back() {
      if is_visited(&visited, (row_y, col_x)) {
          continue;
      }
      set_visited(&mut visited, (row_y, col_x));
      let cell = self.get_cell((row_y, col_x)).clone();
      for (new_row_y, new_col_x, new_cell) in self.get_friends((row_y, col_x)) {
        if new_cell.cost > cell.cost + 1 {
          self.cells.get_mut(new_row_y as usize).unwrap()
            .get_mut(new_col_x as usize).unwrap()
            .cost = cell.cost + 1;
        }
        queue.push_front((new_row_y as usize, new_col_x as usize));
      }
    }
  }
}

fn is_visited(visited: &Vec<Vec<bool>>, (y, x): (usize, usize)) -> bool {
  *visited.get(y).unwrap()
    .get(x).unwrap()
}

fn set_visited(visited: &mut Vec<Vec<bool>>, (y, x): (usize, usize)) {
  *visited.get_mut(y).unwrap()
    .get_mut(x).unwrap() = true;
}

fn part1(matrix: &Matrix) {
  let cloned_matrix: &mut Matrix = &mut matrix.clone();
  cloned_matrix.resolve_costs();

  let result = cloned_matrix.get_cell(cloned_matrix.pos_s).cost;
  println!("Part 1 result : {}", result)
}

fn part2(matrix: &Matrix) {
  let cloned_matrix: &mut Matrix = &mut matrix.clone();
  cloned_matrix.resolve_costs();

  let result = cloned_matrix.cells
    .iter()
    .flatten()
    .filter(|cell| cell.value == 'a' as u32)
    .min_by_key(|cell| cell.cost)
    .unwrap()
    .cost;  
  println!("Part 2 result : {}", result);
}
