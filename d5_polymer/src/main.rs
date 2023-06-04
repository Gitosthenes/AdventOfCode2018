use std::{fs, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let reactions = get_reactions();
    
    part1(&input, &reactions)?;
    part2(&input, &reactions)?;

    Ok(())
}

fn part1(input: &str, reactions: &Vec<String>) -> Result<usize> {
    let mut input = input.clone().to_owned();
    let mut reaction_found = true;

    while reaction_found {
        reaction_found = false;

        for pair in reactions {
            match input.find(pair) {
                Some(_) => {
                    reaction_found = true;
                    input = input.replace(pair, "");
                },
                None => continue,
            }
        }
    }

    // println!(input.len());

    Ok(input.len())
}

fn part2(input: &str, reactions: &Vec<String>) -> Result<()> {
    let mut min: usize = usize::MAX;

    for lower in "abcdefghijklmnopqrstuvwxyz".chars() {
        let upper = lower.to_uppercase().collect::<String>();
        let copy = input.clone().replace(lower, "").replace(&upper, "");

        let reacted_size = part1(&copy, reactions)?;

        min = if reacted_size < min { reacted_size } else { min }
    }

    println!("{}", min);

    Ok(())
}

fn get_reactions() -> Vec<String> {
    let mut reactions: Vec<String> = vec![];

    for lower in "abcdefghijklmnopqrstuvwxyz".chars() {
        let upper = lower.to_uppercase().collect::<String>();

        reactions.push(format!("{}{}", lower, upper));
        reactions.push(format!("{}{}", upper, lower));
    }

    reactions
}