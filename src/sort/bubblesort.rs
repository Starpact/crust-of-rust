use super::Sort;

pub struct Bubblesort;

impl Sort for Bubblesort {
    fn sort<T: Ord>(v: &mut [T]) {
        loop {
            let mut swapped = false;
            for i in 0..v.len() - 1 {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}
