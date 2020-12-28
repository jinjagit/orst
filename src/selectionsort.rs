use super::Sorter;

// Selection sort has average time complexity: O(n^2).
// It's generally worse than insertion sort.
// Works with original data entirely in place.

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 0..slice.len() {
            // find smallest value in slice[unsorted..]
            // move to end of slice[sorted..]
            let mut smallest_in_not_sorted = unsorted;

            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_not_sorted] {
                    smallest_in_not_sorted = i;
                }
            }

            if unsorted != smallest_in_not_sorted {
                slice.swap(unsorted, smallest_in_not_sorted);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
