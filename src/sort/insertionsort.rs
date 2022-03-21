use super::Sort;

pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort<T: Ord>(v: &mut [T]) {
        for unsorted in 1..v.len() {
            let mut i = unsorted;
            while i > 0 && v[i] < v[i - 1] {
                v.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}
