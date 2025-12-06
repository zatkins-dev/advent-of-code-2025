use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


fn read_input(filename: String) -> Result<Vec<i32>, std::io::Error> {
    let f = File::open(filename)?;
    let mut input = BufReader::new(f);
    let mut buf = String::new();
    let mut out = Vec::<i32>::new();

    while input.read_line(&mut buf).is_ok() && buf.len() > 1 {
        let val = &buf[1..buf.len()-1];
        let sign = if buf.starts_with("L") {-1} else {1};
        out.push(sign * val.parse::<i32>().unwrap());
        buf.clear()
    }
    return Ok(out);
}


fn rotate_count_zeros(rotations: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut position: i32 = 50;

    for r in rotations {
        position = (position + r + 100) % 100;
        if position == 0 { count += 1; }
    }
    count
}

fn rotate_count_zero_crossings(rotations: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut position: i32 = 50;

    for r in rotations {
        let raw = position + r;
        let m = (raw / 100).abs() + i32::from(raw <= 0 && position != 0);
        // print!("position: {}, ",position);
        position = (raw % 100 + 100) % 100;
        // print!("rotation: {}, new position: {}, m: {}\n", r, position, m);
        count += m;
    }
    count
}

fn main() {
    let filename = String::from("input.txt");
    let rotations = read_input(filename).unwrap();
    println!("Part 1: {}", rotate_count_zeros(&rotations));
    println!("Part 2: {}", rotate_count_zero_crossings(&rotations));
}
