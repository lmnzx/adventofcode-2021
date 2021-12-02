use std::fs::read_to_string;

fn main() {
    let inputs = read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<String>>();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for input in inputs.iter() {
        let temp = input.split(" ").collect::<Vec<&str>>();
        if temp[0] == "forward" {
            horizontal += temp[1].parse::<i32>().unwrap();
            if aim != 0 {
                depth += temp[1].parse::<i32>().unwrap() * aim;
            }
        } else if temp[0] == "down" {
            aim += temp[1].parse::<i32>().unwrap();
        } else {
            aim -= temp[1].parse::<i32>().unwrap();
        }
    }

    println!("\n");
    println!("horizontal - {}", horizontal);
    println!("depth - {}", depth);
    println!("{}", horizontal * depth);
}
