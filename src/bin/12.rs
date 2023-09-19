use num::Integer;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let spl = input.split(&['=', ',', '>'][..]).collect::<Vec<&str>>();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for i in 0..4 {
        x.push(spl[6*i + 1].parse().unwrap());
        y.push(spl[6*i + 3].parse().unwrap());
        z.push(spl[6*i + 5].parse().unwrap());
    }
    (x, y, z)
}

fn time_step(pos: &mut Vec<i32>, vel: &mut Vec<i32>) {
    for i in 0..pos.len() - 1 {
        for j in i+1..pos.len() {
            if pos[i] > pos[j] {
                vel[j] += 1;
                vel[i] -= 1;
            } else if pos[j] > pos[i] {
                vel[i] += 1;
                vel[j] -= 1;
            }
        }
    }
    for i in 0..pos.len() {
        pos[i] += vel[i];
    }
}

fn find_cycle(orig_pos: Vec<i32>) -> u64 {
    let mut pos = orig_pos.clone();
    let orig_vel = vec![0, 0, 0, 0];
    let mut vel = orig_vel.clone();
    let mut count = 1;
    time_step(&mut pos, &mut vel);
    while !(pos == orig_pos && vel == orig_vel) {
        count += 1;
        time_step(&mut pos, &mut vel);       
    }
    count
}

fn find_energy(input: &str, steps: usize) -> Option<i32> {
    let (mut x_pos, mut y_pos, mut z_pos) = parse_input(input);
    let mut x_vel = vec![0, 0, 0, 0];
    let mut y_vel = vec![0, 0, 0, 0];
    let mut z_vel = vec![0, 0, 0, 0];
    for _ in 0..steps {
        time_step(&mut x_pos, &mut x_vel);
        time_step(&mut y_pos, &mut y_vel);
        time_step(&mut z_pos, &mut z_vel);
    }
    let mut total = 0;
    for i in 0..4 {
        total += (x_pos[i].abs() + y_pos[i].abs() + z_pos[i].abs()) * 
                 (x_vel[i].abs() + y_vel[i].abs() + z_vel[i].abs());
    }
    Some(total)
}

pub fn part_one(input: &str) -> Option<i32> {
    find_energy(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (x_pos, y_pos, z_pos) = parse_input(input);
    let x_cycle = find_cycle(x_pos);
    let y_cycle = find_cycle(y_pos);
    let z_cycle = find_cycle(z_pos);
    let mut lcm = x_cycle.lcm(&y_cycle);
    lcm = lcm.lcm(&z_cycle);
    Some(lcm)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(find_energy(&input, 100), Some(1940));
    }
}
