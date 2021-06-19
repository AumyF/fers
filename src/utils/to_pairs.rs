pub struct ToPair<I: Iterator> {
    iter: I,
}

impl<I: Iterator> ToPair<I> {
    fn new(iter: I) -> ToPair<I> {
        ToPair { iter }
    }
}

impl<I: Iterator> Iterator for ToPair<I> {
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.iter.next()?, self.iter.next()?))
    }
}

pub fn to_pairs<I: Iterator>(i: I) -> ToPair<I> {
    ToPair::new(i)
}

pub trait ToPairBlanket: Iterator {
    ///
    ///```
    ///assert_eq!(vec![1,2,3,4].to_pairs().collect(), vec![(1,2), (3,4)])
    ///```
    ///
    fn to_pairs(&mut self) -> ToPair<&mut Self> {
        ToPair::new(self)
    }
}

mod test {
    use super::ToPairBlanket;
    #[test]
    fn t() {
        assert_eq!(
            vec![1, 2, 3, 4]
                .into_iter()
                .to_pairs()
                .collect::<Vec<(i32, i32)>>(),
            vec![(1, 2), (3, 4)]
        )
    }
}

impl<I> ToPairBlanket for I where I: Iterator {}
