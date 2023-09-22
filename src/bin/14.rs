use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct Chemical {
    amount: u64,
    name: String
}

#[derive(Debug)]
#[derive(Clone)]
struct Recipe {
    result: Chemical,
    ingredients: Vec<Chemical>
}

fn parse_chemical(input: &str) -> Chemical {
    let spl = input.split_ascii_whitespace().collect::<Vec<&str>>();
    Chemical {
        amount: spl[0].parse().unwrap(),
        name: spl[1].to_string()
    }
}

fn parse_recipe(input: &str) -> Recipe {
    let spl = input.split(" => ").collect::<Vec<&str>>();
    let ingredients = spl[0].split(", ").collect::<Vec<&str>>();
    Recipe { 
        result: parse_chemical(spl[spl.len() - 1]),
        ingredients: ingredients[..ingredients.len()].iter().map(|s| parse_chemical(s)).collect::<Vec<Chemical>>()
    }
}

fn calc_depth(name: &str, hm: &HashMap<String, Recipe>) -> u64 {
    if name == "ORE" {
        return 0;
    }

    hm[name].ingredients.iter().map(|i| calc_depth(&i.name, hm)).max().unwrap() + 1
}

fn calc_needed(input: &str, fuel: u64) -> u64 {
    let recipes = input.trim_end().split("\n").map(|s| parse_recipe(s)).collect::<Vec<Recipe>>();
    let mut hm = HashMap::new();
    let mut depths = HashMap::new();
    for r in &recipes {
        hm.insert(r.result.name.clone(), r.clone());
    }
    for k in hm.keys() {
        depths.insert(k.clone(), calc_depth(k, &hm));
    }
    depths.insert("ORE".to_string(), 0);
    let mut needed:HashMap<String, u64> = HashMap::new();
    needed.insert("FUEL".to_string(), fuel);
    while !(needed.len() == 1 && needed.keys().next().unwrap() == "ORE") {
        let mut max_depth = 0;
        let mut best_ingredient = String::new();
        for n in &needed {
            if depths[n.0] > max_depth {
                best_ingredient = n.0.clone();
                max_depth = depths[n.0];
            }
        }
        let amount_needed = needed[&best_ingredient];
        let mut mul = amount_needed / hm[&best_ingredient].result.amount;
        if amount_needed % hm[&best_ingredient].result.amount != 0 {
            mul += 1;
        }
        needed.remove(&best_ingredient);
        for chem in &hm[&best_ingredient].ingredients {
            if !needed.contains_key(&chem.name) {
                needed.insert(chem.name.clone(), chem.amount * mul);
            } else {
                *needed.get_mut(&chem.name).unwrap() += chem.amount * mul;
            }
        }
    }
    needed["ORE"]
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(calc_needed(input, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut high_end = 1000000000000;
    let mut low_end = 1;
    let target = 1000000000000;
    while high_end - low_end != 1 {
        let to_test = (high_end + low_end) / 2;
        let val = calc_needed(input, to_test);
        if val > target {
            high_end = to_test;
        } else {
            low_end = to_test;
        }
    }
    Some(low_end)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
