use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: String) -> Result<Vec<(i128, i128)>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let trimmed = buf.trim();
    let mut out = Vec::new();
    for range in trimmed.split(',') {
        let vals = range.split_once('-').unwrap();
        out.push((
            vals.0.parse::<i128>().unwrap(),
            vals.1.parse::<i128>().unwrap(),
        ));
    }
    return Ok(out);
}

fn find_invalid_ids(ranges: &Vec<(i128, i128)>) -> Vec<i128> {
    let mut out = Vec::new();
    for r in ranges {
        for i in r.0..=r.1 {
            let s = i.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let (first_half, second_half) = s.split_at(s.len() / 2);
            if first_half == second_half {
                out.push(i);
            }
        }
    }
    out
}

fn find_all_invalid_ids(ranges: &Vec<(i128, i128)>) -> Vec<i128> {
    let mut out = Vec::new();
    for r in ranges {
        for i in r.0..=r.1 {
            let s = i.to_string();
            for l in 1..=s.len() / 2 {
                if s.len() % l != 0 {
                    continue;
                }
                let (first_part, rest) = s.split_at(l);
                let mut is_invalid = true;
                for j in 0..rest.len() / l {
                    if *first_part != rest[j * l..j * l + l] {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    out.push(i);
                    break;
                }
            }
        }
    }
    out
}

fn main() {
    let filename = String::from("input.txt");
    let ranges = read_input(filename).unwrap();
    let part1: i128 = find_invalid_ids(&ranges).iter().sum();
    println!("Part 1: {}", part1);
    let part2: i128 = find_all_invalid_ids(&ranges).iter().sum();
    println!("Part 2: {}", part2);
}
