use advent_of_code::intcode::IntCode;

pub fn part_one(input: &str) -> Option<i64> {
    let program = input.trim_end().split(',')
                                 .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut vm = IntCode::new(1024);
    vm.init(program);
    vm.set_input(1);
    let mut output = 0;
    loop {
        vm.run_until_output();
        if vm.is_finished() {
            break;
        }
        output = vm.get_output().unwrap();
    }
    Some(output)
}

pub fn part_two(input: &str) -> Option<i64> {
    let program = input.trim_end().split(',')
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut vm = IntCode::new(1024);
    vm.init(program);
    vm.set_input(2);
    let mut output = 0;
    loop {
        vm.run_until_output();
        if vm.is_finished() {
            break;
        }
        output = vm.get_output().unwrap();
    }
    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
