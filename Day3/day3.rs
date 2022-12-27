use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    let mut wire1: Vec<String> = Vec::new();
    let mut wire2: Vec<String> = Vec::new();
    let mut wire_num: i8 = 0;

    let mut grid = vec![vec![0i8; 50000]; 50000];
    let mut grid_path1 = vec![vec![0i32; 50000]; 50000];
    let mut grid_path2 = vec![vec![0i32; 50000]; 50000];
    let mut step: i32 = 0;
    let mut lowest_step_count: i32 = 25000;
    let mut direction : char;
    let mut magnitude : i32;
    let mut x_position = 25000;    
    let mut y_position = 25000;    

    let mut x_max = 0;
    let mut y_max = 0;
    let mut x_min = 0;
    let mut y_min = 0;

    let mut shortest_manhattan_distance = 50000;

    grid[x_position][y_position] = 3;

    for line in f.lines(){
        for value in line.unwrap().split(","){
            if wire_num == 0{
                wire1.push(value.to_string());
            }
            if wire_num == 1{
                wire2.push(value.to_string());
            }
        }
        wire_num += 1;
    }

    //record path of first wire
    for instruction in wire1{
        direction = instruction.chars().nth(0).unwrap();
        magnitude = instruction[1..].to_string().parse::<i32>().unwrap();

        if direction == 'U' {
            while magnitude > 0{
                y_position += 1;
                grid[x_position][y_position] += 1;
                step += 1;
                if grid_path1[x_position][y_position] == 0
                {
                    grid_path1[x_position][y_position] += step;
                }
                magnitude -= 1;
                if y_position > y_max {
                    y_max = y_position;
                }
            }
        }
        else if direction == 'D' {
            while magnitude > 0{
                y_position -= 1;
                grid[x_position][y_position] += 1;
                step += 1;
                if grid_path1[x_position][y_position] == 0
                {
                    grid_path1[x_position][y_position] += step;
                }
                magnitude -= 1;
                if y_position < y_min {
                    y_min = y_position;
                }
            }
        }
        else if direction == 'L' {
            while magnitude > 0{
                x_position -= 1;
                grid[x_position][y_position] += 1;
                step += 1;
                if grid_path1[x_position][y_position] == 0
                {
                    grid_path1[x_position][y_position] += step;
                }
                magnitude -= 1;
                if x_position < x_min {
                    x_min = x_position;
                }
            }
        }
        else if direction == 'R' {
            while magnitude > 0{
                x_position += 1;
                grid[x_position][y_position] += 1;
                step += 1;
                if grid_path1[x_position][y_position] == 0
                {
                    grid_path1[x_position][y_position] += step;
                }
                magnitude -= 1;
                if x_position > x_max {
                    x_max = x_position;
                }
            }
        }        
    }

    //reset current position to center for second wire
    x_position = 25000;    
    y_position = 25000; 
    step = 0;

    //record path of second wire
    for instruction in wire2{
        direction = instruction.chars().nth(0).unwrap();
        magnitude = instruction[1..].to_string().parse::<i32>().unwrap();
        
        if direction == 'U' {
            while magnitude > 0{
                y_position += 1;
                grid[x_position][y_position] += 2;
                step += 1;
                if grid_path2[x_position][y_position] == 0
                {
                    grid_path2[x_position][y_position] += step;
                }
                magnitude -= 1;
                if y_position > y_max {
                    y_max = y_position;
                }
            }
        }
        else if direction == 'D' {
            while magnitude > 0{
                y_position -= 1;
                grid[x_position][y_position] += 2;
                step += 1;
                if grid_path2[x_position][y_position] == 0
                {
                    grid_path2[x_position][y_position] += step;
                }
                magnitude -= 1;
                if y_position < y_min {
                    y_min = y_position;
                }
            }
        }
        else if direction == 'L' {
            while magnitude > 0{
                x_position -= 1;
                grid[x_position][y_position] += 2;
                step += 1;
                if grid_path2[x_position][y_position] == 0
                {
                    grid_path2[x_position][y_position] += step;
                }
                magnitude -= 1;
                if x_position < x_min {
                    x_min = x_position;
                }
            }
        }
        else if direction == 'R' {
            while magnitude > 0{
                x_position += 1;
                grid[x_position][y_position] += 2;
                step += 1;
                if grid_path2[x_position][y_position] == 0
                {
                    grid_path2[x_position][y_position] += step;
                }
                magnitude -= 1;
                if x_position > x_max {
                    x_max = x_position;
                }
            }
        }        
    }

    //output a cross of the wires
    x_position = x_min;

    while x_position < x_max {
        y_position = y_min;
        while y_position < y_max {
            if grid[x_position][y_position] == 3 {
                //println!("Cross detected at x_position: {}, y_position: {}",x_position,y_position);
                //println!("Manhattan distance to center is : {}",(25000-x_position as i32).abs() + (25000-y_position as i32).abs());
                if (25000-x_position as i32).abs() + (25000-y_position as i32).abs() < shortest_manhattan_distance{
                    if (25000-x_position as i32).abs() + (25000-y_position as i32).abs() != 0 {
                        shortest_manhattan_distance = (25000-x_position as i32).abs() + (25000-y_position as i32).abs();
                    }
                }
                if grid_path1[x_position][y_position] + grid_path2[x_position][y_position] < lowest_step_count {
                    if grid_path1[x_position][y_position] != 0 && grid_path2[x_position][y_position] != 0 {
                        lowest_step_count = grid_path1[x_position][y_position] + grid_path2[x_position][y_position];
                    }
                }
            }
            y_position += 1;
        }
        x_position += 1;
    }



    println!("Part 1: {}", shortest_manhattan_distance);
    println!("Part 2: {}", lowest_step_count);

}