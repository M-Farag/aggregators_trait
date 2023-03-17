pub trait Aggregators {
    fn max(&self) -> &usize;
    fn min(&self) -> &usize;
    fn new_list() -> Self;
}
