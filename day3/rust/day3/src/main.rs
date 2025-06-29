use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn build_path(data: &str) -> (Vec<Position>, HashMap<Position, usize>) {
    let mut path = Vec::new();
    let mut steps_map = HashMap::new();
    let mut pos = Position { x: 0, y: 0 };
    path.push(pos);

    let mut step_count = 0;

    for item in data.split(',') {
        let (dir, num) = item.split_at(1);
        let step = num.parse::<i32>().unwrap();

        for _ in 0..step {
            match dir {
                "U" => pos.y += 1,
                "D" => pos.y -= 1,
                "L" => pos.x -= 1,
                "R" => pos.x += 1,
                _ => panic!("Invalid direction"),
            }

            step_count += 1;
            path.push(pos);
            steps_map.entry(pos).or_insert(step_count);
        }
    }

    (path, steps_map)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let data: Vec<&str> = input.trim().split('\n').collect();

    let (wave1, steps1) = build_path(data[0]);
    let (wave2, steps2) = build_path(data[1]);

    let set1: HashSet<Position> = wave1.into_iter().collect();
    let intersection: Vec<_> = wave2
        .into_iter()
        .filter(|p| set1.contains(p) && !(p.x == 0 && p.y == 0))
        .collect();

    if let Some(pos) = intersection.iter().min_by_key(|p| p.x.abs() + p.y.abs()) {
        println!(
            "Min sum x+y: {}, position: ({}, {})",
            pos.x.abs() + pos.y.abs(),
            pos.x,
            pos.y
        );
    }

    let (point, min_sum) = intersection
        .iter()
        .filter(|p| steps1.contains_key(p) && steps2.contains_key(p))
        .map(|p| (*p, steps1[p] + steps2[p]))
        .min_by_key(|&(_, sum)| sum)
        .unwrap();

    println!("Point ({}, {}) min: {}", point.x, point.y, min_sum);
}
