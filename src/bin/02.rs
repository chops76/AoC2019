use advent_of_code::intcode::IntCode;

fn run_program(data: Vec<i64>) -> i64 {
    let mut vm = IntCode::new( 1024 );
    vm.init(data);
    vm.run();
    vm.get_mem(0)
}

pub fn part_one(input: &str) -> Option<i64> { 
    let mut data = input.trim_end().split(',')
                                  .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    data[1] = 12;
    data[2] = 2;
    Some(run_program(data))  
}

pub fn part_two(input: &str) -> Option<i64> {
    let data = input.trim_end().split(',')
                              .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut test_data = data.clone();
            test_data[1] = noun;
            test_data[2] = verb;
            if run_program(test_data) == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

