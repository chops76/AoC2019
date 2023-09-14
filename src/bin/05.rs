use advent_of_code::intcode::IntCode;

fn run_program(data: Vec<i64>, input_val: i64) -> i64 {
    let mut vm = IntCode::new( 1024 );
    let mut ret_val = 0;
    vm.init(data);
    vm.set_input(input_val);
    while !vm.is_finished() {
        vm.tick();
        if let Some(output) = vm.get_output() {
            ret_val = output;
        }
    }
    ret_val
}

pub fn part_one(input: &str) -> Option<i64> { 
    let data = input.trim_end().split(',')
                                  .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    Some(run_program(data, 1))  
}

pub fn part_two(input: &str) -> Option<i64> { 
    let data = input.trim_end().split(',')
                                  .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    Some(run_program(data, 5))  
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

