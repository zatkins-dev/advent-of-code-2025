mod grid;
use crate::grid::Grid;
use std::fs::File;
use std::io::prelude::*;
use value_enum::value_enum;

value_enum!(
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum SpotType: char {
        EMPTY = '.',
        START = 'S',
        SPLITTER = '^',
        BEAM = '|',
    }
);

// type Grid = Vec<Vec<SpotType>>;

fn read_input(filename: &str) -> Result<Grid<SpotType>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let rows: Vec<Vec<SpotType>> = buf
        .trim()
        .split('\n')
        .map(|s| s.chars().map(|c| SpotType::try_from(c).unwrap()).collect())
        .collect();
    let mut grid = Grid::new(rows.len(), rows[0].len());
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            grid[(i, j)] = rows[i][j];
        }
    }
    Ok(grid)
}

fn count_splits(grid: &mut Grid<SpotType>) -> i32 {
    let mut count = 0;

    for i in 0..grid.rows - 1 {
        for j in 0..grid.cols {
            if grid[i + 1][j] == SpotType::SPLITTER {
                if grid[i][j] == SpotType::BEAM || grid[i][j] == SpotType::START {
                    count += 1;
                    if j > 0 && grid[i + 1][j - 1] == SpotType::EMPTY {
                        grid[i + 1][j - 1] = SpotType::BEAM;
                    }
                    if j < grid.cols - 1 && grid[i + 1][j + 1] == SpotType::EMPTY {
                        grid[i + 1][j + 1] = SpotType::BEAM;
                    }
                }
            } else if (grid[i][j] == SpotType::BEAM || grid[i][j] == SpotType::START)
                && grid[i + 1][j] == SpotType::EMPTY
            {
                grid[i + 1][j] = SpotType::BEAM;
            }
        }
    }
    count
}

fn count_quantum_splits(grid: &mut Grid<SpotType>) -> i128 {
    let mut numeric_grid = Grid::new(grid.rows, grid.cols);

    for i in 0..grid.rows - 1 {
        for j in 0..grid.cols {
            if grid[(i + 1, j)] == SpotType::SPLITTER {
                if grid[(i, j)] == SpotType::BEAM || grid[(i, j)] == SpotType::START {
                    if j > 0 {
                        grid[(i + 1, j - 1)] = SpotType::BEAM;
                        numeric_grid[(i + 1, j - 1)] += numeric_grid[(i, j)];
                    }
                    if j < grid.cols - 1 {
                        grid[(i + 1, j + 1)] = SpotType::BEAM;
                        numeric_grid[(i + 1, j + 1)] += numeric_grid[(i, j)];
                    }
                }
            } else if grid[(i + 1, j)] == SpotType::EMPTY || grid[(i + 1, j)] == SpotType::BEAM {
                if grid[(i, j)] == SpotType::START {
                    grid[(i + 1, j)] = SpotType::BEAM;
                    numeric_grid[(i + 1, j)] = 1;
                } else if grid[(i, j)] == SpotType::BEAM {
                    grid[(i + 1, j)] = SpotType::BEAM;
                    numeric_grid[(i + 1, j)] += numeric_grid[(i, j)];
                }
            }
        }
    }

    numeric_grid[grid.rows - 1].iter().sum()
}

fn main() {
    let filename = String::from("input.txt");
    let mut grid = read_input(&filename).unwrap();
    let part1 = count_splits(&mut grid);
    println!("Part 1: {}", part1);
    let mut grid2 = read_input(&filename).unwrap();
    let part2 = count_quantum_splits(&mut grid2);
    println!("Part 2: {}", part2);
}
