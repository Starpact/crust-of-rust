mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

trait Sort {
    fn sort<T: Ord>(v: &mut [T]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::bubblesort::Bubblesort;
    use crate::sort::insertionsort::InsertionSort;
    use crate::sort::quicksort::QuickSort;
    use crate::sort::selectionsort::SelectionSort;

    fn do_sort<T: Sort>() {
        let mut arr = vec![4, 2, 5, 3, 1];
        T::sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    struct StdSorter;
    impl Sort for StdSorter {
        fn sort<T: Ord>(v: &mut [T]) {
            v.sort();
        }
    }

    #[test]
    fn test_std() {
        do_sort::<StdSorter>();
    }

    #[test]
    fn test_bubble() {
        do_sort::<Bubblesort>();
    }

    #[test]
    fn test_insertion() {
        do_sort::<InsertionSort>();
    }

    #[test]
    fn test_selection() {
        do_sort::<SelectionSort>();
    }

    #[test]
    fn test_quick() {
        do_sort::<QuickSort>();
    }
}
