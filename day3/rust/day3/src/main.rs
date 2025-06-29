use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data: Vec<&str> = input.split("\n").collect();

    let mut wave1: Vec<Position> = Vec::new();
    wave1.push(Position { x: 0, y: 0 });

    let mut wave2: Vec<Position> = Vec::new();
    wave2.push(Position { x: 0, y: 0 });

    let datawave1: Vec<&str> = data[0].split(",").collect();
    let datawave2: Vec<&str> = data[1].split(",").collect();

    for item in datawave1 {
        let (dir, num) = item.split_at(1);
        let last = wave1.last().unwrap().clone();
        let step = num.parse::<i32>().expect("Invalid step");

        if dir == "U" {
            wave1.extend(((last.y + 1)..=(last.y + step)).map(|y| Position { x: last.x, y }));
        } else if dir == "D" {
            wave1.extend(
                ((last.y - step)..=last.y - 1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|y| Position { x: last.x, y }),
            );
        } else if dir == "L" {
            wave1.extend(
                ((last.x - step)..=(last.x - 1))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|x| Position { x, y: last.y }),
            );
        } else if dir == "R" {
            wave1.extend(((last.x + 1)..=(last.x + step)).map(|x| Position { x, y: last.y }));
        }
    }

    for item in datawave2 {
        let (dir, num) = item.split_at(1);
        let last = wave2.last().unwrap().clone();
        let step = num.parse::<i32>().expect("Invalid step");

        if dir == "U" {
            wave2.extend(((last.y + 1)..=(last.y + step)).map(|y| Position { x: last.x, y }));
        } else if dir == "D" {
            wave2.extend(
                ((last.y - step)..=last.y - 1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|y| Position { x: last.x, y }),
            );
        } else if dir == "L" {
            wave2.extend(
                ((last.x - step)..=(last.x - 1))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|x| Position { x, y: last.y }),
            );
        } else if dir == "R" {
            wave2.extend(((last.x + 1)..=(last.x + step)).map(|x| Position { x, y: last.y }));
        }
    }

    let set1: HashSet<Position> = wave1.iter().cloned().collect();
    let intersection: Vec<Position> = wave2
        .iter()
        .filter(|x| set1.contains(x))
        .cloned()
        .filter(|p| !(p.x == 0 && p.y == 0))
        .collect();

    let min_pos = intersection.iter().min_by_key(|p| p.x.abs() + p.y.abs());

    // part 1
    if let Some(pos) = min_pos {
        println!(
            "Min sum x+y: {}, position: ({}, {})",
            pos.x.abs() + pos.y.abs(),
            pos.x,
            pos.y
        );
    }

    // part 2
    let mut min_sum = usize::MAX;
    let mut min_point = None;

    for point in &intersection {
        if let Some(index1) = wave1.iter().position(|p| p == point) {
            if let Some(index2) = wave2.iter().position(|p| p == point) {
                let sum = index1 + index2;
                if sum < min_sum {
                    min_sum = sum;
                    min_point = Some(point);
                }
            }
        }
    }

    if let Some(point) = min_point {
        println!("Point ({}, {}) min: {}", point.x, point.y, min_sum);
    }
}
