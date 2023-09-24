use advent_of_code::intcode::IntCode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Exits {
    north: bool,
    south: bool,
    west: bool,
    east: bool,
    oxygen: bool
}

fn discover_map(x: i32, y: i32, oxygen: bool, vm: &mut IntCode, map: &mut HashMap<(i32, i32), Exits>) {
    if map.contains_key(&(x, y)) {
        return;
    }
    
    map.insert((x, y), Exits { north: false, south: false, east: false, west: false, oxygen: oxygen });

    vm.set_input(1);
    vm.run_until_output();
    let result = vm.get_output().unwrap();
    if result != 0 {
        map.get_mut(&(x, y)).unwrap().north = true;
        discover_map(x, y+1, result == 2, vm, map);
        vm.set_input(2);
        vm.run_until_output();
        vm.get_output();
    }

    vm.set_input(2);
    vm.run_until_output();
    let result = vm.get_output().unwrap();
    if result != 0 {
        map.get_mut(&(x, y)).unwrap().south = true;
        discover_map(x, y-1, result == 2, vm, map);
        vm.set_input(1);
        vm.run_until_output();
        vm.get_output();
    }

    vm.set_input(3);
    vm.run_until_output();
    let result = vm.get_output().unwrap();
    if result != 0 {
        map.get_mut(&(x, y)).unwrap().west = true;
        discover_map(x-1, y, result == 2, vm, map);
        vm.set_input(4);
        vm.run_until_output();
        vm.get_output();
    }

    vm.set_input(4);
    vm.run_until_output();
    let result = vm.get_output().unwrap();
    if result != 0 {
        map.get_mut(&(x, y)).unwrap().east = true;
        discover_map(x+1, y, result == 2, vm, map);
        vm.set_input(3);
        vm.run_until_output();
        vm.get_output();
    }
}

fn find_steps(map: &HashMap<(i32, i32), Exits>) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(((0, 0), 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if map[&pos].oxygen {
            return dist;
        }
        if seen.contains(&pos) {
            continue;
        }
        seen.insert(pos);
        if map[&pos].north {
            queue.push_back(((pos.0, pos.1 + 1), dist + 1));
        }
        if map[&pos].south {
            queue.push_back(((pos.0, pos.1 - 1), dist + 1));
        }
        if map[&pos].west {
            queue.push_back(((pos.0 - 1, pos.1), dist + 1));
        }
        if map[&pos].east {
            queue.push_back(((pos.0 + 1, pos.1), dist + 1));
        }
    }
    0
}

fn find_max_steps(origin: &(i32, i32), map: &HashMap<(i32, i32), Exits>) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut max_dist = 0;
    queue.push_back(((origin.0, origin.1), 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if seen.contains(&pos) {
            continue;
        }
        max_dist = dist;
        seen.insert(pos);
        //println!("{:?}:  {}", pos, dist);
        if map[&pos].north {
            queue.push_back(((pos.0, pos.1 + 1), dist + 1));
        }
        if map[&pos].south {
            queue.push_back(((pos.0, pos.1 - 1), dist + 1));
        }
        if map[&pos].west {
            queue.push_back(((pos.0 - 1, pos.1), dist + 1));
        }
        if map[&pos].east {
            queue.push_back(((pos.0 + 1, pos.1), dist + 1));
        }
    }
    max_dist
}

pub fn part_one(input: &str) -> Option<usize> {
    let program = input.trim_end().split(',')
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut vm = IntCode::new(1024);
    vm.init(program);
    let mut map:HashMap<(i32, i32), Exits> = HashMap::new();
    discover_map(0, 0, false, &mut vm, &mut map);
    Some(find_steps(&map))
}

pub fn part_two(input: &str) -> Option<usize> {
    let program = input.trim_end().split(',')
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut vm = IntCode::new(1024);
    vm.init(program);
    let mut map:HashMap<(i32, i32), Exits> = HashMap::new();
    discover_map(0, 0, false, &mut vm, &mut map);
    let oxygen_pos = map.iter().filter(|(_, d)| d.oxygen).next().unwrap().0;
    Some(find_max_steps(oxygen_pos, &map))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
