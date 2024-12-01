fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut list1: Vec<i32> = (0..numbers.len())
        .step_by(2)
        .map(|i| numbers[i])
        .collect();

    let mut list2: Vec<i32> = (1..numbers.len())
        .step_by(2)
        .map(|i| numbers[i])
        .collect();

    list1.sort();
    list2.sort();

    let result: i32 = (0..list1.len())
        .map(|i| (list1[i] - list2[i]).abs())
        .sum();

    println!("{}", result);
}
