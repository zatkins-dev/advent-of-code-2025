use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

type Bank = Vec<u32>;

fn read_input(filename: String) -> Result<Vec<Bank>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf
        .trim()
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect())
}

fn cmp_ordered((i, x): &(usize, &u32), (j, y): &(usize, &u32)) -> Ordering {
    if (*x).cmp(*y) == Ordering::Equal {
        j.cmp(i)
    } else {
        (*x).cmp(*y)
    }
}

fn max_joltage(bank: &Bank) -> u32 {
    let (max_i, max_x) = bank.iter().enumerate().max_by(cmp_ordered).unwrap();

    if max_i == bank.len() - 1 && bank.iter().sum::<u32>() == *max_x {
        return *max_x;
    } else if max_i == bank.len() - 1 {
        let (_max_j, max_y) = bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .max_by(cmp_ordered)
            .unwrap();
        let val = (max_y.to_string() + &max_x.to_string())
            .parse::<u32>()
            .unwrap();
        println!("{}", val);
        return val;
    }
    let (_max_j, max_y) = bank[max_i + 1..]
        .iter()
        .enumerate()
        .max_by(cmp_ordered)
        .unwrap();
    let val = (max_x.to_string() + &max_y.to_string())
        .parse::<u32>()
        .unwrap();
    // println!("{}", val);
    return val;
    // let largest = bank
    //     .iter()
    //     .enumerate()
    //     .k_largest_by(2, |(_i, x), (_j, y)| x.cmp(y));
    // let val = largest
    //     .sorted_by(|(i, _x), (j, _y)| i.cmp(j))
    //     .fold(String::new(), |acc, (_i, x)| acc + &x.to_string())
    //     .parse::<u32>()
    //     .unwrap();
    // println!("{}", val);
    // val
}

fn max_joltage_k(bank: &Bank, k: u16) -> u128 {
    let (max_i, max_x) = bank[..=bank.len() - usize::from(k)]
        .iter()
        .enumerate()
        .max_by(cmp_ordered)
        .unwrap();
    let mut s = max_x.to_string();
    let mut prev_max_i = max_i;
    for i in 1..=k - 1 {
        let (max_i, max_x) = bank[prev_max_i + 1..=bank.len() - usize::from(k - i)]
            .iter()
            .enumerate()
            .max_by(cmp_ordered)
            .unwrap();
        s += &max_x.to_string();
        prev_max_i += max_i + 1;
    }
    let val = s.parse().unwrap();
    // println!("{}", val);
    return val;
    // let largest = bank
    //     .iter()
    //     .enumerate()
    //     .k_largest_by(2, |(_i, x), (_j, y)| x.cmp(y));
    // let val = largest
    //     .sorted_by(|(i, _x), (j, _y)| i.cmp(j))
    //     .fold(String::new(), |acc, (_i, x)| acc + &x.to_string())
    //     .parse::<u32>()
    //     .unwrap();
    // println!("{}", val);
    // val
}

fn main() {
    let filename = String::from("input.txt");
    let banks = read_input(filename).unwrap();
    let part1: u32 = banks.iter().map(max_joltage).sum();
    println!("Part 1: {}", part1);

    let part1b: u128 = banks.iter().map(|b| max_joltage_k(b, 2)).sum();
    println!("Part 1b: {}", part1b);
    let part2: u128 = banks.iter().map(|b| max_joltage_k(b, 12)).sum();
    println!("Part 2: {}", part2);
}
