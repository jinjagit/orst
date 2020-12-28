use super::Sorter;

// Insertion sort average time complexity: O(n^2).
// Possible good choice for ordering linked lists.

// Note: Insertion sort has 2 advantages, speed on short lists and speed on almost-sorted lists.
// Binary search ruins both of those, due to being quadratic and less cahce friendly.
// Included just to illustrate some of binary search functionality in Rust.

pub struct InsertionSort {
    binary: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in location in slice[..=unsorted]

            if !self.binary {
                let mut i = unsorted;

                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // Use binary search to find index,
                // then use ..rotate_right to splice in i
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // [a, c, e].binary_search(c) => Ok(1)
                    Ok(i) => i,
                    // [a, c, e].binary.search(b) => Err(1) // returns index where element 'should' go.
                    Err(i) => i,
                };

                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn insertion_binary_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    InsertionSort { binary: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn insertion_non_binary_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    InsertionSort { binary: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
