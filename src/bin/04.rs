fn check(input: u32, p2: bool) -> bool {
    let mut total = input;
    let mut digits = Vec::new();
    for _ in 0..6 {
        digits.push(total % 10);
        total /= 10;
    }
    digits.reverse();
    let mut same = false;
    for i in 0..5 {
        if digits[i] > digits[i+1] {
            return false;
        }
        if digits[i] == digits[i+1] && !(i < 4 && p2 && digits[i] == digits[i+2]) &&
                                       !(i > 0 && p2 && digits[i] == digits[i-1]) {
            same = true;
        }
    }
    same
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = input.trim_end().split('-').map(|s| s.parse().unwrap()).collect::<Vec<u32>>();
    let mut count = 0;
    for i in nums[0]..=nums[1] {
        if check(i, false) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = input.trim_end().split('-').map(|s| s.parse().unwrap()).collect::<Vec<u32>>();
    let mut count = 0;
    for i in nums[0]..=nums[1] {
        if check(i, true) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(check(111111, false), true);
        assert_eq!(check(223450, false), false);
        assert_eq!(check(123789, false), false);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(check(112233, true), true);
        assert_eq!(check(123444, true), false);
        assert_eq!(check(111122, true), true);
    }
}
