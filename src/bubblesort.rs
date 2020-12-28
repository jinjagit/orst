use super::Sorter;

// Bubble sort has average time complexity: O(n^2). Don't use it!

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    // &self is unused for this struct + impl.
    where
        T: Ord,
    {
        let mut swapped = true;

        while swapped {
            swapped = false;

            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 3, 5, 2, 1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
