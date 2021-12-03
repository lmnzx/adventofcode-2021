use std::fs::read_to_string;

fn main() {
    //* Part 1
    let inputs = read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<String>>();

    let mut data: Vec<Vec<i8>> = Vec::new();

    for input in inputs {
        let res = input
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        data.push(res);
    }

    // println!("{:?}", data);

    let mut zeros;
    let mut ones;
    let mut gamma: Vec<i8> = Vec::new();
    let mut epsilon: Vec<i8> = Vec::new();

    for i in 0..data[0].len() {
        ones = 0;
        zeros = 0;
        for j in 0..data.len() {
            if data[j][i] == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if ones > zeros {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }

    let gamma_string: String = gamma.iter().map(ToString::to_string).collect();
    let epsilon_string: String = epsilon.iter().map(ToString::to_string).collect();

    let answer = isize::from_str_radix(&gamma_string, 2).unwrap()
        * isize::from_str_radix(&epsilon_string, 2).unwrap();

    println!("{}", answer);
}
