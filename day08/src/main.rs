use disjoint_sets::UnionFind;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::prelude::*;

#[derive(Debug)]
struct ParseErrorPoint3 {}
fn from_str(s: &str) -> Result<[f64; 3], ParseErrorPoint3> {
    let coord_strs: Vec<&str> = s.split(',').collect();
    if coord_strs.len() != 3 {
        Err(ParseErrorPoint3 {})
    } else {
        let mut coords: [f64; 3] = [0.; 3];
        for i in 0..3 {
            match coord_strs[i].parse::<f64>() {
                Ok(c) => coords[i] = c,
                Err(_) => return Err(ParseErrorPoint3 {}),
            }
        }
        Ok(coords)
    }
}

fn read_input(filename: &String) -> Result<Vec<[f64; 3]>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let point_strs: Vec<&str> = buf.trim().split('\n').collect();
    Ok(point_strs.iter().map(|s| from_str(s).unwrap()).collect())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn compute_circuits(points: &Vec<[f64; 3]>, n: u32) -> HashMap<usize, HashSet<usize>> {
    // points.iter().for_each(|p| println!("{:?}", p));
    let mut uf = UnionFind::<usize>::new(points.len());
    let tree = {
        let mut t = KdTree::new(3);
        points
            .iter()
            .enumerate()
            .for_each(|(i, p)| t.add(p, i).unwrap());
        t
    };
    let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
    let min_dist = {
        let mut dist: Vec<(f64, (usize, usize))> = Vec::new();
        points.iter().enumerate().for_each(|(i, p)| {
            let nsearch: usize = 20;
            let mut matches = tree.nearest(p, nsearch, &squared_euclidean).unwrap()[1..]
                .iter()
                .map(|&(d, &j)| (d, (i, j)))
                .collect::<Vec<(f64, (usize, usize))>>();
            dist.append(&mut matches);
        });
        dist.sort_by(|(d1, _), (d2, _)| d1.total_cmp(d2));
        dist.dedup_by(|(_, (i, j)), (_, (i1, j1))| i == i1 && j == j1 || j == i1 && i == j1);
        dist
    };
    let mut count = 0;
    for (dist, (i, j)) in min_dist {
        if count == n {
            break;
        }
        uf.union(i, j);
        count += 1;
        // if uf.union(i, j) {
        //     count += 1;
        // println!(
        //     "{}: Adding ({}, {}) link ({:?}->{:?})",
        //     count, i, j, points[i], points[j]
        // );
        // println!("  Distance: {}", dist);
        // } else {
        //     println!(
        //         "Ignoring ({}, {}) link ({:?}->{:?})",
        //         i, j, points[i], points[j]
        //     );
        // }
    }

    for e in 0..points.len() {
        let index = uf.find(e);
        if !circuits.contains_key(&index) {
            circuits.insert(index, HashSet::new());
        }
        circuits.get_mut(&index).unwrap().insert(e);
    }
    circuits
}

fn compute_circuits_mst(points: &Vec<[f64; 3]>, n: usize) -> Vec<(usize, usize)> {
    // points.iter().for_each(|p| println!("{:?}", p));
    let mut uf = UnionFind::<usize>::new(points.len());
    let tree = {
        let mut t = KdTree::new(3);
        points
            .iter()
            .enumerate()
            .for_each(|(i, p)| t.add(p, i).unwrap());
        t
    };
    let mut circuits = Vec::new();
    let min_dist = {
        let mut dist: Vec<(f64, (usize, usize))> = Vec::new();
        points.iter().enumerate().for_each(|(i, p)| {
            let nsearch: usize = 20;
            let mut matches = tree.nearest(p, nsearch, &squared_euclidean).unwrap()[1..]
                .iter()
                .map(|&(d, &j)| (d, (i, j)))
                .collect::<Vec<(f64, (usize, usize))>>();
            dist.append(&mut matches);
        });
        dist.sort_by(|(d1, _), (d2, _)| d1.total_cmp(d2));
        dist.dedup_by(|(_, (i, j)), (_, (i1, j1))| i == i1 && j == j1 || j == i1 && i == j1);
        dist
    };
    let mut count = 0;
    for (_, (i, j)) in min_dist {
        if count == n {
            break;
        }
        if uf.union(i, j) {
            count += 1;
            circuits.push((i, j));
        }
    }
    {
        let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
        for e in 0..points.len() {
            let index = uf.find(e);
            if !circuits.contains_key(&index) {
                circuits.insert(index, HashSet::new());
            }
            circuits.get_mut(&index).unwrap().insert(e);
        }
        circuits.iter().for_each(|(i, set)| {
            print!("{}: [ ", i);
            set.iter().for_each(|v| print!("{} ", v));
            println!("] ({})", set.len());
        });
    }
    circuits
}

fn main() {
    let filename = String::from("input.txt");
    let points = read_input(&filename).unwrap();
    let circuits = compute_circuits(&points, 1000);
    let mut sorted_sizes: Vec<usize> = circuits.iter().map(|(_, set)| set.len()).collect();
    sorted_sizes.sort_by_key(|&k| std::cmp::Reverse(k));
    circuits.iter().for_each(|(i, set)| {
        print!("{}: [ ", i);
        set.iter().for_each(|v| print!("{} ", v));
        println!("] ({})", set.len());
    });
    let part1: u128 = sorted_sizes[..3]
        .iter()
        .fold(1u128, |acc, &x| acc * (x as u128));
    println!("Part 1: {}", part1);

    println!("Num points: {}", points.len());

    let circuits = compute_circuits_mst(&points, points.len() - 1);
    let &(i, j) = circuits.last().unwrap();
    let part2 = (points[i][0] * points[j][0]) as u128;
    println!("Part 2: {}", part2);
}
