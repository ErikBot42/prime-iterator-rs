pub fn print_primes() {
    for prime in SieveIterator::new().skip(10_000_000).take(20) {
        println!("{}", prime);
    }
}

struct PrimeIterator {
    inner: Option<Box<dyn Iterator<Item = usize>>>,
}
impl PrimeIterator {
    fn new() -> Self {
        PrimeIterator {
            inner: Some(Box::new(2..)),
        }
    }
}
impl Iterator for PrimeIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut old_iter = self.inner.take()?;
        let nxt = old_iter.next()?;
        self.inner = Some(Box::new(old_iter.filter(move |x| x % nxt != 0)));
        Some(nxt)
    }
}

use std::cell::RefCell;
fn make_prime_iter() -> impl Iterator<Item = usize> {
    itertools::iterate(
        (
            0,
            RefCell::new(Some(Box::new(2_usize..) as Box<dyn Iterator<Item = usize>>)),
        ),
        |(_, prev)| {
            let mut new = prev.take().unwrap();
            let prime = new.next().unwrap();
            (
                prime,
                RefCell::new(Some(Box::new(new.filter(move |x| x % prime != 0)))),
            )
        },
    )
    .skip(1)
    .map(|(n, _)| n)
}
use std::rc::Rc;

fn make_prime_iter_vec() -> impl Iterator<Item = usize> {
    let v = Rc::new(RefCell::new(Vec::new()));
    itertools::iterate((2, v), |(mut i, v)| {
        v.borrow_mut().push(i);
        while {
            i += 1;
            v.borrow().iter().any(|p| i % p == 0)
        } {}
        (i, v.clone())
    })
    .map(|(i, _)| i)
}

use bitvec::prelude::*;

struct SieveIterator {
    primes: Vec<usize>,
    flags: Vec<bool>,
    index: usize,
}
impl SieveIterator {
    fn new() -> Self {
        //let vec = bitvec![usize, Msb0; 0; 1];
        SieveIterator {
            primes: Vec::new(),
            flags: vec![false],
            index: 0_usize.wrapping_sub(1),
        }
    }
}
impl Iterator for SieveIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index.wrapping_add(1);
        match self.primes.get(self.index).copied() {
            Some(p) => Some(p),
            None => {
                {
                    // increase size
                    let old_size = self.flags.len();
                    self.flags.extend((0..old_size).map(|_| true));
                    let new_size = self.flags.len();

                    // flag previous primes
                    self.primes.iter().for_each(|&prime| {
                        self.flags
                            .iter_mut()
                            .skip((old_size / prime) * prime - 1)
                            .step_by(prime)
                            .for_each(|b| *b = false);
                    });

                    // flag new primes
                    for i in old_size..new_size {
                        if self.flags[i] {
                            let prime = i + 1;
                            self.primes.push(prime);
                            self.flags
                                .iter_mut()
                                .skip(prime - 1)
                                .step_by(prime)
                                .for_each(|b| *b = false);
                        }
                    }
                };
                self.primes.get(self.index).copied()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_iterators() {
        for (((a, b), c), d) in make_prime_iter()
            .zip(make_prime_iter_vec())
            .zip(PrimeIterator::new())
            .zip(SieveIterator::new())
            .take(1000)
        {
            assert_eq!(a, b);
            assert_eq!(a, c);
            assert_eq!(a, d);
        }
    }
}
