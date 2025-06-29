use std::fs;

#[derive(Debug, PartialEq)]
enum TypeCode {
    Add,
    Multiple,
    Return,
}

#[derive(Debug)]
struct Illustration {
    typecode: TypeCode,
    params1: Option<i32>,
    params2: Option<i32>,
    output: Option<i32>,
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").unwrap();

    let m: Vec<i32> = input
        .split(",")
        .map(|e| e.parse::<i32>().unwrap())
        .collect();

    // part 1
    println!("{}", program(m.clone(), 12, 2));

    // part 2
    for noun in 0..99 {
        for verb in 0..99 {
            let result = program(m.clone(), noun, verb);
            if result == 19690720 {
                println!("result: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn program(mut m: Vec<i32>, num1: i32, num2: i32) -> i32 {
    m[1] = num1;
    m[2] = num2;

    let mut illustrations: Vec<Illustration> = Vec::new();

    for chunk in m.chunks(4) {
        let typecode = if chunk.len() >= 1 && chunk[0] == 1 {
            TypeCode::Add
        } else if chunk[0] == 2 {
            TypeCode::Multiple
        } else {
            TypeCode::Return
        };

        let params1 = if chunk.len() >= 2 {
            Some(chunk[1])
        } else {
            None
        };

        let params2 = if chunk.len() >= 3 {
            Some(chunk[2])
        } else {
            None
        };

        let output = if chunk.len() == 4 {
            Some(chunk[3])
        } else {
            None
        };

        illustrations.push(Illustration {
            typecode: typecode,
            params1: params1,
            params2: params2,
            output: output,
        });
    }

    // Excute it in input
    for illustration in illustrations {
        if illustration.typecode == TypeCode::Add {
            if let (Some(params1), Some(params2), Some(output)) = (
                illustration.params1,
                illustration.params2,
                illustration.output,
            ) {
                let p1 = params1 as usize;
                let p2 = params2 as usize;
                let out = output as usize;
                m[out] = m[p1] + m[p2];
            } else {
                break;
            }
        }

        if illustration.typecode == TypeCode::Multiple {
            if let (Some(params1), Some(params2), Some(output)) = (
                illustration.params1,
                illustration.params2,
                illustration.output,
            ) {
                let p1 = params1 as usize;
                let p2 = params2 as usize;
                let out = output as usize;
                m[out] = m[p1] * m[p2];
            } else {
                break;
            }
        }

        if illustration.typecode == TypeCode::Return {
            break;
        }
    }

    m[0]
}
