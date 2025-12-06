use regex::Regex;
use std::{collections::{BTreeSet, HashMap}, fs::read_to_string, time::Instant};

pub fn main() {
  println!("It's day 16 !!!");

  let valves = &Valve::read_valves();

  //let start = Instant::now();
  //part1(valves);
  //let elapsed_time = start.elapsed();
  //println!("Elapsped time part 1: {:.2?}", elapsed_time);

  let valves_bitmask = &BitMaskValve::read_valves(valves);

  //let start_opti = Instant::now();
  //part1_opti(valves_bitmask);
  //let elapsed_time_opti = start_opti.elapsed();
  //println!("Elapsped time part 1 opti: {:.2?}", elapsed_time_opti);

  //let start_p2 = Instant::now();
  //part2(valves);
  //let elapsed_time_p2 = start_p2.elapsed();
  //println!("Elapsped time part 2: {:.2?}", elapsed_time_p2);

  let start_opti_p2 = Instant::now();
  part2_opti(valves_bitmask);
  let elapsed_time_opti_p2 = start_opti_p2.elapsed();
  println!("Elapsped time part 2 opti: {:.2?}", elapsed_time_opti_p2);
}

#[derive(Debug)]
struct Valve {
  id: String,
  rate: u64,
  lead_to: Vec<String>
}

impl From<&str> for Valve {
  fn from(value: &str) -> Self {
    let regx = Regex::new("Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
    let valve_opt = regx.captures(value);
    if let Some(valve) = valve_opt {
      if valve.len() != 4 {
        panic!("It should have exactly 4 groups captured");
      }
      let id = valve[1].to_string();
      let rate = valve[2].parse::<u64>().expect("Failed parsing rate");
      let lead_to = valve[3].split(", ").map(str::to_string).collect::<Vec<String>>();

      return Valve {
        id,
        rate,
        lead_to
      };
    } else {
      panic!("We shouldn't arrive here");
    }
  }
}

impl Valve {
  fn read_valves() -> Vec<Valve> {
    read_to_string("src/day16_input.txt")
      .unwrap()
      .lines()
      .filter(|valve_str| !valve_str.trim().is_empty())
      .map(Valve::from)
      .collect::<Vec<Valve>>()
  }
}

struct BitMaskValve {
  id: u8,
  rate: u8,
  lead_to: u64
}

impl BitMaskValve {
  fn is_in(&self, bitmask: u64) -> bool {
    1 == ((bitmask >> self.id) & 1)
  }

  fn can_lead_to(&self, next_id: u8) -> bool {
    1 == ((self.lead_to >> next_id) & 1)
  }

  fn read_valves(valves: &Vec<Valve>) -> Vec<BitMaskValve> {
      let mut keys: Vec<&String> = valves.iter().map(|v| &v.id).collect();
      keys.sort();

      let string_to_mask: HashMap<&String, u8> = keys.iter().enumerate().map(|(i, &v)| (v, i as u8)).collect();

      valves.iter()
        .map(|v| BitMaskValve{ 
          id: string_to_mask[&v.id], 
          rate: v.rate as u8, 
          lead_to: v.lead_to.iter()
            .map(|v| string_to_mask[v])
            .fold(0_u64, |mask, next_id| mask | (1 << next_id))
        })
        .collect::<Vec<BitMaskValve>>()
  }
}

fn found_best_solution_bitmask(
  valves: &HashMap<u8, &BitMaskValve>, 
  v: &BitMaskValve, 
  left_iter: u8,
  open_valves_mask: &mut u64,
  openable_valves: u8,
  res_map: &mut HashMap<(u64, u8, u8), u64>
) -> u64 {
  if left_iter == 0 || openable_valves == 0 {
    return 0;
  }
  if let Some(&res) = res_map.get(&(*open_valves_mask, v.id, left_iter)) {
    return res;
  }

  let next_iter = left_iter - 1;
  let mut open_res: u64 = 0;
  if v.rate != 0 && !v.is_in(*open_valves_mask) {
    let new_openable_valves = openable_valves - 1;
    *open_valves_mask |= 1 << v.id;
    let will_get = (v.rate as u64) * (next_iter as u64);

    open_res = will_get + found_best_solution_bitmask(
      valves, 
      v, 
      next_iter, 
      open_valves_mask, 
      new_openable_valves,
      res_map
    );
    *open_valves_mask &= !(1 << v.id);
  }

  for i in 0..64_u8 {
    let next_id = (v.lead_to >> i) & 1;
    if next_id == 0 {
      continue;
    }


  }

  let res = (0..64_u8).filter(|&next_id| v.can_lead_to(next_id))
    .map(|next_id| valves[&next_id])
    .map(|next_v| found_best_solution_bitmask(
      valves, 
      next_v, 
      next_iter, 
      open_valves_mask, 
      openable_valves,
      res_map
    ))
    .max()
    .map(|max_res| max_res.max(open_res))
    .expect("This should not happen");

  res_map.insert((*open_valves_mask, v.id, left_iter), res);
  res
}

fn found_best_solution(
  valves: &HashMap<String, &Valve>, 
  cur_valve: &Valve, 
  left_iter: u64,
  open_valves_set: &mut BTreeSet<String>,
  openable_valves: usize,
  res_map: &mut HashMap<(String, String, u64), u64>
) -> u64 {
  if left_iter == 0 || openable_valves == 0 {
    return 0;
  }
  let open_v_str: String = open_valves_set.iter().map(|v| v.clone()).collect::<Vec<String>>().join("");
  if let Some(&res) = res_map.get(&(open_v_str.clone(), cur_valve.id.clone(), left_iter)) {
    return res;
  }

  let next_iter = left_iter - 1;
  let mut open_res: u64 = 0;
  if cur_valve.rate != 0 && !open_valves_set.contains(&cur_valve.id) {
    let new_openable_valves = openable_valves - 1;
    open_valves_set.insert(cur_valve.id.clone());
    let will_get = cur_valve.rate * next_iter;

    open_res = will_get + found_best_solution(
      valves, 
      cur_valve, 
      next_iter, 
      open_valves_set, 
      new_openable_valves,
      res_map
    );
    open_valves_set.remove(&cur_valve.id.clone());
  }

  let res = cur_valve.lead_to.iter()
    .map(|next_id| valves[next_id])
    .map(|next_v| found_best_solution(
      valves, 
      next_v, 
      next_iter, 
      open_valves_set, 
      openable_valves,
      res_map
    ))
    .max()
    .map(|max_res| max_res.max(open_res))
    .expect("This should not happen");

  res_map.insert((open_v_str, cur_valve.id.clone(), left_iter), res);
  res
}

fn part1(valves: &Vec<Valve>) {
  let map_of_valves: HashMap<String, &Valve> = valves.iter()
    .map(|valve| (valve.id.clone(), valve))
    .collect();
  let first_valve = valves.iter().find(|v| v.id == "AA").expect("There must be always a AA");
  let init_iter: u64 = 30;
  let openable_valves = valves.iter().filter(|v| v.rate != 0).count();
  let mut res_map: HashMap<(String, String, u64), u64> = HashMap::with_capacity(1_000_000);

  let mut open_valves_set: BTreeSet<String> = BTreeSet::new();
  let result = found_best_solution(
    &map_of_valves,
    first_valve,
    init_iter,
    &mut open_valves_set,
    openable_valves,
    &mut res_map
  );
  println!("Part 1 result : {}", result);
  println!("Resmap size: {}", res_map.len());
}

fn part1_opti(valves: &Vec<BitMaskValve>) {
  let map_of_valves: HashMap<u8, &BitMaskValve> = valves.iter()
    .map(|valve| (valve.id, valve))
    .collect();
  let first_valve = valves.iter().find(|v| v.id == 0).expect("There must be always a AA which is 0 mask");
  let init_iter: u8 = 30;
  let openable_valves = valves.iter().filter(|v| v.rate != 0).count() as u8;
  let mut res_map: HashMap<(u64, u8, u8), u64> = HashMap::with_capacity(1_000_000);

  let mut open_valves_mask = 0_u64;
  let result = found_best_solution_bitmask(
    &map_of_valves,
    first_valve,
    init_iter,
    &mut open_valves_mask,
    openable_valves,
    &mut res_map
  );
  println!("Part 1 result : {}", result);
  println!("Resmap size: {}", res_map.len());
}

fn found_best_solution_v2(
  valves: &HashMap<String, &Valve>, 
  cur_valve: &Valve, 
  cur_valve_e: &Valve, 
  left_iter: u64,
  open_valves_set: &mut BTreeSet<String>,
  openable_valves: usize,
  res_map: &mut HashMap<(String, String, String, u64), u64>
) -> u64 {
  if left_iter == 0 || openable_valves == 0 {
    return 0;
  }
  let open_v_str: String = open_valves_set.iter().map(|v| v.clone()).collect::<Vec<String>>().join("");
  if let Some(&res) = res_map.get(&(open_v_str.clone(), cur_valve.id.clone(), cur_valve_e.id.clone(), left_iter)) {
    return res;
  } else if let Some(&res) = res_map.get(&(open_v_str.clone(), cur_valve_e.id.clone(), cur_valve.id.clone(), left_iter)) {
    return res;
  }

  let next_iter = left_iter - 1;
  let mut open_res: u64 = 0;
  if cur_valve.rate != 0 && !open_valves_set.contains(&cur_valve.id) {
    if cur_valve.id != cur_valve_e.id && cur_valve_e.rate != 0 && !open_valves_set.contains(&cur_valve_e.id) {
      let new_openable_valves = openable_valves - 2;
      open_valves_set.insert(cur_valve.id.clone());
      open_valves_set.insert(cur_valve_e.id.clone());
      let will_get = (cur_valve.rate + cur_valve_e.rate) * next_iter;

      open_res = will_get + found_best_solution_v2(
        valves, 
        cur_valve, 
        cur_valve_e,
        next_iter, 
        open_valves_set, 
        new_openable_valves,
        res_map
      );
      open_valves_set.remove(&cur_valve.id.clone());
      open_valves_set.remove(&cur_valve_e.id.clone());
    } else {
      let new_openable_valves = openable_valves - 1;
      open_valves_set.insert(cur_valve.id.clone());
      let will_get = cur_valve.rate * next_iter;

      open_res = cur_valve_e.lead_to.iter()
        .map(|id_e| valves[id_e])
        .map(|v_e| found_best_solution_v2(
          valves, 
          cur_valve, 
          v_e,
          next_iter, 
          open_valves_set, 
          new_openable_valves,
          res_map
        ))
        .max()
        .expect("This should not happen e");

      open_res += will_get;
      open_valves_set.remove(&cur_valve.id.clone());
    }
  } else if cur_valve_e.rate != 0 && !open_valves_set.contains(&cur_valve_e.id) {
      let new_openable_valves = openable_valves - 1;
      open_valves_set.insert(cur_valve_e.id.clone());
      let will_get = cur_valve_e.rate * next_iter;

      open_res = cur_valve.lead_to.iter()
        .map(|id| valves[id])
        .map(|v| found_best_solution_v2(
          valves, 
          v, 
          cur_valve_e,
          next_iter, 
          open_valves_set, 
          new_openable_valves,
          res_map
        ))
        .max()
        .expect("This should not happen e");

      open_res += will_get;
      open_valves_set.remove(&cur_valve_e.id.clone());
  }

  let res = cur_valve.lead_to.iter()
    .flat_map(|next_id| cur_valve_e.lead_to.iter().map(|next_id_e| (next_id.clone(), next_id_e.clone())))
    .map(|(id, id_e)| (valves[&id], valves[&id_e]))
    .map(|(v, v_e)| found_best_solution_v2(
      valves, 
      v, 
      v_e,
      next_iter, 
      open_valves_set, 
      openable_valves,
      res_map
    ))
    .max()
    .map(|max_res| max_res.max(open_res))
    .expect("This should not happen");

  res_map.insert((open_v_str, cur_valve.id.clone(), cur_valve_e.id.clone(), left_iter), res);
  res
}

fn found_best_solution_bitmask_v2(
  valves: &HashMap<u8, &BitMaskValve>, 
  v: &BitMaskValve, 
  v_e: &BitMaskValve, 
  left_iter: u8,
  open_valves_mask: &mut u64,
  openable_valves: u8,
  res_map: &mut HashMap<(u64, u8, u8, u8), u64>
) -> u64 {
  if left_iter == 0 || openable_valves == 0 {
    return 0;
  }
  if let Some(&res) = res_map.get(&(*open_valves_mask, v.id, v_e.id, left_iter)) {
    return res;
  } else if let Some(&res) = res_map.get(&(*open_valves_mask, v_e.id, v.id, left_iter)) {
    return res;
  }

  let next_iter = left_iter - 1;
  let mut open_res: u64 = 0;
  if v.rate != 0 && !v.is_in(*open_valves_mask) {
    if v.id != v_e.id && v_e.rate != 0 && !v_e.is_in(*open_valves_mask) {
      let new_openable_valves = openable_valves - 2;

      *open_valves_mask |= (1 << v.id) | (1 << v_e.id);
      let will_get = ((v.rate + v_e.rate) as u64) * (next_iter as u64);

      open_res = will_get + found_best_solution_bitmask_v2(
        valves, 
        v,
        v_e,
        next_iter, 
        open_valves_mask,
        new_openable_valves,
        res_map
      );
      *open_valves_mask &= !(1 << v.id) & !(1 << v_e.id);
    } else {
      let new_openable_valves = openable_valves - 1;
      *open_valves_mask |= 1 << v.id;
      let will_get = (v.rate as u64) * (next_iter as u64);

      open_res = (0..64_u8).filter(|&id_e| v_e.can_lead_to(id_e))
        .map(|id_e| valves[&id_e])
        .map(|next_v_e| found_best_solution_bitmask_v2(
          valves, 
          v, 
          next_v_e,
          next_iter, 
          open_valves_mask, 
          new_openable_valves,
          res_map
        ))
        .max()
        .expect("This should not happen e");

      open_res += will_get;
      *open_valves_mask &= !(1 << v.id);
    }
  } else if v_e.rate != 0 && !v_e.is_in(*open_valves_mask) {
      let new_openable_valves = openable_valves - 1;
      *open_valves_mask |= 1 << v_e.id;
      let will_get = (v_e.rate as u64) * (next_iter as u64);

      open_res = (0..64_u8).filter(|&id| v.can_lead_to(id))
        .map(|id| valves[&id])
        .map(|next_v| found_best_solution_bitmask_v2(
          valves, 
          next_v, 
          v_e,
          next_iter, 
          open_valves_mask, 
          new_openable_valves,
          res_map
        ))
        .max()
        .expect("This should not happen");

      open_res += will_get;
      *open_valves_mask &= !(1 << v_e.id);
  }

  let res = (0..64_u8).filter(|next_id| v.can_lead_to(*next_id))
    .flat_map(|next_id| (0..64_u8).filter(|next_id_e| v_e.can_lead_to(*next_id_e)).map(move |next_id_e| (next_id, next_id_e)))
    .map(|(id, id_e)| (valves[&id], valves[&id_e]))
    .map(|(v, v_e)| found_best_solution_bitmask_v2(
      valves, 
      v, 
      v_e,
      next_iter, 
      open_valves_mask, 
      openable_valves,
      res_map
    ))
    .max()
    .map(|max_res| max_res.max(open_res))
    .expect("This should not happen");

  res_map.insert((*open_valves_mask, v.id, v_e.id, left_iter), res);
  res
}

fn part2(valves: &Vec<Valve>) {
  let map_of_valves: HashMap<String, &Valve> = valves.iter()
    .map(|valve| (valve.id.clone(), valve))
    .collect();
  let first_valve = valves.iter().find(|v| v.id == "AA").expect("There must be always a AA");
  let init_iter: u64 = 15;
  let openable_valves = valves.iter().filter(|v| v.rate != 0).count();
  let mut res_map: HashMap<(String, String, String, u64), u64> = HashMap::with_capacity(2_000_000);

  let mut open_valves_map: BTreeSet<String> = BTreeSet::new();
  let result = found_best_solution_v2(
    &map_of_valves,
    first_valve,
    first_valve,
    init_iter,
    &mut open_valves_map,
    openable_valves,
    &mut res_map
  );

  println!("Part 2 result : {}", result);
  println!("Resmap size: {}", res_map.len());
}

fn part2_opti(valves: &Vec<BitMaskValve>) {
  let map_of_valves: HashMap<u8, &BitMaskValve> = valves.iter()
    .map(|valve| (valve.id, valve))
    .collect();
  let first_valve = valves.iter().find(|v| v.id == 0).expect("There must be always a AA which is 0 mask");
  let init_iter = 20_u8;
  let openable_valves = valves.iter().filter(|v| v.rate != 0).count() as u8;
  let mut res_map: HashMap<(u64, u8, u8, u8), u64> = HashMap::with_capacity(20_000_000);

  let mut open_valves_mask = 0_u64;
  let result = found_best_solution_bitmask_v2(
    &map_of_valves,
    first_valve,
    first_valve,
    init_iter,
    &mut open_valves_mask,
    openable_valves,
    &mut res_map
  );

  println!("Part 2 result : {}", result);
  println!("Resmap size: {}", res_map.len());
}
