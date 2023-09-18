use std::collections::HashMap;

fn get_asteroids(input: &str) -> Vec<(i32, i32)> {
    let mat = input.trim_end().split_ascii_whitespace()
                                   .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut asteroids = Vec::new();
    for y in 0..mat.len() {
        for x in 0..mat[y].len() {
            if mat[y][x] == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }
    asteroids
}

fn sort_by_distance(x: i32, y: i32, asteroids: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ret_vec = asteroids.clone();
    ret_vec.sort_by_key(|(x1, y1)| (x-x1)*(x-x1) + (y-y1)*(y-y1));
    ret_vec.remove(0);
    ret_vec
}

fn get_visible_from(x: i32, y: i32, asteroids: &Vec<(i32, i32)>) -> HashMap<i64, Vec<(i32, i32)>> {
    let list = sort_by_distance(x, y, &asteroids);
    let mut by_angle = HashMap::new();
    for point in &list {
        let mut angle = ((point.1 - y) as f64).atan2((point.0 - x) as f64);
        if angle < 0.0 {
            angle = std::f64::consts::PI * 2.0 + angle;
        }
        angle += std::f64::consts::FRAC_PI_2;
        if angle >= 2.0 * std::f64::consts::PI {
            angle -= 2.0 * std::f64::consts::PI;
        }
        let angle_i = (angle * 1000000.0) as i64;
        if !by_angle.contains_key(&angle_i) {
            by_angle.insert(angle_i, Vec::new());
        }
        by_angle.get_mut(&angle_i).unwrap().push(point.clone());
    }
    by_angle
}

pub fn part_one(input: &str) -> Option<usize> {
    let asteroids = get_asteroids(input);
    Some(asteroids.iter().map(|a| get_visible_from(a.0, a.1, &asteroids).len()).max().unwrap())
}

pub fn part_two(input: &str) -> Option<i32> {
    let asteroids = get_asteroids(input);
    let mut best_asteroid = (asteroids[0].0, asteroids[0].1);
    let mut most = 0;
    for a in &asteroids {
        let num = get_visible_from(a.0, a.1, &asteroids).len();
        if num > most {
            best_asteroid = (a.0, a.1);
            most = num;
        }
    }
    let mut distances = get_visible_from(best_asteroid.0, best_asteroid.1, &asteroids);
    let mut angles = distances.keys().map(|k| *k).collect::<Vec<i64>>();
    angles.sort();
    let mut count = 0;
    loop {
        for a in &angles {
            if distances[&a].len() > 0 {
                count += 1;
                if count == 200 {
                    return Some(100 * distances[&a][0].0 + distances[&a][0].1);
                }
                distances.get_mut(&a).unwrap().remove(0);
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(210));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(802));
    }
}
