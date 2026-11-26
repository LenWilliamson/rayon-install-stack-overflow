use std::thread::available_parallelism;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn stack_overflow_sum() -> u64 {
    let pool = rayon::ThreadPoolBuilder::default().build().unwrap();
    let add = |a, b| {
        let c = a + b;
        pool.install(|| {});
        c
    };
    (0..10_000).into_par_iter().reduce(|| 0, add)
}

fn no_stack_overflow_sum() -> u64 {
    let (tx, rx) = crossbeam::channel::bounded(100);
    let num_adder = available_parallelism().unwrap().get();
    let mut handles = Vec::new();
    for i in 0..num_adder {
        let pool = rayon::ThreadPoolBuilder::default().build().unwrap();
        let rx_i = rx.clone();
        let handle = std::thread::Builder::new()
            .name(format!("adder-{i}"))
            .spawn(move || {
                let mut res = 0;
                while let Ok(val) = rx_i.recv() {
                    res += val;
                    pool.install(|| {});
                }
                res
            })
            .unwrap();
        handles.push(handle);
    }
    drop(rx);
    (0..10_000)
        .into_par_iter()
        .for_each(|val| tx.send(val).unwrap());
    drop(tx);
    let mut res = 0;
    for handle in handles {
        res += handle.join().unwrap();
    }
    res
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .stack_size(1024)
        .build_global()
        .unwrap();
    let sum = no_stack_overflow_sum();
    println!("sum = {sum}");
    let _ = stack_overflow_sum();
}
