
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

//use bitvec::prelude::*;

struct SieveIterator {
    primes: Vec<usize>,
    //flags: BitVec<usize, Msb0>,
    flags: Vec<bool>,
    index: usize,
}
impl SieveIterator {
    pub fn new() -> Self {
        SieveIterator {
            primes: Vec::new(),
            flags: vec![false],
            index: 0_usize.wrapping_sub(1),
        }
    }

    fn increase_size(&mut self) {
        // increase size
        let old_size = self.flags.len();
        self.flags.extend((0..old_size * 127).map(|_| true));
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
    }
    fn len(&self) -> usize {
        self.primes.len()
    }
}
impl Iterator for SieveIterator {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.index.wrapping_add(1);
        match self.primes.get(self.index).copied() {
            Some(p) => Some(p),
            None => {
                self.increase_size();
                self.primes.get(self.index).copied()
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n;
        while self.index > self.len() {
            self.increase_size();
        }
        //self.advance_by(n).ok()?;
        self.next()
    }
}

use std::cell::Ref;
use std::ops::Index;

/// Immutable list created from lazily evaluated iterator
impl<T> InfiniteList<T>
where
    T: Iterator<Item = usize>,
{
    fn new(iter: T) -> Self {
        Self {
            iter: RefCell::new(iter),
            values: RefCell::new(Vec::new()),
        }
    }
}
struct InfiniteList<T: Iterator<Item = usize>> {
    iter: RefCell<T>,
    values: RefCell<Vec<usize>>,
}
impl<T: Iterator<Item = usize>> Index<usize> for InfiniteList<T> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        while index >= self.values.borrow().len() {
            self.values
                .borrow_mut()
                .push(self.iter.borrow_mut().next().unwrap())
        }
        //self.values.borrow().get(index).unwrap()
        unsafe { &*self.values.borrow().as_ptr().offset(index as isize) }
    }
}

fn prime_list() -> InfiniteList<SieveIterator> {
    InfiniteList::new(SieveIterator::new())
}
#[cfg(test)]
mod tests {
    use super::*;

    fn make_iters() -> impl Iterator<Item = (((usize, usize), usize), usize)> {
        Box::new(
            make_prime_iter()
                .zip(make_prime_iter_vec())
                .zip(PrimeIterator::new())
                .zip(SieveIterator::new()),
        )
    }

    fn cmp_iters(iter: Box<dyn Iterator<Item = (((usize, usize), usize), usize)>>) {
        iter.for_each(|(((a, b), c), d)| {
            assert_eq!(a, b);
            assert_eq!(a, c);
            assert_eq!(a, d);
        })
    }

    #[test]
    fn compare_iterators() {
        cmp_iters(Box::new(make_iters().take(1000)));
    }
    #[test]
    fn compare_iterators_skip() {
        for skip in [0, 1, 2, 611, 88, 712, 109, 141, 16, 306, 813, 108, 367] {
            cmp_iters(Box::new(make_iters().skip(skip).take(50)));
        }
    }

    #[test]
    fn infinite_list() {
        for start in [0, 1, 2, 234] {
            let iter = (0..).into_iter();
            let infinite_list = InfiniteList::new(iter);
            for index in [start, 1, 2, 345, 0, 2348] {
                assert_eq!(infinite_list[index], index);
            }
        }
    }

    #[test]
    fn infinite_prime_list() {
        let prime_list = prime_list();

        assert_eq!(prime_list[3], 7);
        assert_eq!(prime_list[2], 5);
        assert_eq!(prime_list[1], 3);
        assert_eq!(prime_list[0], 2);
    }
}
