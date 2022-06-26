use std::{env::args, time::Instant};

const DEFAULT_SIZE: usize = 100_000_000;

fn main() {
    let size: usize = args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SIZE);
    let mut results: Vec<u16> = vec![0; size];
    let mut last_number = 0;
    let mut max_steps = 0;

    println!("search space size: {}", size);

    let start_time = Instant::now();

    for number in 0..size {
        let mut n = number;
        let mut steps = 0;

        while n > 1 {
            if n < number {
                steps += results[n];
                break;
            }

            steps += 1;

            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
        }

        results[number] = steps;

        if steps > max_steps {
            max_steps = steps;
            last_number = number;
        }
    }

    let time_taken = start_time.elapsed().as_secs_f64();

    println!(
        "duration: {} ({} numbers / sec)",
        time_taken,
        (size as f64) / time_taken
    );
    println!("max steps: {}", max_steps);
    println!("last number: {}", last_number);
}
