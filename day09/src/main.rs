// mod grid;
// use crate::grid::Grid;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use value_enum::value_enum;

value_enum!(
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum SpotType: char {
        EMPTY = '.',
        RED = '#',
        GREEN = 'X',
    }
);

impl std::fmt::Display for SpotType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

fn read_input(filename: &str) -> Result<Vec<[isize; 2]>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let out: Vec<[isize; 2]> = buf
        .trim()
        .split('\n')
        .map(|s| s.split_once(',').unwrap())
        .map(|(x, y)| [x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()])
        .collect();
    Ok(out)
}

fn max_rectangle(points: &Vec<[isize; 2]>) -> usize {
    points
        .iter()
        .combinations(2)
        .map(|pair| (pair[0][0].abs_diff(pair[1][0]) + 1) * (pair[0][1].abs_diff(pair[1][1]) + 1))
        .max()
        .unwrap()
}

// fn draw_line(grid: &mut Grid<SpotType>, &[x1, y1]: &[isize; 2], &[x2, y2]: &[isize; 2]) {
//     if x1.abs_diff(x2) == 0 {
//         let y_max = y1.max(y2);
//         let y_min = y1.min(y2);
//         (y_min + 1..y_max).for_each(|i| {
//             grid[(x1, i)] = match grid[(x1, i)] {
//                 SpotType::EMPTY => SpotType::GREEN,
//                 SpotType::GREEN => SpotType::GREEN,
//                 SpotType::RED => SpotType::RED,
//             };
//         });
//         grid[(x1, y1)] = SpotType::RED;
//         grid[(x2, y2)] = SpotType::RED;
//     } else if y1.abs_diff(y2) == 0 {
//         let x_max = x1.max(x2);
//         let x_min = x1.min(x2);
//         (x_min + 1..x_max).for_each(|i| {
//             grid[(i, y1)] = match grid[(i, y1)] {
//                 SpotType::EMPTY => SpotType::GREEN,
//                 SpotType::GREEN => SpotType::GREEN,
//                 SpotType::RED => SpotType::RED,
//             };
//         });
//     }
//     grid[(x1, y1)] = SpotType::RED;
//     grid[(x2, y2)] = SpotType::RED;
// }

// fn build_grid(points: &Vec<[isize; 2]>) -> Grid<SpotType> {
//     let (min_x, max_x) = match points.iter().minmax_by(|[x1, _], [x2, _]| x1.cmp(x2)) {
//         itertools::MinMaxResult::NoElements => (0isize, 1isize),
//         itertools::MinMaxResult::OneElement(_) => (0isize, 1isize),
//         itertools::MinMaxResult::MinMax(&[x1, _], &[x2, _]) => (x1, x2),
//     };
//     let (min_y, max_y) = match points.iter().minmax_by(|[_, y1], [_, y2]| y1.cmp(y2)) {
//         itertools::MinMaxResult::NoElements => (0isize, 1isize),
//         itertools::MinMaxResult::OneElement(_) => (0isize, 1isize),
//         itertools::MinMaxResult::MinMax(&[_, y1], &[_, y2]) => (y1, y2),
//     };
//     let mut grid = Grid::new_with_bounds(min_x, min_y, max_x, max_y);
//     for (p1, p2) in points[..points.len() - 1].iter().zip(points[1..].iter()) {
//         println!("{:?}->{:?}", p1, p2);
//         draw_line(&mut grid, p1, p2);
//     }
//     println!("{:?}->{:?}", points[points.len() - 1], points[0]);
//     draw_line(&mut grid, &points[points.len() - 1], &points[0]);
//     grid
// }

fn point_in_poly(&[x, y]: &[isize; 2], points: &Vec<[isize; 2]>) -> bool {
    let mut c = false;
    for i in 0..points.len() + 1 {
        let [ax, ay] = points[(i + 1) % points.len()];
        let [bx, by] = points[i % points.len()];
        if x == ax && y == ay {
            return true;
        }
        if (ay > y) != (by > y) {
            let slope = ((x - ax) * (by - ay)) as isize - ((bx - ax) * (y - ay)) as isize;
            if slope == 0 {
                return true;
            } else if (slope < 0) != (by < ay) {
                c = !c;
            }
        }
    }
    c
}

// fn point_on_segment(p: &[isize; 2], q: &[isize; 2], r: &[isize; 2]) -> bool {
//     q[0] <= p[0].max(r[0])
//         && q[0] >= p[0].min(r[0])
//         && q[1] <= p[1].max(r[1])
//         && q[1] >= p[1].min(r[1])
// }

fn orientation(p: &[isize; 2], q: &[isize; 2], r: &[isize; 2]) -> i8 {
    let o = (p[0] - q[0]) * (r[1] - q[1]) - (p[1] - q[1]) * (r[0] - q[0]);
    if o < 0 {
        -1
    } else if o == 0 {
        0
    } else {
        1
    }
}

fn segment_segment_intersection(
    p1: &[isize; 2],
    p2: &[isize; 2],
    q1: &[isize; 2],
    q2: &[isize; 2],
) -> bool {
    if orientation(p1, q1, q2) == 0 && orientation(p2, q1, q2) == 0 {
        return false;
    }
    let o1 = orientation(p2, p1, q1);
    let o2 = orientation(p2, p1, q2);
    let o3 = orientation(q2, q1, p1);
    let o4 = orientation(q2, q1, p2);
    // println!("{o1}, {o2}, {o3}, {o4}");
    if o1 != o2 && o3 != o4 {
        if p1 == q1 || p1 == q2 || p2 == q1 || p2 == q2 {
            return false;
        }
        if orientation(q1, p1, q2) == 0
            || orientation(q1, p2, q2) == 0
            || orientation(p1, q1, p2) == 0
            || orientation(p1, q2, p2) == 0
        {
            return false;
        }
        true
    } else {
        false
    }
}

fn rectangle_clip_query(
    &[x1, y1]: &[isize; 2],
    &[x2, y2]: &[isize; 2],
    points: &Vec<[isize; 2]>,
) -> bool {
    let corners = [[x1, y1], [x2, y1], [x2, y2], [x1, y2]];
    for c in corners.iter() {
        if !point_in_poly(c, points) {
            return false;
        }
    }
    // println!("Checking rect: [{x1}, {y1}]x[{x2}, {y2}]");
    for i in 0..corners.len() + 1 {
        let p1 = corners[i % corners.len()];
        let p2 = corners[(i + 1) % corners.len()];

        for j in 0..points.len() + 1 {
            let q1 = points[j % points.len()];
            let q2 = points[(j + 1) % points.len()];
            let inter = segment_segment_intersection(&p1, &p2, &q1, &q2);
            // println!("{p1:?}->{p2:?} intersects {q1:?}->{q2:?}? {inter}");

            if inter {
                return false;
            }
        }
    }
    return true;
}

fn max_rectangle_constrained(points: &Vec<[isize; 2]>) -> usize {
    points
        .iter()
        .combinations(2)
        .filter(|v| rectangle_clip_query(v[0], v[1], points))
        .map(|pair| (pair[0][0].abs_diff(pair[1][0]) + 1) * (pair[0][1].abs_diff(pair[1][1]) + 1))
        .max()
        .unwrap()
}

fn main() {
    let filename = String::from("input.txt");
    let points = read_input(&filename).unwrap();
    let part1 = max_rectangle(&points);
    println!("Part 1: {part1}");
    // let grid = build_grid(&points);
    // println!("{}", grid);
    let part2 = max_rectangle_constrained(&points);
    println!("Part 2: {part2}");
}
