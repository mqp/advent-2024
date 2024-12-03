use std::error::Error;
use std::io::{self, Read};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref R: Regex = Regex::new(r"(do(n't)?\(\)|mul\((?<a>\d+),(?<b>\d+)\))").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut total: usize = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut enabled = true;
    for m in R.captures_iter(&input) {
        match m.get(0).unwrap().as_str() {
            "do()" => { enabled = true; }
            "don't()" => { enabled = false; }
            _ => {
                if enabled {
                    total += m["a"].parse::<usize>()? * m["b"].parse::<usize>()?;
                }

            }
        }
    }
    println!("{}", total);
    Ok(())
}