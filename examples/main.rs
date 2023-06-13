extern crate rust_micro_benchmark;

use rust_micro_benchmark::benchmark;

#[benchmark(|duration| {
    println!("invoke_test_fn1 Benchmark completed in {} nanoseconds", duration);
})]
fn invoke_test_fn1() {
    // println!("invoke_test_fn1");
    let mut _sum: u128 = 0;
    for i in 0..1_000 {
        _sum += i as u128;
    }
}

#[benchmark(|duration| {
    println!("invoke_test_fn2 Benchmark completed in {} nanoseconds", duration);
})]
fn invoke_test_fn2() {
    // println!("invoke_test_fn2");
    let mut _sum: u128 = 0;
    for i in 0..10_000 {
        _sum += i as u128;
    }
}

#[benchmark(|duration| {
    println!("invoke_test_fn3 Benchmark completed in {} nanoseconds", duration);
})]
fn invoke_test_fn3() {
    // println!("invoke_test_fn3");
    let mut _sum: u128 = 0;
    for i in 0..100_000 {
        _sum += i as u128;
    }
}

fn main() {
    invoke_test_fn1();
    invoke_test_fn2();
    invoke_test_fn3();
}