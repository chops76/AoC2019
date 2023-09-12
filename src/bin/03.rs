use std::collections::HashMap;
use std::collections::HashSet;

fn get_positions(instructions: &Vec<&str>) -> HashMap<(i32, i32), usize> {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut count = 0;
    let mut positions = HashMap::new();
    for i in instructions {
        let val = i[1..].parse().unwrap();
        let xd;
        let yd;
        match i.chars().next().unwrap() {
            'U' => { xd = 0; yd = -1 },
            'D' => { xd = 0; yd = 1 },
            'R' => { xd = 1; yd = 0},
            'L' => { xd = -1; yd = 0},
            _ => { println!("Illegal Command");  xd = 0; yd = 0 }
        }
        for _ in 0..val {
            x_pos += xd;
            y_pos += yd;
            count += 1;

            if !positions.contains_key(&(x_pos, y_pos)) {
                positions.insert((x_pos, y_pos), count);
            }
        }
    }
    positions
}

pub fn part_one(input: &str) -> Option<i32> {
    let wire_strings = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let wire1 = wire_strings[0].split(',').collect::<Vec<&str>>();
    let wire2 = wire_strings[1].split(',').collect::<Vec<&str>>();
    let pos1 = get_positions(&wire1);
    let pos2 = get_positions(&wire2);
    let keys1: HashSet<(i32, i32)> = pos1.keys().cloned().collect();
    let keys2 = pos2.keys().cloned().collect();
    let int = keys1.intersection(&keys2);
    Some(int.map(|(x,y)| x.abs() + y.abs()).min().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let wire_strings = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let wire1 = wire_strings[0].split(',').collect::<Vec<&str>>();
    let wire2 = wire_strings[1].split(',').collect::<Vec<&str>>();
    let pos1 = get_positions(&wire1);
    let pos2 = get_positions(&wire2);
    let keys1: HashSet<(i32, i32)> = pos1.keys().cloned().collect();
    let keys2 = pos2.keys().cloned().collect();
    let int = keys1.intersection(&keys2);
    Some(int.map(|(x,y)| pos1[&(*x, *y)] + pos2[&(*x, *y)]).min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(159));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(610));
    }
}
