fn calc1(weight: u64) -> u64 {
    weight / 3 - 2
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.split_whitespace().map(|w| calc1(w.parse().unwrap())).sum())
}

fn calc2(weight: u64) -> u64 {
    let mut fuel:i64 = weight as i64;
    let mut sum = 0;
    fuel = fuel / 3 - 2;
    while fuel > 0 {
        sum += fuel;
        fuel = fuel / 3 - 2;
    }    
    sum as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.split_whitespace().map(|w| calc2(w.parse().unwrap())).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(33583));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(50346));
    }
}
