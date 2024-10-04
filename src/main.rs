extern crate cryptonight_c as c;
extern crate cryptonight_rust as rust;
use std::time::{Duration, Instant};

use rand::Rng;

const PRINT_INTERVAL: usize = if cfg!(debug_assertions) { 1 } else { 10 };
const HASHES_PER_PRINT: usize = PRINT_INTERVAL * 4;

fn gen_input(rng: &mut impl Rng) -> (Vec<u8>, u64) {
    let length = rng.gen_range(43..=1024);
    let mut input = vec![0_u8; length];
    rng.fill(&mut input[..]);
    let height = rng.gen();
    (input, height)
}

fn main() {
    if cfg!(debug_assertions) {
        println!("WARNING: The rust code more than 50x slower in debug mode.");
    }

    let mut rng = rand::thread_rng();
    let mut r_time = Duration::ZERO;
    let mut c_time = Duration::ZERO;

    for n in 1..usize::MAX {
        let (input, height) = gen_input(&mut rng);

        let start = Instant::now();
        let r_v0_hash = rust::cryptonight_hash_v0(&input);
        let r_v1_hash = rust::cryptonight_hash_v1(&input).unwrap();
        let r_v2_hash = rust::cryptonight_hash_v2(&input);
        let r_vr_hash = rust::cryptonight_hash_r(&input, height);
        r_time += start.elapsed();

        let start = Instant::now();
        let c_v0_hash = c::cryptonight_hash_v0(&input);
        let c_v1_hash = c::cryptonight_hash_v1(&input).unwrap();
        let c_v2_hash = c::cryptonight_hash_v2(&input);
        let c_vr_hash = c::cryptonight_hash_r(&input, height);
        c_time += start.elapsed();

        assert_eq!(c_v0_hash, r_v0_hash, "IN: {input:x?}");
        assert_eq!(c_v1_hash, r_v1_hash, "IN: {input:x?}");
        assert_eq!(c_v2_hash, r_v2_hash, "IN: {input:x?}");
        assert_eq!(c_vr_hash, r_vr_hash, "IN: {input:x?}, HT: {height}");

        #[allow(clippy::modulo_one)]
        if n % PRINT_INTERVAL == 0 {
            let ratio = r_time.as_secs_f64() / c_time.as_secs_f64();
            println!("N: {n}, rust: {r_time:.3?}, c: {c_time:.3?} (ratio {ratio:.2?}) [over {HASHES_PER_PRINT} hashes]");
            r_time = Duration::ZERO;
            c_time = Duration::ZERO;
        }
    }
}
