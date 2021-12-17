use bitvec::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::ops::BitAnd;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let input = std::fs::read_to_string(&args[0]).unwrap();

    let split = input.rsplitn(3, " ").collect::<Vec<&str>>();
    let y_area = split[0].rsplit("=").collect::<Vec<&str>>()[0]
        .split("..")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let x_area = split[1].replace(",", "").rsplit("=").collect::<Vec<&str>>()[0]
        .split("..")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let (y_min, y_max) = (*y_area.iter().min().unwrap(), *y_area.iter().max().unwrap());
    let (x_min, x_max) = (*x_area.iter().min().unwrap(), *x_area.iter().max().unwrap());
    println!("[{},{}]", x_min, x_max);
    println!("[{},{}]", y_min, y_max);
    let mut max_x_vel = x_max;
    let mut current_max_y_vel = y_min;

    let mut pos = (0, 0);
    let mut velocity = (6, 9);
    let mut velocities = Vec::new();
    for j in current_max_y_vel..10000 {
        for i in 0..max_x_vel+1 {
            let initial_velocity = (i, j);
            velocity = initial_velocity;
            pos = (0, 0);
            while pos.0 <= x_max
                && (pos.0 >= x_min || velocity.0 > 0)
                && (pos.1 >= y_min || velocity.1 > 0)
            {
                pos = (pos.0 + velocity.0, pos.1 + velocity.1);
                if velocity.0 > 0 {
                    velocity.0 -= 1;
                }
                velocity.1 -= 1;
                if pos.0 >= x_min && pos.0 <= x_max && pos.1 >= y_min && pos.1 <= y_max {
                    println!("{:?}", initial_velocity);
                    velocities.push(initial_velocity);
                    break;
                }
            }
        }
    }
    println!("{}", (current_max_y_vel * (current_max_y_vel + 1))/2 );
    println!("{}", velocities.len() );
}
