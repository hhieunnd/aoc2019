use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let data: Vec<&str> = input.split("-").collect();

    let min = data[0].parse::<i32>().unwrap();
    let max = data[1].parse::<i32>().unwrap();

    let mut count = 0;

    for i in min..=max {
        let ispassword = is_password(i);
        if ispassword {
            count += 1;
        }
    }

    println!("count: {count}")
}

fn is_password(num: i32) -> bool {
    let num = num.to_string();
    // 6 digit
    if num.len() != 6 {
        return false;
    }

    let num: Vec<&str> = num.split("").filter(|e| *e != "").collect();

    // Hashmap
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for (index, value) in num.iter().enumerate() {
        let counter: &mut i32 = counts.entry(value).or_insert(0);

        if index > 0 && num[index] != num[index - 1] {
            *counter = 1;
        } else {
            *counter += 1;
        }
    }

    let is_have_count = counts.values().any(|&count| count == 2);

    if !is_have_count {
        return false;
    }

    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    if (num[0].parse::<i32>().unwrap() > num[1].parse::<i32>().unwrap())
        || (num[1].parse::<i32>().unwrap() > num[2].parse::<i32>().unwrap())
        || (num[2].parse::<i32>().unwrap() > num[3].parse::<i32>().unwrap())
        || (num[3].parse::<i32>().unwrap() > num[4].parse::<i32>().unwrap())
        || (num[4].parse::<i32>().unwrap() > num[5].parse::<i32>().unwrap())
    {
        return false;
    }

    return true;
}
