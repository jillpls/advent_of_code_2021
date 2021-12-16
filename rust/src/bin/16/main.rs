use bitvec::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::ops::BitAnd;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let input = std::fs::read_to_string(&args[0]).unwrap();
    let mut bytes = Vec::new();
    for i in (0..input.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&input[i..i + 2], 16).unwrap());
    }
    let bits = BitVec::<Msb0, _>::from_vec(bytes);
    let (val, _, s) = get_packet(&bits);
    println!("{}", s);
    println!("{}", val);
}

fn get_packet(input: &BitSlice<Msb0, u8>) -> (u64, u64, u64) {
    println!();
    println!("{}", input);
    let version: u8 = (&input[0..3]).load_be::<u8>();
    let type_id: u8 = (&input[3..6]).load_be::<u8>();
    if type_id == 4 {
        let mut slice = &input[6..11];
        let mut slice_idx = 6;
        let mut bit_num = BitVec::<Msb0, u64>::new();
        while slice[0] {
            bit_num.extend(&slice[1..]);
            slice_idx += 5;
            if slice_idx + 5 >= input.len() {
                break;
            }
            slice = &input[slice_idx..slice_idx + 5];
        }
        if slice_idx + 5 <= input.len() {
            bit_num.extend(&input[slice_idx + 1..slice_idx + 5]);
        }
        return (
            bit_num.load_be::<u64>(),
            (slice_idx + 5) as u64,
            version as u64,
        );
    }

    let mut version_sum = version as u64;

    let mut sub_results = Vec::new();

    let pos = if input[6] {
        let num_packets = input[7..18].load_be::<u64>();
        let mut pos = 18;
        for _ in 0..num_packets {
            let (val, new_pos, vs) = get_packet(&input[pos..]);
            sub_results.push(val);
            version_sum += vs;
            println!("{}, {}", val, new_pos);
            pos += new_pos as usize;
        }
        pos
    } else {
        let sub_packet_length = input[7..22].load_be::<u64>();
        println!("{}", &input[7..22]);
        println!("{}", sub_packet_length);
        let sub_packet_max = sub_packet_length + 22;
        let mut pos = 22;
        while (pos as u64) < sub_packet_max {
            let (val, new_pos, vs) = get_packet(&input[pos..(sub_packet_max as usize)]);
            sub_results.push(val);
            version_sum += vs;
            println!("{}, {}", val, new_pos);
            pos += new_pos as usize;
        }
        pos
    };

    let val = match type_id {
        0 => sub_results.iter().sum::<u64>(),
        1 => sub_results.iter().product::<u64>(),
        2 => *sub_results.iter().min().unwrap(),
        3 => *sub_results.iter().max().unwrap(),
        5 => {
            if sub_results.len() != 2 {
                panic!("Invalid Input");
            }
            if sub_results[0] > sub_results[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_results.len() != 2 {
                panic!("Invalid Input");
            }
            if sub_results[0] < sub_results[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_results.len() != 2 {
                panic!("Invalid Input");
            }
            if sub_results[0] == sub_results[1] {
                1
            } else {
                0
            }
        }
        _ => 0,
    };

    (val, pos as u64, version_sum)
}
