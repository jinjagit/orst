use super::Sorter;

// Insertion sort has generally poor performance: O(n^2).
// Possible good choice for ordering linked lists.
pub struct InsertionSort{
    binary: bool
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

            let binary = true;

            if !self.binary {
                let mut i = unsorted;

                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i -1, i);
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
fn it_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    InsertionSort { binary: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}