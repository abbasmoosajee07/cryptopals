use std::time::Instant;
use cryptopals::{run_python};

fn main() {
    let run_sets: Vec<i32> = vec![1, 2];
    // let run_sets: Vec<i32> = vec![2];
    let start: Instant = Instant::now();

    for set in run_sets {
        let script: String = format!("../../set_{:02}/run_set_{:02}.py", set, set);
        run_python(&script);
    }

    let elapsed: std::time::Duration = start.elapsed();
    println!("Total time: {:.2?}", elapsed);
}
