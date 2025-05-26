use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_pair = (data[0], data[1]);
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }

    Some(min_pair)
}

pub fn print_vector_with_min_pair(vec: &[i32], min_pair: (i32, i32)) {
    println!("Vector: {:?}", vec);
    println!(
        "Minimal adjacent pair with sum = {}: ({}, {})",
        min_pair.0 + min_pair.1,
        min_pair.0,
        min_pair.1
    );
}

pub fn run() {
    let vec = gen_random_vector(20);
    if let Some(pair) = min_adjacent_sum(&vec) {
        print_vector_with_min_pair(&vec, pair);
    } else {
        println!("Vector too small for adjacent sum.");
    }
}