fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    return Flatten {
        outer: iter,
        inner: None,
    };
}

struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = self.inner.as_mut() {
                if let item @ Some(_) = inner.next() {
                    return item;
                }
            }
            self.inner = Some(self.outer.next()?.into_iter());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_outer() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_inner() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![()])).count(), 1);
    }

    #[test]
    fn two_inner() {
        assert_eq!(flatten(std::iter::once(vec![(), ()])).count(), 2);
    }

    #[test]
    fn two_outer() {
        assert_eq!(flatten(vec![vec![()], vec![()]].into_iter()).count(), 2);
    }
}
