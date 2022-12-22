use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_fuel(amount:f32) -> f32
{
    let mut instruction = amount / 3.0;
    instruction = instruction.floor() - 2.0;
    return instruction;
}

fn part1() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    //PART 1
    let mut sum : f32 = 0.0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let instruction = line.parse::<f32>().unwrap();
        let module = calculate_fuel(instruction);
        sum += module;
    }
    println!("{}", sum);
}

fn part2() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    //PART 1
    let mut sum : f32 = 0.0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut module = line.parse::<f32>().unwrap();
        while module > 0.0
        {
            module = calculate_fuel(module);
            if module > 0.0
            {
                sum += module;
            }
        }
    }
    println!("{}", sum);
}

fn main()
{
    part1();
    part2();
}