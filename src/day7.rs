use std::collections::VecDeque;
use std::fs::read_to_string;
use std::fmt::{self};

pub fn main() {
  println!("It's day 7 !!!");
  let lines: Vec<Tokens> = parse_file();
  let heap_tree = HeapTree::create_heap_tree_from_tokens(lines);

  part1(&heap_tree);
  part2(&heap_tree);
}

#[derive(Debug)]
enum Tokens {
  Action(Actions),
  Print(Prints),
  Undefined
}

#[derive(Debug)]
enum Actions {
  CD(Directions),
  LS
}

#[derive(Debug)]
enum Directions {
  Root,
  In(String),
  Out
}

#[derive(Debug)]
enum Prints {
  Dir(String),
  File(usize, String)
}

impl fmt::Display for Tokens {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tokens::Action(action) => match action {
        Actions::CD(direction) => match direction {
          Directions::Root => write!(f, "Action(CD(Root))"),
          Directions::Out => write!(f, "Action(CD(Out))"),
          Directions::In(dir_id) => write!(f, "Action(CD(In({})))", dir_id)
        },
        Actions::LS => write!(f, "Action(LS)")
      },
      Tokens::Print(print) => match print {
        Prints::Dir(dir_id) => write!(f, "Print(Dir({}))", dir_id),
        Prints::File(size, file_id) => write!(f, "Print(File({}, {}))", size, file_id)
      },
      _ => panic!("Bizarre")
    }
  }
}

fn parse_file() -> Vec<Tokens> {
  //let file = read_to_string("src/day7_input_training.txt").unwrap();
  let file = read_to_string("src/day7_input.txt").unwrap();
  let lines = file.lines();

  let mut parsed_lines: Vec<Tokens> = Vec::new();
  for line in lines {
    let split: Vec<&str> = line.split(' ').collect();
    let token: Tokens;
    if line.starts_with('$') {
      token = match *split.get(1).unwrap() {
        "cd" => match *split.get(2).unwrap() {
          "/" => Tokens::Action(Actions::CD(Directions::Root)),
          ".." => Tokens::Action(Actions::CD(Directions::Out)),
          dir => Tokens::Action(Actions::CD(Directions::In(dir.to_string())))
        },
        "ls" => Tokens::Action(Actions::LS),
        _ => Tokens::Undefined
      }
    } else {
      token = if line.starts_with("dir") { Tokens::Print(
        Prints::Dir(
          split.get(1).unwrap().to_string()
        )) } else { Tokens::Print(
          Prints::File(
            split.get(0).unwrap().parse::<usize>().unwrap(), 
            split.get(1).unwrap().to_string()
          )
        )}
    }

    parsed_lines.push(token);
  }

  parsed_lines
}

#[derive(Debug)]
struct HeapTree {
  heap_elements: Vec<HeapElement>
}

#[derive(Debug)]
enum HeapElement {
  Directory(DirectoryElt),
  File(FileElt)
}

#[derive(Debug)]
struct DirectoryElt {
  name: String,
  dir_files: Vec<usize>,
  parent: Option<usize>
}

#[derive(Debug)]
struct FileElt {
  name: String,
  size: usize
}

trait HasName {
  fn get_name(&self) -> &str;
}

impl HasName for DirectoryElt {
  fn get_name(&self) -> &str {
    self.name.as_str()
  }
}

impl HasName for FileElt {
  fn get_name(&self) -> &str {
    self.name.as_str()
  }
}

impl HeapElement {
  fn get_child(&self) -> &dyn HasName {
    match self {
      HeapElement::Directory(child) => child,
      HeapElement::File(child) => child
    }
  }
}

impl fmt::Display for HeapTree {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let nb_elt = self.heap_elements.len();
    if nb_elt == 0 {
      return writeln!(f, "No elements");
    }

    let mut elt_to_treat: VecDeque<usize> = VecDeque::new();
    let mut depth_queue: VecDeque<usize> = VecDeque::new();
    depth_queue.push_front(1);
    let mut cur_elt_opt = self.heap_elements.get(0);
    while let Some(cur_elt) = cur_elt_opt {
      let nb_at_cur_depth_opt = depth_queue.front_mut();
      match nb_at_cur_depth_opt {
        Some(nb_at_cur_depth) => { 
          if *nb_at_cur_depth == 0 {
            // if the current depth is empty, then we must treat the next depth, 
            // but the next depth might be empty too, so we need to redo the same code until founding a not empty depth
            depth_queue.pop_front();
            continue;
          } else {
            *nb_at_cur_depth = *nb_at_cur_depth - 1;
          }
        },
        None => panic!("There is no more element at this depth {}", depth_queue.len() - 1)
      }

      let result;
      let cur_tabs = "  ".repeat(depth_queue.len() - 1);
      match cur_elt {
        HeapElement::Directory(dir) => {
          result = writeln!(f, "{}- {} (dir)", cur_tabs, dir.name);
          dir.dir_files.iter().rev().for_each(|elt| elt_to_treat.push_front(*elt));
          depth_queue.push_front(dir.dir_files.len());
        },
        HeapElement::File(file) => {
          result = writeln!(f, "{}- {} (file, size={})", cur_tabs, file.name, file.size);
        }
      }

      if let Err(_) = result {
        return result;
      }

      cur_elt_opt = match elt_to_treat.pop_front() {
        Some(idx) => {
          //cur_idx = idx;
          self.heap_elements.get(idx)
        },
        None => None
      };
    }
    
    Ok(())
  }
}

impl HeapTree {
  fn create_heap_tree_from_tokens(lines: Vec<Tokens>) -> HeapTree {
    let root = HeapElement::Directory(
      DirectoryElt { name: "/".to_string(), dir_files: Vec::new(), parent: None }
    );
    let mut heap_tree = HeapTree { heap_elements: vec![root] };
  
    let mut cur_dir_idx = 0;
    for line in lines {
      match line {
        Tokens::Action(Actions::CD(Directions::Root)) => cur_dir_idx = 0,
        Tokens::Action(Actions::CD(Directions::In(dir_name))) 
          => cur_dir_idx = heap_tree.find_index_by_name_of(&dir_name, cur_dir_idx),
        Tokens::Action(Actions::CD(Directions::Out)) => {
          cur_dir_idx = match heap_tree.heap_elements.get(cur_dir_idx) {
            Some(HeapElement::Directory(dir)) => match dir.parent {
              Some(new_idx) => new_idx,
              None => panic!("Faire un out quand on est sur root c'est chelou - idx : {}", cur_dir_idx)
            },
            Some(HeapElement::File(file)) => panic!("L'index actuel pointe sur le fichier {}", file.name),
            None => panic!("Il n'y a pas d'élément à cet index : {}", &cur_dir_idx)
          }
        },
        Tokens::Action(Actions::LS) => {},
        Tokens::Print(Prints::Dir(dir_name)) => {
          let new_dir = HeapElement::Directory(DirectoryElt { 
            name: dir_name, 
            dir_files: Vec::new(), 
            parent: Some(cur_dir_idx) 
          });
          heap_tree.add_to(cur_dir_idx, new_dir);
        },
        Tokens::Print(Prints::File(file_size, file_name)) => {
          let new_file = HeapElement::File(FileElt { name: file_name, size: file_size });
          heap_tree.add_to(cur_dir_idx, new_file);
        },
        _ => panic!("Chelou ce token")
      }
    }

    heap_tree
  }

  fn add_to(&mut self, parent_idx: usize, mut new_elt: HeapElement) {
    let push_idx = self.heap_elements.len();
    let parent_elt_opt = self.heap_elements.get_mut(parent_idx); // First mut borrow
    match parent_elt_opt {
      Some(HeapElement::Directory(ref mut dir)) => {
        if let HeapElement::Directory(ref mut new_dir) = new_elt {
          new_dir.parent = Option::Some(parent_idx);
        }
        dir.dir_files.push(push_idx); // End of first mut borrow
        self.heap_elements.push(new_elt); // Second mut borrow after end of first mut borrow so ok
      },
      _ => panic!("This should not arrive cur elt {:?}, parent {:?}", new_elt, parent_elt_opt)
    }
  }

  fn find_index_by_name_of(&mut self, name: &String, parent_idx: usize) -> usize {
    let parent_elt = self.heap_elements.get(parent_idx);
    match parent_elt {
      Some(HeapElement::Directory(parent_dir)) => {
        *parent_dir.dir_files.iter()
          .find(|&&child_idx| match self.heap_elements.get(child_idx) {
            Some(heap_elt) => heap_elt.get_child().get_name() == *name,
            None => panic!("No child find at this index {}", child_idx)
          })
          .unwrap()
      },
      _ => panic!("No parent directory (either no one, or a file)")
    }
  }

  fn get_sizes(&self) -> Vec<usize> {
    let mut sizes: Vec<usize> = vec![0; self.heap_elements.len()];
    self.heap_elements.iter()
      .enumerate()
      .rev()
      .for_each(|(idx, elt)| match elt {
        HeapElement::Directory(dir) => {
          sizes[idx] = dir.dir_files.iter()
            .map(|child_idx| sizes[*child_idx])
            .sum();
        },
        HeapElement::File(file) => {sizes[idx] = file.size;}
      });
    sizes
  }

  fn is_dir(&self, idx: usize) -> bool {
    let elt = &self.heap_elements[idx];
    match elt {
      HeapElement::Directory(_) => true,
      HeapElement::File(_) => false
    }
  }
}

fn part1(heap_tree: &HeapTree) {
  //println!("Root :\n{}", heap_tree);

  let result: usize = heap_tree.get_sizes()
    .iter()
    .enumerate()
    .filter(|(idx, _)| heap_tree.is_dir(*idx))
    .map(|(_, size)| *size)
    .filter(|size| *size < 100_000)
    .sum();
  println!("Part 1 result : {}", result);
}

fn part2(heap_tree: &HeapTree) {
  //println!("Root :\n{}", heap_tree);
  let max_final_size: usize = 40_000_000;

  let heap_tree_sizes = heap_tree.get_sizes();
  let cur_full_size = *heap_tree_sizes.get(0).unwrap();
  
  let result: usize = heap_tree_sizes.into_iter()
    .enumerate()
    .filter(|(idx, _)| heap_tree.is_dir(*idx))
    .map(|(_, size)| size)
    .filter(|&size| cur_full_size - size <= max_final_size)
    .min()
    .unwrap();

  println!("Part 2 result : {}", result);
}
