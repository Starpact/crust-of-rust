use super::Sort;

pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort<T: Ord>(v: &mut [T]) {
        for i in 0..v.len() {
            let mut min = i;
            for j in i + 1..v.len() {
                if v[j] < v[min] {
                    min = j;
                }
                v.swap(i, min);
            }
        }
    }
}
