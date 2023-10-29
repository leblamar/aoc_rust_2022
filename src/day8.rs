use std::fs::read_to_string;
use std::time::Instant;

pub fn main() {
  println!("It's day 8 !!!");
  let trees = get_forest();
  part1(&trees);
  part2(&trees);
}

fn get_forest() -> Vec<Vec<u32>> {
  return read_to_string("src/day8_input.txt")
    .unwrap()
    .lines()
    .into_iter()
    .map(|line| line.chars()
      .into_iter()
      .map(|char| char.to_digit(10).unwrap())
      .collect()
    )
    .collect();
}

fn find_top_visible_trees(trees: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  let mut ret: Vec<Vec<bool>> = (0..nb_rows)
    .map(|_| vec![false; nb_columns])
    .collect();

  for j in 0..nb_columns {
    let mut max_tree_height = *trees.get(0).unwrap().get(j).unwrap();
    *ret.get_mut(0).unwrap().get_mut(j).unwrap() = true;
    for i in 1..nb_rows {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        *ret.get_mut(i).unwrap().get_mut(j).unwrap() = true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }

  ret
}

fn find_bottom_visible_trees(trees: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  let mut ret: Vec<Vec<bool>> = (0..nb_rows)
    .map(|_| vec![false; nb_columns])
    .collect();

  for j in 0..nb_columns {
    let mut max_tree_height = *trees.get(nb_rows - 1).unwrap().get(j).unwrap();
    *ret.get_mut(nb_rows - 1).unwrap().get_mut(j).unwrap() = true;
    for i in (0..(nb_rows - 1)).rev() {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        *ret.get_mut(i).unwrap().get_mut(j).unwrap() = true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }

  ret
}

fn find_left_visible_trees(trees: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  let mut ret: Vec<Vec<bool>> = (0..nb_rows)
    .map(|_| vec![false; nb_columns])
    .collect();

  for i in 0..nb_rows {
    let mut max_tree_height = *trees.get(i).unwrap().get(0).unwrap();
    *ret.get_mut(i).unwrap().get_mut(0).unwrap() = true;
    for j in 1..nb_columns {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        *ret.get_mut(i).unwrap().get_mut(j).unwrap() = true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }

  ret
}

fn find_right_visible_trees(trees: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  let mut ret: Vec<Vec<bool>> = (0..nb_rows)
    .map(|_| vec![false; nb_columns])
    .collect();

  for i in 0..nb_rows {
    let mut max_tree_height = *trees.get(i).unwrap().get(nb_columns - 1).unwrap();
    *ret.get_mut(i).unwrap().get_mut(nb_columns - 1).unwrap() = true;
    for j in (0..(nb_columns - 1)).rev() {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        *ret.get_mut(i).unwrap().get_mut(j).unwrap() = true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }

  ret
}

fn part1(trees: &Vec<Vec<u32>>) {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();
  let start = Instant::now();

  let top_visible = find_top_visible_trees(&trees);
  let bottom_visible = find_bottom_visible_trees(&trees);
  let left_visible = find_left_visible_trees(&trees);
  let right_visible = find_right_visible_trees(&trees);

  let result_2: usize = (0..nb_rows)
    .map(|i| (0..nb_columns)
      .filter(|&j| *top_visible.get(i).unwrap().get(j).unwrap()
        || *bottom_visible.get(i).unwrap().get(j).unwrap()
        || *left_visible.get(i).unwrap().get(j).unwrap()
        || *right_visible.get(i).unwrap().get(j).unwrap()
      )
      .count()
    )
    .sum();
  
  println!("Part 1 result : {} in {}", result_2, start.elapsed().as_micros())
}

fn count_top_tree_views(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();
  
  let mut ret: Vec<Vec<u32>> = (0..nb_rows)
    .map(|_| vec![0; nb_columns])
    .collect();

  for j in 0..nb_columns {
    let mut prev_tree_height = *trees.get(0).unwrap().get(j).unwrap();
    let mut prev_tree_nb = 0;
    *ret.get_mut(0).unwrap().get_mut(j).unwrap() = 0;
    for i in 1..nb_rows {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      let cur_tree_nb = if cur_tree_height > prev_tree_height {prev_tree_nb + 1} else {1};

      *ret.get_mut(i).unwrap().get_mut(j).unwrap() = cur_tree_nb;
      
      prev_tree_height = cur_tree_height;
      prev_tree_nb = cur_tree_nb;
    }
  }

  ret
}

fn count_bottom_tree_views(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();
  
  let mut ret: Vec<Vec<u32>> = (0..nb_rows)
    .map(|_| vec![0; nb_columns])
    .collect();

  for j in 0..nb_columns {
    let mut prev_tree_height = *trees.get(nb_rows - 1).unwrap().get(j).unwrap();
    let mut prev_tree_nb = 0;
    *ret.get_mut(nb_rows - 1).unwrap().get_mut(j).unwrap() = 0;
    for i in (0..(nb_rows - 1)).rev() {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      let cur_tree_nb = if cur_tree_height > prev_tree_height {prev_tree_nb + 1} else {1};

      *ret.get_mut(i).unwrap().get_mut(j).unwrap() = cur_tree_nb;
      
      prev_tree_height = cur_tree_height;
      prev_tree_nb = cur_tree_nb;
    }
  }

  ret
}

fn count_left_tree_views(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();
  
  let mut ret: Vec<Vec<u32>> = (0..nb_rows)
    .map(|_| vec![0; nb_columns])
    .collect();

  for i in 0..nb_rows {
    let mut prev_tree_height = *trees.get(i).unwrap().get(0).unwrap();
    let mut prev_tree_nb = 0;
    *ret.get_mut(i).unwrap().get_mut(0).unwrap() = 0;
    for j in 1..nb_columns {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      let cur_tree_nb = if cur_tree_height > prev_tree_height {prev_tree_nb + 1} else {1};

      *ret.get_mut(i).unwrap().get_mut(j).unwrap() = cur_tree_nb;
      
      prev_tree_height = cur_tree_height;
      prev_tree_nb = cur_tree_nb;
    }
  }

  ret
}

fn count_right_tree_views(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();
  
  let mut ret: Vec<Vec<u32>> = (0..nb_rows)
    .map(|_| vec![0; nb_columns])
    .collect();

  for i in 0..nb_rows {
    let mut prev_tree_height = *trees.get(i).unwrap().get(nb_columns - 1).unwrap();
    let mut prev_tree_nb = 0;
    *ret.get_mut(i).unwrap().get_mut(nb_columns - 1).unwrap() = 0;
    for j in (0..(nb_columns - 1)).rev() {
      let cur_tree_height = *trees.get(i).unwrap().get(j).unwrap();
      let cur_tree_nb = if cur_tree_height > prev_tree_height {prev_tree_nb + 1} else {1};

      *ret.get_mut(i).unwrap().get_mut(j).unwrap() = cur_tree_nb;
      
      prev_tree_height = cur_tree_height;
      prev_tree_nb = cur_tree_nb;
    }
  }

  ret
}

fn count_top_tree(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
  let height = *trees.get(x).unwrap().get(y).unwrap();
  let mut counter = 0;
  for i in (0..x).rev() {
    let cur_height = *trees.get(i).unwrap().get(y).unwrap();
    counter += 1;
    if cur_height >= height {
      return counter;
    }
  }

  counter
}

fn count_bottom_tree(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
  let height = *trees.get(x).unwrap().get(y).unwrap();
  let mut counter = 0;
  for i in (x+1)..trees.len() {
    let cur_height = *trees.get(i).unwrap().get(y).unwrap();
    counter += 1;
    if cur_height >= height {
      return counter;
    }
  }

  counter
}

fn count_left_tree(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
  let height = *trees.get(x).unwrap().get(y).unwrap();
  let mut counter = 0;
  for j in (0..y).rev() {
    let cur_height = *trees.get(x).unwrap().get(j).unwrap();
    counter += 1;
    if cur_height >= height {
      return counter;
    }
  }

  counter
}

fn count_right_tree(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
  let height = *trees.get(x).unwrap().get(y).unwrap();
  let mut counter = 0;
  for j in (y+1)..trees.get(0).unwrap().len() {
    let cur_height = *trees.get(x).unwrap().get(j).unwrap();
    counter += 1;
    if cur_height >= height {
      return counter;
    }
  }

  counter
}

fn part2(trees: &Vec<Vec<u32>>) {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  let start = Instant::now();

  let top_nb_trees = count_top_tree_views(trees);
  let bottom_nb_trees = count_bottom_tree_views(trees);
  let left_nb_trees = count_left_tree_views(trees);
  let right_nb_trees = count_right_tree_views(trees);

  let result: u32 = (0..nb_rows)
    .map(|i| (0..nb_columns)
      .map(|j| *top_nb_trees.get(i).unwrap().get(j).unwrap()
        * *bottom_nb_trees.get(i).unwrap().get(j).unwrap()
        * *left_nb_trees.get(i).unwrap().get(j).unwrap()
        * *right_nb_trees.get(i).unwrap().get(j).unwrap()
      )
      .max()
      .unwrap()
    )
    .max()
    .unwrap();

  println!("Part 2 result : {} in {}", result, start.elapsed().as_micros());

  let start_2 = Instant::now();

  let result_2: u32 = (0..nb_rows)
    .map(|i| (0..nb_columns)
      .map(|j| count_top_tree(trees, i, j)
        * count_bottom_tree(trees, i, j)
        * count_left_tree(trees, i, j)
        * count_right_tree(trees, i, j)
      )
      .max()
      .unwrap()
    )
    .max()
    .unwrap();

  println!("Part 2 result 2 : {} in {}", result_2, start_2.elapsed().as_micros())
}
