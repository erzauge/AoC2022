use std::fs;
use itertools::Itertools;





pub fn solve() {
    let input = fs::read_to_string("data/day1.dat").expect("file day1 fehlt");
    let elfs = input.split("\n\n").map(|s|s.lines().map(|x|x.parse::<u32>()).collect()).collect::<Result<Vec<Vec<u32>>,_>>().unwrap();
    let max = elfs.iter().map(|x|x.iter().sum::<u32>()).max().unwrap();
    println!("a: {}",max);
    let max3 = elfs.iter().map(|x|x.iter().sum::<u32>()).sorted().rev().take(3).sum::<u32>();
    println!("b: {}",max3);
}