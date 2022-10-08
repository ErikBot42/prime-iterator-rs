
pub fn gen_primes() {
    //let mut num_iter: Box<dyn Iterator<Item = usize>> = Box::new((2..).into_iter());
    //for _ in 0..10 {
    //    let nxt = num_iter.next().unwrap();
    //    num_iter = Box::new(num_iter.filter(move |x| x % nxt != 0));
    //    println!("{}", nxt);
    //}

    //for prime in PrimeIterator::new() {
    for prime in make_prime_iter().take(20) {
        println!("{}", prime);
    }

    // iter,
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

use itertools::iterate;
use std::cell::RefCell;
fn make_prime_iter() -> Box<dyn Iterator<Item = usize>> {
    let init: RefCell<Option<Box<dyn Iterator<Item = usize>>>> = RefCell::new(Some(Box::new(2..)));
    Box::new(
        iterate((0, init), |(_, old)| {
            let mut new = old.take().unwrap();
            let prime = new.next().unwrap();
            (
                prime,
                RefCell::new(Some(Box::new(new.filter(move |x| x % prime != 0)))),
            )
        })
        .skip(1)
        .map(|(nxt, _)| nxt),
    )
}
//impl Iterator for Fibonacci {
//    // We can refer to this type using Self::Item
//    type Item = u32;
//
//    // Here, we define the sequence using `.curr` and `.next`.
//    // The return type is `Option<T>`:
//    //     * When the `Iterator` is finished, `None` is returned.
//    //     * Otherwise, the next value is wrapped in `Some` and returned.
//    // We use Self::Item in the return type, so we can change
//    // the type without having to update the function signatures.
//    fn next(&mut self) -> Option<Self::Item> {
//        let current = self.curr;
//
//        self.curr = self.next;
//        self.next = current + self.next;
//
//        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
//        // will never return `None`, and `Some` is always returned.
//        Some(current)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        gen_primes();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
