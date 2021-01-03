// Benchmark of n of comparisons performed, as approximation of computational complexity.
// Somewhat innacurate, as does not include n swaps performed.

use ::std::cell::Cell;
use ::std::rc::Rc;
use rand::prelude::*;
use orst::*;
use std::cmp::Ordering;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);

        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(), cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
        values.shuffle(&mut rand);

        let took = bench(BubbleSort, &values, &counter);
        println!("{} {} {}", "bubble", n, took);
        let took = bench(InsertionSort { binary: true }, &values, &counter);
        println!("{} {} {}", "insertion-binary", n, took);
        let took = bench(InsertionSort { binary: false }, &values, &counter);
        println!("{} {} {}", "insertion-non-binary", n, took);
        let took = bench(SelectionSort, &values, &counter);
        println!("{} {} {}", "selection", n, took);
        let took = bench(QuickSort, &values, &counter);
        println!("{} {} {}", "quick", n, took);
        }
    }

}

fn bench<T: Ord + Clone, S: Sorter>(sorter: S, values: &[SortEvaluator<T>], counter: &Cell<usize>) -> usize {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    let count = counter.get();

    for i in 1..values.len() {
        assert!(values[i] >= values[i -1]);
    }

    count
}
