use advent_of_code::intcode::IntCode;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Color {
    Black,
    White
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn get_new_dir(turn_dir: i64, facing: &Dir) -> Dir {
    match facing {
        Dir::Up => { 
            match turn_dir { 0 => Dir::Left, _ => Dir::Right }
        },
        Dir::Down => { 
            match turn_dir { 0 => Dir::Right, _ => Dir::Left }
        },
        Dir::Left => { 
            match turn_dir { 0 => Dir::Down, _ => Dir::Up }
        },
        Dir::Right => { 
            match turn_dir { 0 => Dir::Up, _ => Dir::Down }
        }
    }
}

fn paint_grid(code: Vec<i64>, initial_color: Color) -> HashMap<(i32, i32), Color> {
    let mut vm = IntCode::new(1024);
    vm.init(code);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = Dir::Up;
    let mut grid: HashMap<(i32, i32), Color> = HashMap::new();
    grid.insert((x, y), initial_color);
    loop {
        if !grid.contains_key(&(x, y)) || grid[&(x,y)] == Color::Black {
            vm.set_input(0);
        } else {
            vm.set_input(1);
        }
        vm.run_until_output();
        if vm.is_finished() {
            break;
        }
        let color_code = vm.get_output().unwrap();
        grid.insert((x, y), match color_code { 0 => Color::Black, _ => Color::White });
        vm.run_until_output();
        let turn_dir = vm.get_output().unwrap();
        dir = get_new_dir(turn_dir, &dir);
        match dir {
            Dir::Up => y -= 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1
        }
    }
    grid
}

pub fn part_one(input: &str) -> Option<usize> {
    let program = input.trim_end().split(',')
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let grid = paint_grid(program, Color::Black);
    Some(grid.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let program = input.trim_end().split(',')
    .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let grid = paint_grid(program, Color::White);
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pos in grid.keys() {
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
    }

    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !grid.contains_key(&(x, y)) || grid[&(x, y)] == Color::Black {
                result += " ";
            } else {
                result += "#";
            }
        }
        result += "\n";
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

