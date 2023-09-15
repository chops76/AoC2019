fn get_layers(input: &str) -> Vec<Vec<Vec<char>>> {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let mut x = 0;
    let mut y = 0;
    let mut layer = 0;
    let mut layers = Vec::new();
    for c in input.trim_end().chars() {
        if x == 0 {
            if y == 0 {
                layers.push(Vec::new());
            }
            layers[layer].push(Vec::new());
        }
        layers[layer][y].push(c);
        x += 1;
        if x == WIDTH {
            x = 0;
            y += 1;
        }
        if y == HEIGHT {
            y = 0;
            layer += 1;
        }
    }
    layers.reverse();
    layers
}

pub fn part_one(input: &str) -> Option<usize> {
    let layers = get_layers(input);
    let mut best = usize::MAX;
    let mut best_val = 0;
    for l in layers {
        let flat = l.iter().flat_map(|a| a.iter()).collect::<Vec<&char>>();
        let num_zero = flat.iter().filter(|c| ***c == '0').count();
        if num_zero < best {
            best = num_zero;
            let num_one = flat.iter().filter(|c| ***c == '1').count();
            let num_two = flat.iter().filter(|c| ***c == '2').count();
            best_val = num_one * num_two;
        }
    }
    Some(best_val)
}

pub fn part_two(input: &str) -> Option<String> {
    let layers = get_layers(input);
    let mut pic = layers[0].clone();
    for l in layers.iter().skip(1) {
        for y in 0..l.len() {
            for x in 0..l[y].len() {
                if l[y][x] != '2' {
                    pic[y][x] = l[y][x];
                }
            }
        }
    }
    let mut s = String::new();
    for line in pic {
        for c in line {
            if c == '1' {
                s += "* ";
            } else {
                s += "  ";
            }
        }
        s += "\n";
    }
    Some(s)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
