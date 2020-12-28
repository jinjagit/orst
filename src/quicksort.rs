use super::Sorter;

// Quick sort has worst time complexity: O(n^2), but
// average time complexity: O(n log n).

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            // Already on correct side.
            left += 1;
        } else if &rest[right] > pivot {
            // Right already on the correct side.
            // Avoids unnecessary swaps back and forth.
            right -= 1;
        } else {
            // Left holds a right, and right holds a left; swap them.
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // Re-align left to account for the pivot at 0.
    let left = left + 1;

    // Place pivot in its final position.
    slice.swap(0, left - 1);

    // Split_at_mut(mid: usize) -> (&mut [..mid, &mut[mid..]])
    let (left, right) = slice.split_at_mut(left - 1);

    assert!(left.last() <= right.first());

    // These recursions continue the ordering process, by passing on
    // each of the two parts of the list for further processing.
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice)
    }
}

#[test]
fn selection_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
