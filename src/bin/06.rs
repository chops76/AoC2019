use std::collections::HashMap;

fn get_map(input: &str) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    let spl = input.split_ascii_whitespace().map(|s| s.split(')').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    for planet in spl {
        hm.insert(planet[1].to_string(), planet[0].to_string());
    }
    hm
}

fn depth(object: &str, map: &HashMap<String, String>) -> usize {
    let mut d = 0;
    let mut cur = object.to_string();
    while cur != "COM" {
        cur = map[&cur].to_string();
        d += 1;
    }
    d
}

fn path(object: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut p = vec![object.to_string()];
    let mut cur = object.to_string();
    while cur != "COM" {
        cur = map[&cur].to_string();
        p.push(cur.clone());
    }    
    p.reverse();
    p
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = get_map(input);
    Some(map.keys().map(|k| depth(k, &map)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = get_map(input);
    let p1 = path(&map["YOU"], &map);
    let p2 = path(&map["SAN"], &map);
    let mut pos = 0;
    while pos < p1.len() && pos < p2.len() && p1[pos] == p2[pos] {
        pos += 1;
    }
    Some((p1.len() - pos) + (p2.len() - pos))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(42));
    }
}
