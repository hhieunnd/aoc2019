use std::{fs::File, io::Read};

fn main() {
    let file = File::open("input.txt");

    let mut input = String::new();

    let _ = file.unwrap().read_to_string(&mut input);

    let lines = input.lines();

    // part 1
    let mut sum1 = 0;

    // part 2
    let mut sum2 = 0;

    for line in lines {
        let num = line.parse::<i32>().unwrap();

        sum1 += num / 3 - 2;

        sum2 += sum_to_zero(num);
    }

    println!("result 1: {sum1}");
    println!("result 2: {sum2}");
}

fn sum_to_zero(mut number: i32) -> i32 {
    let mut result = 0;

    while number > 0 {
        number = number / 3 - 2;
        if number < 0 {
            number = 0;
        }

        result += number
    }

    result
}
