fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut list_left: Vec<i32> = (0..numbers.len())
        .step_by(2)
        .map(|i| numbers[i])
        .collect();

    let mut list_right: Vec<i32> = (1..numbers.len())
        .step_by(2)
        .map(|i| numbers[i])
        .collect();

    list_left.sort();
    list_right.sort();

    let mut sim_index = 0;
    let mut num_cache = i32::MAX;
    let mut freq_cache = 0;

    for l in &list_left{
        if *l == num_cache {
            sim_index += *l * freq_cache;
            continue;
        }
        let mut freq = 0;
        for r in &list_right {
            if *l == *r {
                freq += 1;
            }
        }
        sim_index += *l * freq;
        num_cache = *l;
        freq_cache  = freq;
    }
    
    let result: i32 = (0..list_left.len())
        .map(|i| (list_left[i] - list_right[i]).abs())
        .sum();

    println!("list difference: {}", result);
    println!("sim index: {}", sim_index);
}
