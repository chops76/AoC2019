use advent_of_code::intcode::IntCode;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

pub fn part_one(input: &str) -> Option<usize> {
    let program = input.trim_end().split(',')
                                 .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut vm = IntCode::new(1024);
    vm.init(program);
    let mut grid:HashMap::<(i64, i64), Tile> = HashMap::new();
    loop {
        vm.run_until_output();
        if vm.is_finished() {
            break;
        }
        let x = vm.get_output().unwrap();
        vm.run_until_output();
        let y = vm.get_output().unwrap();
        vm.run_until_output();
        let tile = match vm.get_output().unwrap() {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            _ => Tile::Ball
        };
        grid.insert((x, y), tile);
    }
    Some(grid.values().filter(|v| **v == Tile::Block).count())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut program = input.trim_end().split(',')
                                        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    program[0] = 2;
    let mut vm = IntCode::new(1024);
    vm.init(program);
    let mut grid:HashMap::<(i64, i64), Tile> = HashMap::new();
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut ball_set = false;
    let mut paddle_set = false;
    let mut score = 0;
    loop {
        vm.run_until_output();
        if vm.is_finished() {
            break;
        }
        let x = vm.get_output().unwrap();
        vm.run_until_output();
        let y = vm.get_output().unwrap();
        vm.run_until_output();
        if x == -1 {
            score = vm.get_output().unwrap();
            vm.set_input(0);
            continue;
        }
        let tile = match vm.get_output().unwrap() {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            _ => Tile::Ball
        };
        if tile == Tile::Ball {
            ball_set = true;
            ball_x = x;
        }
        if tile == Tile::Paddle {
            paddle_set = true;
            paddle_x = x;
        }
        if ball_set && paddle_set && ball_x < paddle_x {
            vm.set_input(-1);
        } else if ball_set && paddle_set && ball_x > paddle_x {
            vm.set_input(1);
        } else {
            vm.set_input(0);
        }
        grid.insert((x, y), tile);
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
