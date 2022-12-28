use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main(){
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    let mut min: i32 = 0;
    let mut max: i32 = 0;
    let mut index: i32 = 0;
    let mut current_num: i32;
    let mut digit1: i32;
    let mut digit2: i32;
    let mut digit3: i32;
    let mut counter_part1 = 0;
    let mut counter_part2 = 0;
    let mut is_increasing: bool;
    let mut is_double_adjacent_only: bool;
    let mut is_double_adjacent: bool;
    let mut previous_digit: i32;
    let mut position: i8;

    for line in f.lines(){
        for value in line.unwrap().split("-"){
            if index == 0 {
                min = FromStr::from_str(value).unwrap();
            }
            if index == 1 {
                max = FromStr::from_str(value).unwrap();
            }
            index += 1;
        }
    }

    let mut index = min;
    while index <= max {
        current_num = index;
        is_double_adjacent_only = false;
        is_double_adjacent = false;
        is_increasing = true;
        previous_digit = current_num % 10;
        position = 0;
        while current_num > 10 {
            position += 1;
            digit1 = current_num % 10;
            digit2 = current_num / 10;
            digit2 = digit2 % 10;
            digit3 = current_num / 100;
            digit3 = digit3 % 10;
            current_num = current_num / 10;  
            
            //part1
            if digit1 == digit2 {
                is_double_adjacent = true;
            }
            
            //part2
            if (digit1 == digit2) && (digit1 != digit3) {
                if position == 1 {
                    is_double_adjacent_only = true;
                }
                else if digit2 != previous_digit {
                    is_double_adjacent_only = true;
                }
            }

            if digit2 > digit1{
                is_increasing = false;
            }

            previous_digit = digit1;
        }
        if is_double_adjacent && is_increasing {
            counter_part1 += 1;
        }
        if is_double_adjacent_only && is_increasing {
            counter_part2 += 1;
        }
        index += 1;
    }

    println!("Part 1: {}", counter_part1); //not 174, not 163 or 263
    println!("Part 2: {}", counter_part2); //not 174, not 163 or 263
}