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

/// Multinomial Distribution, which uses Binomail samples
#[derive(Debug, Clone, PartialEq)]
pub struct MultinomialConst<const K: usize> {
    /// Number of draws
    n: u64,
    /// normalized weights for the multinomial distribution
    /// Garantied to be not negative and they should add to a value close to 1.0
    weights: [f64; K],
}

impl<const K: usize> Distribution<[u64; K]> for MultinomialConst<K> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [u64; K] {
        // This follows the binomial approach in "The computer generation of multinomial random variates" by Charles S. Davis
        // Se also the numpy soruce for random_multinomial

        // We assume K >= 1
        // We assume that self.weights are all non negative and finite
        // 

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
            //This cannot overflow because sample[i] is garantied to be <= remaining_n, because it's a binomial sample
            remaining_n -= sample[i];
            if remaining_n == 0 {
                break;
            }
            remaining_p -= self.weights[i];
        }

        sample[K-1] = remaining_n;

        sample
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_multinomial() {}
}
