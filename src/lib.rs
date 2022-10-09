pub fn print_primes() {
    for prime in SieveIterator::new().skip(1_000_000).take(20) {
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

struct SieveIterator {
    primes: Vec<usize>,
    flags: Vec<bool>,
    index: usize,
}
impl SieveIterator {
    fn new() -> Self {
        SieveIterator {
            primes: Vec::new(),
            flags: vec![false],
            index: 0_usize.wrapping_sub(1),
        }
    }
    fn double_size(primes: &mut Vec<usize>, flag: &mut Vec<bool>) {
        let old_len = flag.len();
        flag.extend((0..old_len).map(|_| true));
        let max_size = flag.len();
        let flag_slice = &mut flag[0..max_size];
        primes.iter_mut().for_each(|prime| {
            [false]
                .iter_mut()
                .chain(&mut *flag_slice)
                .skip((old_len / *prime) * *prime)
                .step_by(*prime)
                .for_each(|b| *b = false);
        });
        for i in old_len..max_size {
            if flag[i] {
                flag[i] = false;
                let prime = i + 1;
                primes.push(prime);
                for j in (2 * prime - 1..max_size).into_iter().step_by(prime) {
                    flag[j] = false;
                }
            }
        }
    }
}
impl Iterator for SieveIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index.wrapping_add(1);
        Some(match self.primes.get(self.index).copied() {
            Some(p) => p,
            None => {
                Self::double_size(&mut self.primes, &mut self.flags);
                self.primes.get(self.index).copied()?
            }
        })
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
