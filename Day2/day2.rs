use std::fs::File;
use std::io::{BufRead, BufReader};

fn do_stuff(noun: i32,verb: i32) -> i32 {
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
    v[1] = noun;
    v[2] = verb;

    let length : usize = v.len();
    let mut index : usize = 0;
    while index < length {
        //println!("{}",index);
        let operation : i32 = v[index];
        let num1index : i32 = v[index+1];
        let num2index : i32 = v[index+2];
        let output : i32 = v[index+3];

        let num1: i32 = v[num1index as usize];
        let num2: i32 = v[num2index as usize];
        
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
    v[0]
}

fn main(){
    let mut answer = do_stuff(12,2);
    println!("Part 1: {:?}", answer);

    for i in 0..100
    {
        for j in 0..100
        {
            answer = do_stuff(i,j);
            if answer == 19690720
            {
                println!("Part 2. Noun is {} and verb is {}",i,j);
                println!("{}",100*i+j);
            }
        }
    }
}