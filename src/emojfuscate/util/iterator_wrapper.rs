pub struct IteratorWrapper<I>
where
    I: Iterator,
{
    pub iter: I,
}

impl<I, A> Iterator for IteratorWrapper<I>
where
    I: Iterator<Item = A>,
{
    type Item = A;
    fn next(&mut self) -> Option<A> {
        self.iter.next()
    }
}
