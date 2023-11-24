// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The multinomial distribution.

use crate::{Binomial, Distribution};
use rand::Rng;


/// Error type returned from `Multinomial::new`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// There is a negative weight or Nan
    ProbabilityNegative,
    /// All weights are zero
    ProbabilityZero,
    /// One of the weights is inf or the sum overflows
    ProbabilityInfinity,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::ProbabilityNegative => "One of the weights is negative or Nan",
            Error::ProbabilityZero => "All of the weights are zero",
            Error::ProbabilityInfinity => "One of the weights is inf or the sum overflows",
        })
    }
}

/// Multinomial Distribution with compile time known number of categories.
#[derive(Debug, Clone, PartialEq)]
pub struct MultinomialConst<const K: usize> {
    /// Number of draws
    n: u64,
    /// normalized weights for the multinomial distribution
    /// Garantied to be not negative and they should add to a value close to 1.0
    weights: [f64; K],
}

fn normalize<const K: usize>(weights: &mut [f64; K]) -> Result<(), Error> {
    if weights.iter().any(|&x| x < 0.0) {
        return Err(Error::ProbabilityNegative);
    }

    let sum: f64 = weights.iter().sum();

    if sum == 0.0 {
        return Err(Error::ProbabilityZero);
    }

    if sum == f64::INFINITY {
        return Err(Error::ProbabilityInfinity);
    }

    weights.iter_mut().for_each(|x| *x /= sum);

    Ok(())
}

impl<const K: usize> MultinomialConst<K> {
    /// Constructs a new `MultinomialConst` which samples `[f64; K]` samples for a compile time constant `K`.
    /// 
    /// `weights` will be normalized so it sums up to 1.
    pub fn new(n: u64, mut weights: [f64; K]) -> Result<Self, Error> {
        // With improvements in Rust support for const generics this can probably be solved better
        if K == 0 {
            panic!("MultinomialConst<0> is not a valid type");
        }

        normalize(&mut weights)?;

        Ok(MultinomialConst { n, weights })
    }
}

impl<const K: usize> Distribution<[u64; K]> for MultinomialConst<K> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [u64; K] {
        // This follows the binomial approach in "The computer generation of multinomial random variates" by Charles S. Davis
        // Se also the numpy soruce for random_multinomial

        // We assume K >= 1
        // We assume that self.weights are all non negative and finite
        // If the weights sum up < 1.0 the last component will get the remaining weight
        // If the weights sum up > 1.0 the components after the first i with weights[..i] > 1.0 will get zero weights

        let mut sample = [0u64; K];
        let mut remaining_p = 1.0;
        let mut remaining_n = self.n;

        for i in 0..(K - 1) {
            if remaining_p <= 0.0 {
                break;
            }

            // It's possible that weights/remaining_p can become slightly bigger than 1.0
            let binomial = Binomial::new(remaining_n, (self.weights[i] / remaining_p).min(1.0))
                .expect("We know that prob is between 0.0 and 1.0");
            sample[i] = binomial.sample(rng);
            // This cannot overflow because sample[i] is garantied to be <= remaining_n, because it's a binomial sample
            remaining_n -= sample[i];
            if remaining_n == 0 {
                break;
            }
            remaining_p -= self.weights[i];
        }

        sample[K - 1] = remaining_n;

        sample
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_multinomial() {}
}
