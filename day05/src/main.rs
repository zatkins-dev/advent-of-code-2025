use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: String) -> Result<(Vec<(i128, i128)>, Vec<i128>), std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let mut fresh_ranges = Vec::new();
    let mut out_ids = Vec::new();
    let (ranges, ids) = buf.trim().split_once("\n\n").unwrap();

    for range in ranges.trim().split('\n') {
        let vals = range.split_once('-').unwrap();
        fresh_ranges.push((vals.0.parse().unwrap(), vals.1.parse().unwrap()));
    }

    fresh_ranges.sort_by(|a: &(i128, i128), b: &(i128, i128)| (&a.0).cmp(&b.0));

    for id in ids.split('\n') {
        out_ids.push(id.parse().unwrap());
    }
    return Ok((fresh_ranges, out_ids));
}

fn is_fresh(id: i128, ranges: &Vec<(i128, i128)>) -> bool {
    for r in ranges {
        if id < r.0 {
            return false;
        }
        if id <= r.1 {
            return true;
        }
    }
    return false;
}

fn simplify_ranges(ranges: &Vec<(i128, i128)>) -> Vec<(i128, i128)> {
    let mut out = Vec::new();
    let mut head = ranges[0].0;
    let mut tail = ranges[0].1;
    for (start, end) in ranges[1..].iter() {
        if *start <= tail && *end > tail {
            tail = *end;
        } else if *start > tail {
            // push and reset
            out.push((head, tail));
            head = *start;
            tail = *end;
        }
    }
    out.push((head, tail));
    out
}

fn main() {
    let filename = String::from("input.txt");
    let (fresh_ranges, available) = read_input(filename).unwrap();
    println!("Number of ranges: {}", fresh_ranges.len());
    fresh_ranges
        .iter()
        .for_each(|(start, stop)| println!("  {}-{}", start, stop));
    let simple_fresh_ranges = simplify_ranges(&fresh_ranges);
    println!("Number of simplified ranges: {}", simple_fresh_ranges.len());
    simple_fresh_ranges
        .iter()
        .for_each(|(start, stop)| println!("  {}-{}", start, stop));
    println!("IDs to check: {}", available.len());
    let part1: i128 = available
        .iter()
        .map(|id| i128::from(is_fresh(*id, &simple_fresh_ranges)))
        .sum();
    println!("Part 1: {}", part1);
    let part2: i128 = simple_fresh_ranges
        .iter()
        .map(|(start, stop)| stop - start + 1)
        .sum();
    println!("Part 2: {}", part2);
}
