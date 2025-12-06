use std::fs::File;
use std::io::prelude::*;
use value_enum::value_enum;

value_enum!(
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum SpotType: char {
        EMPTY = '.',
        FULL = '@',
    }
);

type Grid = Vec<Vec<SpotType>>;

fn read_input(filename: String) -> Result<Grid, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf
        .trim()
        .split('\n')
        .map(|s| s.chars().map(|c| SpotType::try_from(c).unwrap()).collect())
        .collect())
}

fn count_empty(grid: &Grid, i: i32, j: i32) -> u32 {
    let rows = i32::try_from(grid.len()).unwrap();
    let cols = i32::try_from(grid.get(0).unwrap().len()).unwrap();
    let mut count = 0;

    for ii in i - 1..=i + 1 {
        if ii < 0 || ii >= rows {
            continue;
        }
        for jj in j - 1..=j + 1 {
            if jj < 0 || jj >= cols || (ii == i && jj == j) {
                continue;
            } else if unsafe {
                *grid
                    .get_unchecked(usize::try_from(ii).unwrap())
                    .get_unchecked(usize::try_from(jj).unwrap())
                    == SpotType::FULL
            } {
                count += 1;
            }
        }
    }
    count
}

fn count_all_empty(grid: &Grid) -> u32 {
    let rows = i32::try_from(grid.len()).unwrap();
    let cols = i32::try_from(grid.get(0).unwrap().len()).unwrap();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            let ii = usize::try_from(i).unwrap();
            let jj = usize::try_from(j).unwrap();
            if grid[ii][jj] == SpotType::FULL && count_empty(grid, i, j) < 4 {
                count += 1;
                print!("x");
            } else {
                print!("{}", char::from(grid[ii][jj]));
            }
        }
        print!("\n");
    }
    count
}

fn count_all_empty_mut(grid: &mut Grid) -> u32 {
    let rows = i32::try_from(grid.len()).unwrap();
    let cols = i32::try_from(grid.get(0).unwrap().len()).unwrap();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            let ii = usize::try_from(i).unwrap();
            let jj = usize::try_from(j).unwrap();
            if grid[ii][jj] == SpotType::FULL && count_empty(grid, i, j) < 4 {
                count += 1;
                grid[ii][jj] = SpotType::EMPTY;
                print!("x");
            } else {
                print!("{}", char::from(grid[ii][jj]));
            }
        }
        print!("\n");
    }
    count
}

fn count_all_empty_recurse(grid: &mut Grid) -> u32 {
    let mut prev_count = 1;
    let mut count = 0;
    let mut round = 0;

    while prev_count > 0 {
        print!("\n");
        prev_count = count_all_empty_mut(grid);
        count += prev_count;
        round += 1;
        println!(
            "Round {}: new removed: {}, total: {}",
            round, prev_count, count
        );
    }
    count
}

fn main() {
    let filename = String::from("input.txt");
    let mut grid = read_input(filename).unwrap();
    let part1: u32 = count_all_empty(&grid);
    println!("Part 1: {}", part1);
    let part2: u32 = count_all_empty_recurse(&mut grid);
    println!("Part 2: {}", part2);

    // let part1b: u128 = banks.iter().map(|b| max_joltage_k(b, 2)).sum();
    // println!("Part 1b: {}", part1b);
    // let part2: u128 = banks.iter().map(|b| max_joltage_k(b, 12)).sum();
    // println!("Part 2: {}", part2);
}
