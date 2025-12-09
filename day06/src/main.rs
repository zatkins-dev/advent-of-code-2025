use std::fs::File;
use std::io::prelude::*;

struct Problem {
    inputs: Vec<i128>,
    op: fn(i128, i128) -> i128,
}

impl Problem {
    fn solve(&self) -> Option<i128> {
        self.inputs.iter().copied().reduce(|a, b| (self.op)(a, b))
    }
}

fn read_input(filename: &String) -> Result<Vec<Problem>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let mut out = Vec::new();
    let (inputs, ops_str) = buf.trim().rsplit_once('\n').unwrap();
    let input_lists: Vec<Vec<&str>> = inputs
        .split('\n')
        .map(|s| s.split_whitespace().collect())
        .collect();
    let ops: Vec<fn(i128, i128) -> i128> = ops_str
        .split_whitespace()
        .map(|op| {
            if op == "+" {
                return i128::wrapping_add as fn(i128, i128) -> i128;
            } else {
                return i128::wrapping_mul;
            }
        })
        .collect();
    for (i, op) in ops.iter().enumerate() {
        let mut input = Vec::new();
        for l in input_lists[..].iter() {
            input.push(l[i].parse().unwrap());
        }
        out.push(Problem {
            inputs: input,
            op: *op,
        });
    }

    return Ok(out);
}

fn read_input_p2(filename: &String) -> Result<Vec<Problem>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let mut col_width = Vec::new();
    let mut out = Vec::new();
    let (inputs, ops_str) = buf[..buf.len() - 1].rsplit_once('\n').unwrap();
    let mut width = 0;
    for c in ops_str.chars() {
        // print!("{}", if c == ' ' { 'x' } else { c });
        if c != ' ' {
            if width > 0 {
                col_width.push(width);
            }
            width = 1;
        } else {
            width = width + 1;
        }
    }
    println!();
    // col_width.iter().copied().for_each(|w| println!("  {}", w));
    let input_strs: Vec<&str> = inputs.split('\n').collect();
    let mut input_lists: Vec<Vec<&str>> = Vec::new();
    for s in input_strs {
        let mut head = s;
        let mut list = Vec::new();
        for &width in col_width.iter() {
            let (chunk, new_head) = head.split_at(width);
            list.push(chunk);
            head = new_head;
        }
        list.push(head);
        input_lists.push(list);
    }
    let ops: Vec<fn(i128, i128) -> i128> = ops_str
        .split_whitespace()
        .map(|op| {
            if op == "+" {
                return i128::wrapping_add as fn(i128, i128) -> i128;
            } else {
                return i128::wrapping_mul;
            }
        })
        .collect();
    for (i, op) in ops.iter().enumerate() {
        let mut inputs = Vec::new();
        let mut inputs_str: Vec<String> = Vec::new();
        for _ in 0..input_lists[0][i].len() {
            inputs_str.push(String::new());
        }

        for l in input_lists[..].iter() {
            for (j, c) in l[i].chars().rev().enumerate() {
                inputs_str[j] += &c.to_string();
            }
        }
        for input in inputs_str {
            let trimmed = input.trim();
            // println!("{}", input);
            if trimmed.len() == 0 {
                continue;
            };
            inputs.push(trimmed.parse().unwrap());
        }
        out.push(Problem {
            inputs: inputs,
            op: *op,
        });
    }

    return Ok(out);
}

fn main() {
    let filename = String::from("input.txt");
    let problems = read_input(&filename).unwrap();
    let part1: i128 = problems.iter().map(|p| p.solve().unwrap()).sum();

    println!("Part 1: {}", part1);

    let problems2 = read_input_p2(&filename).unwrap();
    let part2: i128 = problems2.iter().map(|p| p.solve().unwrap()).sum();
    println!("Part 2: {}", part2);
}
