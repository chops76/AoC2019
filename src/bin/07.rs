use advent_of_code::intcode::IntCode;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let program = input.trim_end().split(',')
                                 .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let phase = vec![0, 1, 2, 3, 4];
    let mut best = 0;
    for perm in phase.iter().permutations( phase.len() ) {
        let mut level = 0;
        for i in 0..5 {
            let mut amp = IntCode::new( 1024 );
            amp.set_input(*perm[i]);
            amp.init(program.clone());
            amp.run();
            amp.set_input(level);
            amp.run_until_output();
            level = amp.get_output().unwrap();
        }
        best = best.max(level);
    }
    Some(best)
}

pub fn part_two(input: &str) -> Option<i64> {
    let program = input.trim_end().split(',')
                                 .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let phase = vec![5, 6, 7, 8, 9];
    let mut best = 0;
    for perm in phase.iter().permutations( phase.len() ) {
        let mut amps = Vec::new();
        for i in 0..5 {
            let mut amp = IntCode::new( 1024 );
            amp.set_input(*perm[i]);
            amp.init(program.clone());
            amp.run();
            amps.push(amp);
        }
        let mut level = 0;
        let mut halted = false;
        while !halted {
            for i in 0..5 {
                amps[i].set_input(level);
                amps[i].run_until_output();
                if amps[i].is_finished() {
                    halted = true;
                    break;
                }
                level = amps[i].get_output().unwrap();
            }
        }
        best = best.max(level);
    }
    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }
}
