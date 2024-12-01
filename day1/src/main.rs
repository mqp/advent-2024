use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums = Vec::<usize>::new();
    let mut cts = HashMap::<usize, usize>::new();
    for (l, r) in input.split_whitespace().tuples() {
        nums.push(l.parse()?);
        cts.entry(r.parse()?).and_modify(|ct| *ct += 1).or_insert(1);
    }
    let total: usize = nums.iter().map(|n| n * cts.get(n).unwrap_or(&0)).sum();
    println!("{}", total);
    Ok(())
}
