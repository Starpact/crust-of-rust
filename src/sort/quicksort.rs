use super::Sort;

pub struct QuickSort;

impl Sort for QuickSort {
    fn sort<T: Ord>(v: &mut [T]) {
        if v.len() <= 1 {
            return;
        }

        let (mut left, mut right) = (1, v.len() - 1);
        while left < right {
            if v[left] <= v[0] {
                left += 1;
            } else if v[right] >= v[0] {
                right -= 1;
            } else {
                v.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        if v[left] > v[0] {
            left -= 1;
        }

        v.swap(0, left);
        Self::sort(&mut v[..left]);
        Self::sort(&mut v[left + 1..])
    }
}
