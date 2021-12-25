use std::env;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone)]
enum SFNumber {
    Number(i32),
    Pair(Box<SFNumber>, Box<SFNumber>),
}

impl SFNumber {
    pub fn sum(self, other: SFNumber) -> SFNumber {
        SFNumber::Pair(Box::from(self), Box::from(other)).reduce()
    }

    pub fn magnitude(&self) -> i32 {
        match self {
            Self::Number(v) => *v,
            Self::Pair(v1, v2) => 3 * v1.magnitude() + 2 * v2.magnitude(),
        }
    }

    pub fn reduce(self) -> Self {
        let mut number = self;
        let mut split = true;
        while split {
            let mut exploded = true;
            while exploded {
                let (num, _, exp) = number.explode_rec(0);
                number = num;
                exploded = exp;
            }
            let (num, s) = number.split();
            number = num;
            split = s;
        }
        number
    }

    fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    fn is_number_pair(&self) -> bool {
        match self {
            Self::Number(_) => false,
            Self::Pair(v1, v2) => v1.is_number() && v2.is_number(),
        }
    }

    pub fn split(self) -> (Self, bool) {
        match self {
            Self::Number(v) => {
                if v >= 10 {
                    (
                        Self::Pair(
                            Box::from(Self::Number((v as f32 / 2 as f32).floor() as i32)),
                            Box::from(Self::Number((v as f32 / 2 as f32).ceil() as i32)),
                        ),
                        true,
                    )
                } else {
                    (self, false)
                }
            }
            Self::Pair(v1, v2) => {
                let (v1, s1) = v1.split();
                let (v2, s2) = if !s1 { v2.split() } else { (*v2, false) };
                (Self::Pair(Box::from(v1), Box::from(v2)), s1 || s2)
            }
        }
    }

    fn add_first_left(self, amount: i32) -> Self {
        match self {
            Self::Number(v) => Self::Number(v + amount),
            Self::Pair(v1, v2) => Self::Pair(Box::from(v1.add_first_left(amount)), v2),
        }
    }

    fn add_first_right(self, amount: i32) -> Self {
        match self {
            Self::Number(v) => Self::Number(v + amount),
            Self::Pair(v1, v2) => Self::Pair(v1, Box::from(v2.add_first_right(amount))),
        }
    }

    fn explode_rec(self, depth: u32) -> (Self, Option<(i32, i32)>, bool) {
        if self.is_number_pair() && depth >= 4 {
            let exp_val = if let Self::Pair(v1, v2) = &self {
                (
                    if let Self::Number(v) = v1.as_ref() {
                        *v
                    } else {
                        panic!()
                    },
                    if let Self::Number(v) = v2.as_ref() {
                        *v
                    } else {
                        panic!()
                    },
                )
            } else {
                panic!();
            };
            return (Self::Number(0), Some(exp_val), true);
        }
        match self {
            Self::Number(_) => (self, None, false),
            Self::Pair(v1, v2) => {
                let (v1, exp_result, exploded) = v1.explode_rec(depth + 1);
                if exploded {
                    if let Some(r) = exp_result {
                        let outer_exp_result = (r.0, 0);
                        let v2 = if let Self::Number(v) = v2.as_ref() {
                            Box::from(Self::Number(v + r.1))
                        } else {
                            Box::from(v2.add_first_left(r.1))
                        };
                        let r_number = SFNumber::Pair(Box::from(v1), v2);
                        if outer_exp_result == (0, 0) {
                            (r_number, None, exploded)
                        } else {
                            (r_number, Some(outer_exp_result), exploded)
                        }
                    } else {
                        (Self::Pair(Box::from(v1), v2), None, true)
                    }
                } else {
                    let (v2, exp_result, exploded) = v2.explode_rec(depth + 1);
                    if exploded {
                        if let Some(r) = exp_result {
                            let outer_exp_result = (0, r.1);
                            let v1 = if let Self::Number(v) = v1 {
                                Box::from(Self::Number(v + r.0))
                            } else {
                                Box::from(v1.add_first_right(r.0))
                            };
                            let r_number = SFNumber::Pair(v1, Box::from(v2));
                            if outer_exp_result == (0, 0) {
                                (r_number, None, exploded)
                            } else {
                                (r_number, Some(outer_exp_result), exploded)
                            }
                        } else {
                            (Self::Pair(Box::from(v1), Box::from(v2)), None, true)
                        }
                    } else {
                        (Self::Pair(Box::from(v1), Box::from(v2)), None, false)
                    }
                }
            }
        }
    }
}

impl Display for SFNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(v) => {
                write!(f, "{}", v)
            }
            Self::Pair(v1, v2) => {
                write!(f, "[{},{}]", v1, v2)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut numbers = Vec::new();
    for line in lines {
        let mut chars = line.chars();
        let number = parse_sf_number(&mut chars);
        numbers.push(number);
    }

    let mut final_number = numbers[0].clone();

    for number in numbers.iter_mut().skip(1) {
        final_number = final_number.sum(number.clone().reduce());
    }
    println!("{}", final_number);
    println!("{}", final_number.magnitude());

    let mut max = 0;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let num1 = numbers[i].clone();
            let num2 = numbers[j].clone();
            let num12 = num1.clone().sum(num2.clone()).magnitude();
            if num12 > max {
                max = num12;
            }
            let num21 = num2.sum(num1).magnitude();
            if num21 > max {
                max = num21;
            }
        }
    }
    println!("{}", max);
}

fn parse_sf_number(chars: &mut std::str::Chars) -> SFNumber {
    let char = chars.next().unwrap();
    if char.is_digit(10) {
        return SFNumber::Number(char.to_digit(10).unwrap() as i32);
    }
    let number1 = parse_sf_number(chars);
   loop {
        let char = chars.next();
        if char.is_none() {
            panic!();
        }
        if char.unwrap() == ',' {
            break char.unwrap();
        }
    };

    let number2 = parse_sf_number(chars);

    SFNumber::Pair(Box::from(number1), Box::from(number2))
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
