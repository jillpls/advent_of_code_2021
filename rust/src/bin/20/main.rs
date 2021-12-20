use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let mut lines = read_lines(&args[0]).unwrap().map(|x| x.unwrap());
    let algorithm = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let rows = lines.collect::<Vec<String>>();
    let padding = 200;
    let empty_img = vec![vec![0;rows[0].len()+2*padding];rows.len()+2*padding];
    let mut img : Vec<Vec<u8>> = empty_img.clone();
    for (i, r) in rows.iter().enumerate() {
        for (j,c) in r.chars().enumerate() {
            match c {
                '.' => { img[i+padding][j+padding] = 0 },
                '#' => { img[i+padding][j+padding] = 1 },
                _ => { panic!() }
            }
        }
    }
    // print_img(&img);
    let mut result_img = img;
    let mut l = 0;
    for _ in 0..50 {
        let x  = enhance(&result_img, &algorithm, padding);
        result_img = x.0;
        l = x.1;
    }
    print_img(&result_img);
    println!("{}", l);
}

fn enhance(img: &Vec<Vec<u8>>, algorithm : &Vec<char>, padding : usize) -> (Vec<Vec<u8>>, u32) {
    let empty_pixel =
        match algorithm[
            {
                if img[0][0] == 0 {
                    0
                } else {
                    254
                }
            }
    ] {
            '.' => { 0 },
            '#' => {
                1 },
            _ => { panic!() }
        };
    let mut result_img= vec![vec![empty_pixel;img[0].len()];img.len()];
    let mut light_pixels = 0;

    for i in padding/2..img.len()-padding/2 {
        for j in padding/2..img[0].len()-padding/2 {
            let r1 = [img[i-1][j-1],img[i-1][j],img[i-1][j+1]];
            let r2 = [img[i][j-1],img[i][j],img[i][j+1]];
            let r3 = [img[i+1][j-1],img[i+1][j],img[i+1][j+1]];
            let idx = (r3[2] + r3[1] * 2 + r3[0] * 4 + r2[2] * 8 + r2[1] * 16 + r2[0] * 32 + r1[2] * 64 + r1[1] * 128) as u32  + (r1[0] as u32 * 256);
            match algorithm[idx as usize] {
                '.' => { result_img[i][j] = 0 },
                '#' => {
                    light_pixels += 1;
                    result_img[i][j] = 1 },
                _ => { panic!() }
            }
        }
    }
    (result_img, light_pixels)
}

fn print_img(img : &Vec<Vec<u8>>) {
    for r in img {
        let s = r.iter().map(|x|  {
            if *x == 0 {
                '.' // ' '
            } else {
                '#' // '█'
            }
        }).collect::<String>();
        println!("{}", s);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
