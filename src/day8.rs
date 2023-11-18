use std::fs::read_to_string;
use std::time::Instant;

pub fn main() {
  println!("It's day 8 !!!");
  let mut trees = get_forest();
  part1(&trees);
  part2(&mut trees);
}

fn get_forest() -> Vec<Vec<Tree>> {
  read_to_string("src/day8_input.txt")
    .unwrap()
    .lines()
    .into_iter()
    .map(|line| line.chars()
      .into_iter()
      .map(|char| Tree { height: char.to_digit(10).unwrap(), max_idx: 0 })
      .collect()
    )
    .collect()
}

fn find_top_visible_trees(trees: &Vec<Vec<Tree>>, ret: &mut Vec<Vec<bool>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for j in 0..nb_columns {
    let mut max_tree_height = trees[0][j].height;
    ret[0][j] |= true;
    for i in 1..nb_rows {
      let cur_tree_height = trees[i][j].height;
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        ret[i][j] |= true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }
}

fn find_bottom_visible_trees(trees: &Vec<Vec<Tree>>, ret: &mut Vec<Vec<bool>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for j in 0..nb_columns {
    let mut max_tree_height = trees[nb_rows-1][j].height;
    ret[nb_rows-1][j] |= true;
    for i in (0..(nb_rows - 1)).rev() {
      let cur_tree_height = trees[i][j].height;
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        ret[i][j] |= true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }
}

fn find_left_visible_trees(trees: &Vec<Vec<Tree>>, ret: &mut Vec<Vec<bool>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for i in 0..nb_rows {
    let mut max_tree_height = trees[i][0].height;
    ret[i][0] |= true;
    for j in 1..nb_columns {
      let cur_tree_height = trees[i][j].height;
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        ret[i][j] |= true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }
}

fn find_right_visible_trees(trees: &Vec<Vec<Tree>>, ret: &mut Vec<Vec<bool>>) {
  let nb_rows = trees.len();
  let nb_columns = trees.get(0).unwrap().len();

  for i in 0..nb_rows {
    let mut max_tree_height = trees[i][nb_columns-1].height;
    ret[i][nb_columns-1] |= true;
    for j in (0..(nb_columns - 1)).rev() {
      let cur_tree_height = trees[i][j].height;
      if cur_tree_height > max_tree_height {
        max_tree_height = cur_tree_height;
        ret[i][j] |= true;
        if max_tree_height == 9 {
          break;
        }
      }
    }
  }
}

fn part1(trees: &Vec<Vec<Tree>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();
  let start = Instant::now();

  let mut ret_visible = vec![vec![false; nb_columns]; nb_rows];
  find_top_visible_trees(trees, &mut ret_visible);
  find_bottom_visible_trees(trees, &mut ret_visible);
  find_left_visible_trees(trees, &mut ret_visible);
  find_right_visible_trees(trees, &mut ret_visible);

  let result_2: usize = (0..nb_rows)
    .map(|i| (0..nb_columns)
      .filter(|&j| ret_visible[i][j])
      .count()
    )
    .sum();
  
  println!("Part 1 result : {} in {}", result_2, start.elapsed().as_micros())
}

#[derive(Debug)]
struct Tree {
  height: u32,
  max_idx: usize
}

fn update_with_top_values(trees: &mut Vec<Vec<Tree>>, ret_tree: &mut Vec<Vec<usize>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for j in 0..nb_columns {
    for i in 1..nb_rows {
      let height = trees[i][j].height;

      let mut max_position = i - 1;
      let mut cur_tree = &trees[max_position][j];
      while height > cur_tree.height && max_position > 0 {
        max_position = cur_tree.max_idx;
        cur_tree = &trees[max_position][j];
      }

      trees[i][j].max_idx = max_position;
      ret_tree[i][j] *= i - max_position;
    }
  }
}

fn update_with_bottom_values(trees: &mut Vec<Vec<Tree>>, ret_tree: &mut Vec<Vec<usize>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for j in 0..nb_columns {
    for i in (0..(nb_rows - 1)).rev() {
      let height = trees[i][j].height;

      let mut max_position = i + 1;
      let mut cur_tree = &trees[max_position][j];
      while height > cur_tree.height && max_position < nb_rows - 1 {
        max_position = cur_tree.max_idx;
        cur_tree = &trees[max_position][j];
      }

      trees[i][j].max_idx = max_position;
      ret_tree[i][j] *= max_position - i;
    }
  }
}

fn update_with_left_values(trees: &mut Vec<Vec<Tree>>, ret_tree: &mut Vec<Vec<usize>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for i in 0..nb_rows {
    for j in 1..nb_columns {
      let height = trees[i][j].height;

      let mut max_position = j - 1;
      let mut cur_tree = &trees[i][max_position];
      while height > cur_tree.height && max_position > 0 {
        max_position = cur_tree.max_idx;
        cur_tree = &trees[i][max_position];
      }

      trees[i][j].max_idx = max_position;
      ret_tree[i][j] *= j - max_position;
    }
  }
}

fn update_with_right_values(trees: &mut Vec<Vec<Tree>>, ret_tree: &mut Vec<Vec<usize>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  for i in 0..nb_rows {
    for j in (0..(nb_columns - 1)).rev() {
      let height = trees[i][j].height;

      let mut max_position = j + 1;
      let mut cur_tree = &trees[i][max_position];
      while height > cur_tree.height && max_position < nb_columns - 1 {
        max_position = cur_tree.max_idx;
        cur_tree = &trees[i][max_position];
      }

      trees[i][j].max_idx = max_position;
      ret_tree[i][j] *= max_position - j;
    }
  }
}

fn part2(trees: &mut Vec<Vec<Tree>>) {
  let nb_rows = trees.len();
  let nb_columns = trees[0].len();

  let start = Instant::now();

  let mut res_tree = vec![vec![1; nb_columns]; nb_rows];
  update_with_top_values(trees, &mut res_tree);
  update_with_bottom_values(trees, &mut res_tree);
  update_with_left_values(trees, &mut res_tree);
  update_with_right_values(trees, &mut res_tree);

  let result: usize = res_tree.into_iter()
    .flatten()
    .max()
    .unwrap();

  println!("Part 2 result 2 : {} in {}", result, start.elapsed().as_micros());
}
