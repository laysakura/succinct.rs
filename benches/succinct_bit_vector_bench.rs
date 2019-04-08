#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Duration;

fn c() -> Criterion {
    Criterion::default()
        .sample_size(10) // must be >= 10 for Criterion v0.3
        .warm_up_time(Duration::from_secs(1))
        .with_plots()
}

fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

mod bit_vector {
    use criterion::{BatchSize, Criterion};
    use succinct_rs::{BitString, BitVectorBuilder};

    const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

    pub fn builder_from_length_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!(
                "[{}] BitVectorBuilder::from_length(N).build()",
                super::git_hash()
            ),
            |b, &&n| b.iter(|| BitVectorBuilder::from_length(n).build()),
            &NS,
        );
    }

    pub fn builder_from_bit_string_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!(
                "[{}] BitVectorBuilder::from_bit_string(\"00...(repeated N-times)\").build()",
                super::git_hash()
            ),
            |b, &&n| {
                b.iter_batched(
                    || {
                        let s = String::from_utf8(vec!['0' as u8; n as usize]).unwrap();
                        BitString::new(&s)
                    },
                    |bs| BitVectorBuilder::from_bit_string(bs).build(),
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn rank_benchmark(_: &mut Criterion) {
        let times = 1_000_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] BitVector::rank(N) {} times", super::git_hash(), times),
            move |b, &&n| {
                b.iter_batched(
                    || BitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // rank() takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.rank(n - 1), 0);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn select_benchmark(_: &mut Criterion) {
        let times = 1_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] BitVector::select(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let mut builder = BitVectorBuilder::from_length(n);
                        for i in 0..n {
                            builder.set_bit(i);
                        }
                        builder.build()
                    },
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // rank() takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.select(n - 1), Some(n - 2));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn rank0_benchmark(_: &mut Criterion) {
        let times = 1_000_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] BitVector::rank0(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || BitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // rank() takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.rank0(n - 1), n);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn select0_benchmark(_: &mut Criterion) {
        let times = 1_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] BitVector::select(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || BitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // rank() takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.select0(n - 1), Some(n - 2));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }
}

criterion_group!(
    benches,
    bit_vector::builder_from_length_benchmark,
    bit_vector::builder_from_bit_string_benchmark,
    bit_vector::rank_benchmark,
    bit_vector::select_benchmark,
    bit_vector::rank0_benchmark,
    bit_vector::select0_benchmark,
);
criterion_main!(benches);
