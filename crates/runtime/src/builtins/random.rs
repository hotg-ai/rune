use anyhow::Error;
use rand::{Rng, SeedableRng};

use crate::{Tensor, builtins::Arguments};

pub fn random(args: &Arguments) -> Result<Tensor, Error> {
    let count: usize = args.parse_or_default("amount", 1)?;

    let rng = rand::thread_rng();
    random_tensor(count, rng)
}

pub fn seeded_random(args: &Arguments, seed: u64) -> Result<Tensor, Error> {
    let count: usize = args.parse_or_default("amount", 1)?;

    let rng = rand::rngs::SmallRng::seed_from_u64(seed);
    random_tensor(count, rng)
}

fn random_tensor(count: usize, mut rng: impl Rng) -> Result<Tensor, Error> {
    let mut numbers: Vec<u32> = Vec::new();
    for _ in 0..count {
        numbers.push(rng.gen());
    }
    Ok(Tensor::new(&numbers, &[1, count]))
}
