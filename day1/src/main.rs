use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn parse_pair(line: &str) -> Result<(usize, usize), Box<dyn Error>> {
    if let Some((l, r)) = line.split_whitespace().collect_tuple() {
        Ok((l.parse()?, r.parse()?))
    } else {
        Err(format!("Failed to parse line: {}", line).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut nums = Vec::<usize>::new();
    let mut cts = HashMap::<usize, usize>::new();
    for line in io::stdin().lines() {
        let (l, r) = parse_pair(&line?)?;
        nums.push(l);
        cts.entry(r).and_modify(|ct| *ct += 1).or_insert(1);
    }
    let total: usize = nums.iter().map(|n| n * cts.get(n).unwrap_or(&0)).sum();
    println!("{}", total);
    Ok(())
}
