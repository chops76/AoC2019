pub fn part_one(input: &str) -> Option<String> {
    let mut arr = input.trim_end().chars().map(|c| c.to_digit(10).unwrap() as i32)
                                 .collect::<Vec<i32>>();
    let arr_size = arr.len();
    for _ in 1..=100 {
        let mut new_arr: Vec<i32> = Vec::new();
        for i in 1..=arr_size {
            let repeated = [0,1,0,-1].iter()
                                            .flat_map(|n| std::iter::repeat(n).take(i))
                                            .cycle().skip(1);
            new_arr.push(arr.iter().zip(repeated).map(|(a,b)| (a * b)).sum::<i32>().abs() % 10);
        }
        arr = new_arr;
    }
    Some(arr.iter().take(8).map(|d| char::from_digit(*d as u32, 10).unwrap()).collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let offset = input[0..7].parse::<usize>().unwrap();
    let mut arr = input.trim_end().chars().map(|c| c.to_digit(10).unwrap() as i32)
        .cycle().take(input.trim_end().len() * 10000).skip(offset).collect::<Vec<i32>>();
    for _ in 0..100 {
        for i in (0..(arr.len() - 1)).rev() {
            arr[i] += arr[i+1];
            arr[i] %= 10;
        }
    }
    Some(arr.iter().take(8).map(|d| char::from_digit(*d as u32, 10).unwrap()).collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

