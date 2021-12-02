fn main() {
    let inputs = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut count = 0;
    for input in 3..inputs.len() {
        let a = inputs[input] + inputs[input - 1] + inputs[input - 2];
        let b = inputs[input - 1] + inputs[input - 2] + inputs[input - 3];
        if a > b {
            count += 1;
        }
    }
    println!("{}", count);
}
