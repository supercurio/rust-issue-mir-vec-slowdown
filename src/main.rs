extern crate time;

use std::env;
use time::precise_time_ns;

/// Audio sample rate for the test set, used for realtime speed
/// calculation
const SAMPLE_RATE: f64 = 48000.0;
/// Total length of samples the filter benchmarks are ran on
const SAMPLE_COUNT: u64 = 524288;
/// Select how many IIR filters should be applied consecutively
/// on each buffer during the benchmark
const FILTER_COUNT: usize = 100;

const BUFFER_LEN: usize = 128;

#[allow(dead_code)]
enum VecTest {
    NoTest,
    Resize,
    ExtendFromSlice,
}

#[cfg(not(any(feature = "resize",
              feature = "extend_from_slice")))]
const RUN_VEC_TEST: VecTest = VecTest::NoTest;

#[cfg(feature = "resize")]
const RUN_VEC_TEST: VecTest = VecTest::Resize;
#[cfg(feature = "extend_from_slice")]
const RUN_VEC_TEST: VecTest = VecTest::ExtendFromSlice;

/// 2nd order biquad filter
#[derive(Copy)]
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,

    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl Clone for Biquad {
    fn clone(&self) -> Biquad {
        *self
    }
}

impl Biquad {
    fn new() -> Biquad {
        Biquad {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

/// Displays the benchmark timing results and a real-time performance estimate
fn print_elapsed(msg: &str, start: u64, filter_count: usize) {
    let elapsed = precise_time_ns() - start;
    let duration = elapsed as f64 / filter_count as f64 / SAMPLE_COUNT as f64;
    let realtime = 1.0 / duration / SAMPLE_RATE * 1e+9;
    println!("\t{:<10}{:.3} ns\t{:.0}x realtime", msg, duration, realtime);
}

fn iir(buf: &mut [f64], bq: &mut Biquad) {
    for i in 0..buf.len() {
        let x = buf[i];
        buf[i] = (bq.b0 * x) + (bq.b1 * bq.x1) + (bq.b2 * bq.x2) - (bq.a1 * bq.y1) -
                 (bq.a2 * bq.y2);

        bq.x2 = bq.x1;
        bq.x1 = x;

        bq.y2 = bq.y1;
        bq.y1 = buf[i];
    }
}

fn test_vec_resize() {
    println!("Create an empty vector, resized then discarded");
    let mut vec_test: Vec<f64> = Vec::new();
    vec_test.resize(1234, 0.0);
}

fn test_vec_extend_from_slice() {
    println!("Create an empty vector, extended then discarded");
    let mut vec_test: Vec<f64> = Vec::new();
    vec_test.extend_from_slice(&[2.0, 3.0, 4.0]);
}

fn main() {
    println!("Rust MIR Vector slow-down issue demo");

    let mut buffer_len = BUFFER_LEN;
    if let Some(arg1) = env::args().nth(1) {
        buffer_len = arg1.parse::<usize>().unwrap();
        println!("Overriding buffer_len_immut to {}", {
            buffer_len
        });
    }

    let buffer_count = SAMPLE_COUNT / buffer_len as u64;

    for _ in 0..10 {
        let mut buf = vec![0.0; buffer_len];
        let mut biquads = [Biquad::new(); FILTER_COUNT];
        let start = precise_time_ns();
        for _ in 0..buffer_count {
            for f in 0..FILTER_COUNT {
                iir(buf.as_mut_slice(), &mut biquads[f]);
            }
        }
        print_elapsed("iir", start, FILTER_COUNT);
    }

    match RUN_VEC_TEST {
        VecTest::NoTest => println!("No vector resize or extend_from_slice call compiled in"),
        VecTest::Resize => test_vec_resize(),
        VecTest::ExtendFromSlice => test_vec_extend_from_slice(),
    }

}

