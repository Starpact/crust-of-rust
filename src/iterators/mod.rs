pub trait IteratorExt: Iterator + Sized {
    fn my_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn my_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        Flatten::new(self)
    }
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner_forward: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_backward: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner_forward: None,
            inner_backward: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner_forward {
                if let item @ Some(_) = inner.next() {
                    return item;
                }
            }
            if let Some(outer) = self.outer.next() {
                self.inner_forward = Some(outer.into_iter());
            } else {
                return self.inner_backward.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner_backward {
                if let item @ Some(_) = inner.next_back() {
                    return item;
                }
            }
            if let Some(outer) = self.outer.next_back() {
                self.inner_backward = Some(outer.into_iter());
            } else {
                return self.inner_forward.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_outer() {
        assert_eq!(std::iter::empty::<Vec<()>>().my_flatten().count(), 0);
    }

    #[test]
    fn empty_inner() {
        assert_eq!(
            vec![Vec::<()>::new(), vec![], vec![]]
                .into_iter()
                .my_flatten()
                .count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(std::iter::once(vec![()]).my_flatten().count(), 1);
    }

    #[test]
    fn two_inner() {
        assert_eq!(std::iter::once(vec![(), ()]).my_flatten().count(), 2);
    }

    #[test]
    fn two_outer() {
        assert_eq!(vec![vec![()], vec![()]].into_iter().my_flatten().count(), 2);
    }

    #[test]
    fn reverse_one() {
        assert_eq!(
            std::iter::once(vec![1, 2, 3])
                .my_flatten()
                .rev()
                .collect::<Vec<_>>(),
            vec![3, 2, 1],
        )
    }

    #[test]
    fn reverse_two() {
        assert_eq!(
            vec![vec![0, 1], vec![2, 3]]
                .into_iter()
                .my_flatten()
                .rev()
                .collect::<Vec<_>>(),
            vec![3, 2, 1, 0],
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = vec![vec!["a1", "a2"], vec!["b1", "b2"]]
            .into_iter()
            .my_flatten();
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
