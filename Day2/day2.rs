use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut v: Vec<i32> = Vec::new();
    for line in f.lines(){
        for value in line.unwrap().split(","){
            v.push(value.parse::<i32>().unwrap());
            //println!("{}",value.parse::<i32>().unwrap());
        }
    }

    //replace postition 1 with 12 and position 2 with 2 per instructions
    v[1] = 12;
    v[2] = 2;

    let length : usize = v.len();
    let mut index : usize = 0;
    while index < length {
        //println!("{}",index);
        let operation : i32 = v[index];
        let num1 : i32 = v[index+1];
        let num2 : i32 = v[index+2];
        let output : i32 = v[index+3];
        
        if operation == 99
        {
            break;
        }
        else if operation == 1
        {
            v[output as usize] = num1 + num2;
        }
        else if operation == 2
        {
            v[output as usize] = num1 * num2;
        }
        
        index = index + 4;
    }
    println!("Part 1: {}",v[0]);
}

fn main(){
    part1();
}