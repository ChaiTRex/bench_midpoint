#![allow(unstable_name_collisions)]

use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[allow(unused_mut)]
pub fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! random_iter {
        ($type:ty) => {
            thread_rng().sample_iter::<$type, Uniform<$type>>(Uniform::new_inclusive(
                <$type>::MIN,
                <$type>::MAX,
            ))
        };
    }

    macro_rules! benches {
        ($($module:ident : $algorithm_name:expr , $($ty:ident)*);+) => {
            $(
                $(
                    c.bench_function(concat!($algorithm_name, " for ", stringify!($ty)), |b| {
                        use bench_midpoint::$module::Midpoint;
                        let mut randoms = random_iter!($ty);

                        b.iter(|| {
                            let x = black_box(randoms.next().unwrap());
                            let y = black_box(randoms.next().unwrap());

                            black_box($ty::midpoint_2(x, y));
                        })
                    });
                )*
            )*
        };
    }

    benches!(
        hackers_delight: "Hacker's Delight", u8 u16 u32 u64 u128;
        next_larger_type: "next larger type", u8 u16 u32 u64;
        overflow_flag: "overflow flag", u8 u16 u32 u64 u128
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
