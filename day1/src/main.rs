use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums = Vec::<i32>::new();
    for num in input.split_whitespace() {
        nums.push(num.parse()?);
    }
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for i in 0..(nums.len() / 2) {
        let l = nums[2 * i];
        let r = nums[2 * i + 1];
        left.push(l);
        right.push(r);
    }
    let mut counts = HashMap::new();
    for n in right {
        counts.entry(n).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut total = 0;
    for n in left {
        let count = counts.get(&n).unwrap_or_else(|| &0);
        total += count * n;
    }
    println!("{}", total);
    Ok(())
}
